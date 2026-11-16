//! The `nonlinear` description: parsed, validated, and refused.
//!
//! `examples/p1/example-p1b.txt` describes a game the linear engine cannot advance:
//! `ẋ = f(x, u, v)` with turn-limited headings. Track 2's plan is to replace the interval map
//! with a certified linearisation (there is no support-function algebra for `f`), and until that
//! exists the honest behaviour is to read the file, check the fields that define the case, and
//! say so — not to reject it as a typo at the first unknown line.
//!
//! Two shapes are handled: `nonlinear <kind>` opens a block of `key value` lines, and
//! `grid <name> <low> <high> <count>` records a reference grid. `unicycle-pursuit` is the one
//! kind with a meaning here; its eight fields are required, and the case is only valid when the
//! evader turns slower than the pursuer, which is what makes its bridge non-degenerate
//! (`../kb/entries/02-p1-relative-interior.md`).

use crate::input_parser::ParseError;

/// One axis of a reference grid.
#[derive(Clone, Debug, PartialEq)]
pub struct GridAxis {
    pub name: String,
    pub low: f64,
    pub high: f64,
    pub count: usize,
}

/// A parsed `nonlinear` description.
#[derive(Clone, Debug, PartialEq)]
pub struct NonlinearProblem {
    pub kind: String,
    /// Fields in file order, names as written.
    pub fields: Vec<(String, f64)>,
    pub grid: Vec<GridAxis>,
}

/// The eight fields of `unicycle-pursuit`, checked.
#[derive(Clone, Debug, PartialEq)]
pub struct Unicycle {
    pub speed_pursuer: f64,
    pub speed_evader: f64,
    pub turn_rate_pursuer: f64,
    pub turn_rate_evader: f64,
    pub scale_theta: f64,
    pub capture_radius: f64,
    pub horizon: f64,
    pub step: f64,
}

fn descriptive(line: usize, message: impl Into<String>) -> ParseError {
    ParseError::Descriptive {
        line,
        message: message.into(),
    }
}

/// Whether the text claims to describe a non-linear game.
///
/// The driver asks this before parsing as linear, so a `nonlinear` file gets a refusal that
/// names the missing method instead of "unexpected \"nonlinear\"".
pub fn contains_block(text: &str) -> bool {
    text.lines().any(|line| {
        let code = line.split_once('#').map_or(line, |(code, _)| code);
        code.trim_start().to_ascii_lowercase().starts_with("nonlinear")
    })
}

