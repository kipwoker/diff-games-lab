//! Parser for the problem description.
//!
//! Grammar, directives in any order, `#` comments and blank lines between blocks:
//!
//! ```text
//! dimension <n>
//! constraints M | P | Q        # each following line: coefficients… rhs
//! matrix A | B | C | pi        # each following line: one row of coefficients
//! time <T>
//! step <Step>
//! ```
//!
//! Row counts are implied by the lines written, and every shape is checked against the sizes
//! the other blocks imply, so a mistyped row is reported with its line number rather than
//! silently describing a different problem.

use std::fmt;
use std::path::Path;

use crate::matrix::Matrix;

/// A complete problem instance.
///
/// `m_rows` is the terminal constraint matrix `M`, `a`/`b`/`c` are the dynamics and the two
/// input maps, `pi` is the terminal functional, and `p`/`q` are the control constraint sets.
#[derive(Clone, Debug, PartialEq)]
pub struct InputData {
    /// State dimension.
    pub n: usize,
    /// Number of inequalities defining `M`.
    pub m: usize,
    /// Rows of `M`, each `(n coefficients, right-hand side)`.
    pub m_rows: Vec<Vec<f64>>,
    /// Size of the square matrix `A`.
    pub dim_a: usize,
    pub a: Matrix,
    /// Column count of `B`; the row count is `dim_a`.
    pub cols_b: usize,
    pub b: Matrix,
    /// Column count of `C`; the row count is `dim_a`.
    pub cols_c: usize,
    pub c: Matrix,
    /// Row count of the terminal functional `pi`.
    pub rows_pi: usize,
    /// Terminal functional, `rows_pi × dim_a`.
    pub pi: Matrix,
    /// Rows of `P`, each `(cols_b coefficients, right-hand side)`.
    pub p: Vec<Vec<f64>>,
    /// Rows of `Q`, each `(cols_c coefficients, right-hand side)`.
    pub q: Vec<Vec<f64>>,
    /// Terminal time.
    pub t: f64,
    /// Time step, the length of one interval of the partition.
    pub step: f64,
}

impl InputData {
    /// Number of printed blocks: `floor(T / Step)`.
    ///
    /// A step that does not divide `T` exactly loses the last partial block.
    pub fn block_count(&self) -> usize {
        let blocks = (self.t / self.step).floor();
        if blocks.is_finite() && blocks > 0.0 {
            blocks as usize
        } else {
            0
        }
    }
}

/// A problem that could not be read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// A problem in the input file, at `line` (0 means "at end of input").
    Descriptive { line: usize, message: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Descriptive { line: 0, message } => write!(f, "at end of input: {message}"),
            Self::Descriptive { line, message } => write!(f, "line {line}: {message}"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parses a whole problem description.
pub fn parse(text: &str) -> Result<InputData, ParseError> {
    parse_descriptive(text)
}

/// Reads and parses a file.
pub fn parse_file(path: impl AsRef<Path>) -> Result<InputData, String> {
    let path = path.as_ref();
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    parse(&text).map_err(|error| format!("{}: {error}", path.display()))
}

/// The blocks a description can carry, in the order they normally appear.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    M,
    A,
    B,
    C,
    Pi,
    P,
    Q,
}

impl Slot {
    fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_lowercase().as_str() {
            "m" => Some(Self::M),
            "a" => Some(Self::A),
            "b" => Some(Self::B),
            "c" => Some(Self::C),
            "pi" => Some(Self::Pi),
            "p" => Some(Self::P),
            "q" => Some(Self::Q),
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::M => "M",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::Pi => "pi",
            Self::P => "P",
            Self::Q => "Q",
        }
    }
}

/// One collected matrix block and the line that started it.
struct Block {
    slot: Slot,
    rows: Vec<Vec<f64>>,
    line: usize,
}

fn descriptive(line: usize, message: String) -> ParseError {
    ParseError::Descriptive { line, message }
}

