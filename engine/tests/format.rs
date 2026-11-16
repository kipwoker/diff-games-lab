//! The input grammar and the output document.
//!
//! The parser must accept directives in any order, report the line of a malformed block, and
//! check every shape against the others. The writer must state everything a reader needs to
//! place a block on the timeline without the input file.

use std::path::PathBuf;

use stable_bridge::input_parser::{self, ParseError};
use stable_bridge::output_writer::{self, Header};

fn manifest_path(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for part in parts {
        path.push(part);
    }
    path
}

fn sample_text() -> String {
    let path = manifest_path(&["tests", "data", "input_descriptive.txt"]);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

#[test]
fn the_sample_problem_parses_completely() {
    let data = input_parser::parse(&sample_text()).expect("sample should parse");
    assert_eq!((data.n, data.m), (2, 4));
    assert_eq!((data.dim_a, data.cols_b, data.cols_c), (2, 2, 2));
    assert_eq!((data.rows_pi, data.p.len(), data.q.len()), (2, 4, 4));
    assert_eq!((data.t, data.step), (2.0, 0.2));
    assert_eq!(data.block_count(), 10);
}

#[test]
fn directives_may_come_in_any_order_and_carry_inline_comments() {
    let text = "\
step 0.25   # printed every quarter
time 1
matrix pi
1 0
0 1
dimension 2
matrix A
0 0
1 0
matrix B
1 0
0 1
matrix C
1 0
0 1
constraints Q
1 0 2
-1 0 2
0 -1 2
0 1 2
constraints P
1 0 2
-1 0 2
0 -1 2
0 1 2
constraints M
1 0 0.5
-1 0 0.5
0 -1 0.5
0 1 0.5
";

    let data = input_parser::parse(text).expect("order must not matter");
    assert_eq!((data.n, data.dim_a, data.cols_b), (2, 2, 2));
    assert_eq!((data.t, data.step), (1.0, 0.25));
    assert_eq!(data.block_count(), 4);
    assert_eq!(data.m_rows.len(), 4);
    assert_eq!(data.q.len(), 4);
}

#[test]
fn a_ragged_matrix_is_reported_with_its_line() {
    let text = "\
dimension 2
matrix A
0 0
1
";
    let error = input_parser::parse(text).expect_err("a short row must be rejected");
    assert_eq!(
        error,
        ParseError::Descriptive {
            line: 4,
            message: String::from("A: row 2 has 1 values, but the first row has 2"),
        }
    );
}

#[test]
fn an_unknown_directive_is_reported_with_its_line() {
    let text = "dimension 2\nmatrx A\n0 0\n";
    match input_parser::parse(text) {
        Err(ParseError::Descriptive { line, message }) => {
            assert_eq!(line, 2);
            assert!(message.contains("matrx"), "unhelpful message: {message}");
        }
        other => panic!("expected a line-tagged error, got {other:?}"),
    }
}

#[test]
fn every_block_is_required() {
    let text = "\
dimension 2
matrix A
0 0
1 0
time 2
step 0.2
";
    let error = input_parser::parse(text).expect_err("B, C, pi, M, P and Q are still missing");
    assert_eq!(
        error,
        ParseError::Descriptive {
            line: 0,
            message: String::from("no `B` block"),
        }
    );
}

#[test]
fn shapes_are_checked_against_each_other() {
    // M rows are three wide, but `dimension 3` says they need four fields.
    let text = "\
dimension 3
constraints M
1 0 0.5
matrix A
0 0 0
1 0 0
0 0 0
matrix B
1 0 0
0 1 0
0 0 1
matrix C
1 0 0
0 1 0
0 0 1
matrix pi
1 0 0
0 1 0
0 0 1
constraints P
1 0 0 2
constraints Q
1 0 0 2
time 1
step 0.5
";
    match input_parser::parse(text) {
        Err(ParseError::Descriptive { line, message }) => {
            assert_eq!(line, 2);
            assert!(message.starts_with("M: row 1 has 3 values"), "{message}");
        }
        other => panic!("expected M's width to be rejected, got {other:?}"),
    }
}

#[test]
fn a_document_header_states_everything_the_blocks_need() {
    let header = Header {
        dimension: 3,
        terminal_time: 1.0,
        step: 0.5,
        blocks: 2,
        directions: 110,
    };
    let blocks = vec![
        vec![vec![0.5, -0.5, 0.25], vec![1.0, 2.0, 3.0]],
        vec![vec![0.0, 0.0, 0.0]],
    ];

    let text = output_writer::format_document(&header, &blocks);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        &lines[..6],
        &[
            "stable_bridge output v2",
            "dimension 3",
            "T 1",
            "Step 0.5",
            "blocks 2",
            "directions 110",
        ]
    );

    // Each block says its one-based index, its own time and how many vertices follow.
    assert!(text.contains("block 1 time 0.5 vertices 2\n"));
    assert!(text.contains("block 2 time 1 vertices 1\n"));

    // The vertices are the body, and the document only wraps them: each body starts right
    // after its meta line, and the last one runs to the end of the file.
    assert!(text.contains(&format!(
        "vertices 2\n{}",
        output_writer::format_block(&blocks[0])
    )));
    assert!(text.ends_with(&output_writer::format_block(&blocks[1])));
    assert_eq!(
        text.lines().filter(|line| *line == "separator").count(),
        2,
        "one separator per block"
    );
}

#[test]
fn meta_reals_are_short_but_keep_their_value() {
    assert_eq!(output_writer::format_real(2.0), "2");
    assert_eq!(output_writer::format_real(0.2), "0.2");
    assert_eq!(output_writer::format_real(3.0 * 0.1), "0.3");
    assert_eq!(output_writer::format_real(-0.25), "-0.25");
    assert_eq!(output_writer::format_real(0.0), "0");
}

/// The version marker is part of the contract: a reader keys on it.
#[test]
fn the_document_starts_with_its_magic_and_version() {
    let header = Header {
        dimension: 2,
        terminal_time: 2.0,
        step: 0.2,
        blocks: 0,
        directions: 110,
    };
    assert!(output_writer::format_header(&header)
        .starts_with("stable_bridge output v2\n"));
    assert_eq!(output_writer::VERSION, 2);
}