/// Reads a `nonlinear` description.
pub fn parse(text: &str) -> Result<NonlinearProblem, ParseError> {
    let mut kind: Option<String> = None;
    let mut fields: Vec<(String, f64)> = Vec::new();
    let mut grid: Vec<GridAxis> = Vec::new();
    let mut inside = false;

    for (index, raw) in text.lines().enumerate() {
        let line = index + 1;
        let code = raw.split_once('#').map_or(raw, |(code, _)| code);
        let trimmed = code.trim();
        if trimmed.is_empty() {
            inside = false;
            continue;
        }

        let mut parts = trimmed.split_whitespace();
        let head = parts.next().unwrap_or_default();
        let keyword = head.to_ascii_lowercase();

        match keyword.as_str() {
            "nonlinear" => {
                let Some(name) = parts.next() else {
                    return Err(descriptive(line, "`nonlinear` needs a kind, for example `unicycle-pursuit`"));
                };
                if parts.next().is_some() {
                    return Err(descriptive(line, "`nonlinear` takes exactly one kind"));
                }
                if kind.replace(name.to_string()).is_some() {
                    return Err(descriptive(line, "`nonlinear` is given more than once"));
                }
                inside = true;
            }
            "grid" => {
                inside = false;
                let mut next = || parts.next().unwrap_or_default();
                let name = next();
                if name.is_empty() {
                    return Err(descriptive(line, "`grid` needs a name"));
                }
                let mut numbers = [0.0_f64; 3];
                for (slot, label) in numbers.iter_mut().zip(["low", "high", "count"]) {
                    let token = next();
                    *slot = token.parse::<f64>().map_err(|_| {
                        descriptive(line, format!("grid {name}: {label} is not a number, found {token:?}"))
                    })?;
                }
                if parts.next().is_some() {
                    return Err(descriptive(line, "`grid` takes a name, a low, a high and a count"));
                }
                if numbers[2] < 2.0 {
                    return Err(descriptive(line, format!("grid {name}: count must be at least 2")));
                }
                grid.push(GridAxis {
                    name: name.to_string(),
                    low: numbers[0],
                    high: numbers[1],
                    count: numbers[2] as usize,
                });
            }
            "dimension" => {
                inside = false;
            }
            _ if inside => {
                let value = parts.next().ok_or_else(|| {
                    descriptive(line, format!("{head}: expected a number after the field name"))
                })?;
                if parts.next().is_some() {
                    return Err(descriptive(line, format!("{head}: takes exactly one value")));
                }
                if fields.iter().any(|(name, _)| name == head) {
                    return Err(descriptive(line, format!("{head} is given more than once")));
                }
                let parsed = value.parse::<f64>().map_err(|_| {
                    descriptive(line, format!("{head}: expected a number, found {value:?}"))
                })?;
                fields.push((head.to_string(), parsed));
            }
            _ => {
                return Err(descriptive(
                    line,
                    format!("unexpected {head:?}: expected `nonlinear` or `grid` before any field"),
                ));
            }
        }
    }

    let Some(kind) = kind else {
        return Err(descriptive(0, String::from("no `nonlinear` line")));
    };
    Ok(NonlinearProblem { kind, fields, grid })
}

impl NonlinearProblem {
    /// The `unicycle-pursuit` fields, or the first reason they are not usable.
    pub fn unicycle(&self) -> Result<Unicycle, String> {
        if self.kind != "unicycle-pursuit" {
            return Err(format!(
                "unknown nonlinear kind {:?}: expected `unicycle-pursuit`",
                self.kind
            ));
        }
        let mut read = |name: &str| {
            self.fields
                .iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| *value)
                .ok_or_else(|| format!("missing field `{name}`"))
        };
        let case = Unicycle {
            speed_pursuer: read("speed_pursuer")?,
            speed_evader: read("speed_evader")?,
            turn_rate_pursuer: read("turn_rate_pursuer")?,
            turn_rate_evader: read("turn_rate_evader")?,
            scale_theta: read("scale_theta")?,
            capture_radius: read("capture_radius")?,
            horizon: read("horizon")?,
            step: read("step")?,
        };
        if case.turn_rate_evader >= case.turn_rate_pursuer {
            return Err(String::from(
                "the redefined case needs `turn_rate_evader < turn_rate_pursuer`: with a free \
                 evader the bridge collapses to the capture disc",
            ));
        }
        if case.scale_theta <= 0.0 || case.step <= 0.0 || case.horizon <= 0.0 {
            return Err(String::from("scale_theta, step and horizon must be positive"));
        }
        Ok(case)
    }

    /// The message the driver prints for a case it cannot advance.
    pub fn refusal(&self) -> String {
        let axes: Vec<String> = self
            .grid
            .iter()
            .map(|axis| format!("{} {}", axis.name, axis.count))
            .collect();
        let grid = if axes.is_empty() {
            String::from("no reference grid")
        } else {
            format!("grid {}", axes.join(", "))
        };
        format!(
            "`{}` describes a non-linear game ({} fields, {grid}); the interval map for it is \
             track 2's E4 and is not built.\n\
             The case currently runs through `tools/p1_relative.py`; see \
             `kb/entries/02-p1-relative-interior.md`.",
            self.kind,
            self.fields.len(),
        )
    }
}