/// Parses the whole description, checking every shape against the others.
pub fn parse_descriptive(text: &str) -> Result<InputData, ParseError> {
    let mut dimension: Option<usize> = None;
    let mut terminal_time: Option<f64> = None;
    let mut step: Option<f64> = None;
    let mut open: Option<Block> = None;
    let mut blocks: Vec<Block> = Vec::new();

    for (index, raw) in text.lines().enumerate() {
        let line = index + 1;
        let code = raw.split_once('#').map_or(raw, |(code, _)| code);
        let trimmed = code.trim();

        // A blank line closes the block that is being written.
        if trimmed.is_empty() {
            close(&mut open, &mut blocks)?;
            continue;
        }

        let mut fields = trimmed.split_whitespace();
        let head = fields.next().unwrap_or_default();
        let keyword = head.to_ascii_lowercase();

        match keyword.as_str() {
            "dimension" => {
                close(&mut open, &mut blocks)?;
                let value = single(fields, line, "dimension")?;
                let count = value.parse::<usize>().map_err(|_| {
                    descriptive(
                        line,
                        format!("dimension: expected a whole number, found {value:?}"),
                    )
                })?;
                if count == 0 {
                    return Err(descriptive(
                        line,
                        String::from("dimension must be at least 1"),
                    ));
                }
                if dimension.replace(count).is_some() {
                    return Err(descriptive(
                        line,
                        String::from("dimension is given more than once"),
                    ));
                }
            }
            "constraints" | "matrix" => {
                close(&mut open, &mut blocks)?;
                let name = single(fields, line, &keyword)?;
                let slot = Slot::from_name(name).ok_or_else(|| {
                    descriptive(
                        line,
                        format!(
                            "unknown block {name:?}: expected one of M, P, Q for constraints \
                             or A, B, C, pi for matrix"
                        ),
                    )
                })?;
                let fits = if keyword == "constraints" {
                    matches!(slot, Slot::M | Slot::P | Slot::Q)
                } else {
                    matches!(slot, Slot::A | Slot::B | Slot::C | Slot::Pi)
                };
                if !fits {
                    return Err(descriptive(
                        line,
                        format!(
                            "{} belongs on a `{}` line, not `{keyword}`",
                            slot.label(),
                            if matches!(slot, Slot::M | Slot::P | Slot::Q) {
                                "constraints"
                            } else {
                                "matrix"
                            }
                        ),
                    ));
                }
                let repeated = blocks.iter().any(|block| block.slot == slot)
                    || open.as_ref().is_some_and(|block| block.slot == slot);
                if repeated {
                    return Err(descriptive(
                        line,
                        format!("{} is given more than once", slot.label()),
                    ));
                }
                open = Some(Block {
                    slot,
                    rows: Vec::new(),
                    line,
                });
            }
            "time" | "step" => {
                close(&mut open, &mut blocks)?;
                let value = single(fields, line, &keyword)?;
                let parsed = parse_real(value, line, &keyword)?;
                let target = if keyword == "time" {
                    &mut terminal_time
                } else {
                    &mut step
                };
                if target.replace(parsed).is_some() {
                    return Err(descriptive(
                        line,
                        format!("{keyword} is given more than once"),
                    ));
                }
            }
            _ => {
                let Some(block) = open.as_mut() else {
                    return Err(descriptive(
                        line,
                        format!(
                            "unexpected {head:?}: a row of numbers must follow a `matrix` or \
                             `constraints` line"
                        ),
                    ));
                };

                let mut row = Vec::new();
                for token in std::iter::once(head).chain(fields) {
                    row.push(parse_real(token, line, block.slot.label())?);
                }
                if let Some(first) = block.rows.first() {
                    if first.len() != row.len() {
                        return Err(descriptive(
                            line,
                            format!(
                                "{}: row {} has {} values, but the first row has {}",
                                block.slot.label(),
                                block.rows.len() + 1,
                                row.len(),
                                first.len()
                            ),
                        ));
                    }
                }
                block.rows.push(row);
            }
        }
    }

    close(&mut open, &mut blocks)?;
    resolve(dimension, terminal_time, step, &blocks)
}

/// Pushes the open block into the list, refusing an empty one.
fn close(open: &mut Option<Block>, blocks: &mut Vec<Block>) -> Result<(), ParseError> {
    if let Some(block) = open.take() {
        if block.rows.is_empty() {
            return Err(descriptive(
                block.line,
                format!("{} has no rows", block.slot.label()),
            ));
        }
        blocks.push(block);
    }
    Ok(())
}

/// Reads the single remaining token of a directive.
fn single<'a>(
    mut fields: impl Iterator<Item = &'a str>,
    line: usize,
    what: &str,
) -> Result<&'a str, ParseError> {
    let value = fields
        .next()
        .ok_or_else(|| descriptive(line, format!("`{what}` needs a value")))?;
    if fields.next().is_some() {
        return Err(descriptive(
            line,
            format!("`{what}` takes exactly one value"),
        ));
    }
    Ok(value)
}

