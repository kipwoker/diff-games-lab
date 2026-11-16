//! Output formatting: vertices, blocks and the self-describing document.
//!
//! One line per vertex, exactly `n` numbers with nine decimals and single spaces between
//! them, no trailing space, and a `separator` line closing the block. Around that body the
//! writer adds a six-line header and one `block` line per block, so the dimension, `T`,
//! `Step`, the block count and each block's own time travel with the data: a reader can place
//! a shell on the timeline without the input file.

/// Version written on the first line of a document.
pub const VERSION: u32 = 2;

/// Fixed first line of a document, followed by ` v2`.
pub const MAGIC: &str = "stable_bridge output";

/// Formats one vertex: values separated by single spaces, newline at the end.
pub fn format_vertex(vertex: &[f64]) -> String {
    let mut line = String::with_capacity(vertex.len() * 14 + 1);
    for (index, value) in vertex.iter().enumerate() {
        if index + 1 < vertex.len() {
            line.push_str(&format!("{value:.9} "));
        } else {
            line.push_str(&format!("{value:.9}\n"));
        }
    }
    line
}

/// Formats one block: every vertex followed by its `separator` line.
pub fn format_block(vertices: &[Vec<f64>]) -> String {
    let mut block = String::new();
    for vertex in vertices {
        block.push_str(&format_vertex(vertex));
    }
    block.push_str("separator\n");
    block
}

/// Everything the header states about the run.
///
/// All of it is fixed for the whole file, so it is written once instead of repeated on every
/// block. The per-block time is derived from [`Self::step`] and the one-based block index.
#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    /// State dimension: how many coordinates each vertex line holds.
    pub dimension: usize,
    /// Terminal time `T`.
    pub terminal_time: f64,
    /// Step between printed blocks.
    pub step: f64,
    /// Number of blocks that follow.
    pub blocks: usize,
    /// Number of support values that produced each block.
    pub directions: usize,
}

/// Formats a real for the meta lines: fixed notation, no exponent, no padding.
///
/// Nine decimals are formatted exactly as the vertex lines are, then trailing zeros are
/// removed, so `2.0` is written `2` and `3 * 0.1` is written `0.3` rather than
/// `0.30000000000000004`. Precision far below the algorithm's own is dropped on purpose:
/// these fields are labels, not measurements.
pub fn format_real(value: f64) -> String {
    if !value.is_finite() {
        return format!("{value}");
    }

    let mut text = format!("{value:.9}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" {
        text = String::from("0");
    }
    text
}

/// Formats the header, followed by one blank line separating it from the first block.
pub fn format_header(header: &Header) -> String {
    let mut text = format!("{MAGIC} v{VERSION}\n");
    text.push_str(&format!("dimension {}\n", header.dimension));
    text.push_str(&format!("T {}\n", format_real(header.terminal_time)));
    text.push_str(&format!("Step {}\n", format_real(header.step)));
    text.push_str(&format!("blocks {}\n", header.blocks));
    text.push_str(&format!("directions {}\n", header.directions));
    text.push('\n');
    text
}

/// Formats one block's meta line: `block <k> time <t> vertices <n>`.
///
/// `index` is one-based, so the first block of a ten-block run is `block 1 time 0.2 …`.
pub fn format_block_header(index: usize, time: f64, vertices: usize) -> String {
    format!(
        "block {index} time {} vertices {vertices}\n",
        format_real(time)
    )
}

/// Formats a whole document: header, then every block preceded by its meta line.
pub fn format_document(header: &Header, blocks: &[Vec<Vec<f64>>]) -> String {
    let mut text = format_header(header);
    for (offset, block) in blocks.iter().enumerate() {
        let index = offset + 1;
        text.push_str(&format_block_header(
            index,
            index as f64 * header.step,
            block.len(),
        ));
        text.push_str(&format_block(block));
    }
    text
}