fn parse_real(token: &str, line: usize, what: &str) -> Result<f64, ParseError> {
    token
        .parse::<f64>()
        .map_err(|_| descriptive(line, format!("{what}: expected a number, found {token:?}")))
}

fn find(blocks: &[Block], slot: Slot) -> Result<&Block, ParseError> {
    blocks
        .iter()
        .find(|block| block.slot == slot)
        .ok_or_else(|| descriptive(0, format!("no `{}` block", slot.label())))
}

/// Turns the collected blocks into [`InputData`], checking every shape against the others.
fn resolve(
    dimension: Option<usize>,
    terminal_time: Option<f64>,
    step: Option<f64>,
    blocks: &[Block],
) -> Result<InputData, ParseError> {
    let n = dimension.ok_or_else(|| descriptive(0, String::from("no `dimension` line")))?;
    let t = terminal_time.ok_or_else(|| descriptive(0, String::from("no `time` line")))?;
    let step = step.ok_or_else(|| descriptive(0, String::from("no `step` line")))?;

    let a = find(blocks, Slot::A)?;
    let dim_a = a.rows.len();
    for (offset, row) in a.rows.iter().enumerate() {
        if row.len() != dim_a {
            return Err(descriptive(
                a.line,
                format!(
                    "A must be square: row {} has {} values, expected {dim_a}",
                    offset + 1,
                    row.len()
                ),
            ));
        }
    }

    let b = find(blocks, Slot::B)?;
    let c = find(blocks, Slot::C)?;
    check_row_count(b, dim_a)?;
    check_row_count(c, dim_a)?;
    let cols_b = b.rows[0].len();
    let cols_c = c.rows[0].len();

    let pi = find(blocks, Slot::Pi)?;
    for (offset, row) in pi.rows.iter().enumerate() {
        if row.len() != dim_a {
            return Err(descriptive(
                pi.line,
                format!(
                    "pi: row {} has {} values, expected {dim_a} like A",
                    offset + 1,
                    row.len()
                ),
            ));
        }
    }

    let m = find(blocks, Slot::M)?;
    for (offset, row) in m.rows.iter().enumerate() {
        if row.len() != n + 1 {
            return Err(descriptive(
                m.line,
                format!(
                    "M: row {} has {} values, expected {} ({n} coefficients and a right-hand \
                     side)",
                    offset + 1,
                    row.len(),
                    n + 1
                ),
            ));
        }
    }

    let p = find(blocks, Slot::P)?;
    for (offset, row) in p.rows.iter().enumerate() {
        if row.len() != cols_b + 1 {
            return Err(descriptive(
                p.line,
                format!(
                    "P: row {} has {} values, expected {} ({} control coefficients and a \
                     right-hand side)",
                    offset + 1,
                    row.len(),
                    cols_b + 1,
                    cols_b
                ),
            ));
        }
    }

    let q = find(blocks, Slot::Q)?;
    for (offset, row) in q.rows.iter().enumerate() {
        if row.len() != cols_c + 1 {
            return Err(descriptive(
                q.line,
                format!(
                    "Q: row {} has {} values, expected {} ({} disturbance coefficients and a \
                     right-hand side)",
                    offset + 1,
                    row.len(),
                    cols_c + 1,
                    cols_c
                ),
            ));
        }
    }

    Ok(InputData {
        n,
        m: m.rows.len(),
        m_rows: m.rows.clone(),
        dim_a,
        a: Matrix::from_rows(&a.rows),
        cols_b,
        b: Matrix::from_rows(&b.rows),
        cols_c,
        c: Matrix::from_rows(&c.rows),
        rows_pi: pi.rows.len(),
        pi: Matrix::from_rows(&pi.rows),
        p: p.rows.clone(),
        q: q.rows.clone(),
        t,
        step,
    })
}

fn check_row_count(block: &Block, expected: usize) -> Result<(), ParseError> {
    if block.rows.len() != expected {
        return Err(descriptive(
            block.line,
            format!(
                "{} has {} rows, expected {expected} like A",
                block.slot.label(),
                block.rows.len()
            ),
        ));
    }
    Ok(())
}
