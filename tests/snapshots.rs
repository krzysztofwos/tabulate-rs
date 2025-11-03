use std::{collections::BTreeMap, fs::File, io::BufReader};

use serde::Deserialize;
use tabulate_rs::{
    Alignment, HeaderAlignment, Headers, RowAlignment, ShowIndex, TabulateOptions, tabulate,
};

#[derive(Deserialize)]
struct SnapshotCase {
    data: Vec<serde_json::Value>,
    kwargs: serde_json::Value,
    output: String,
}

fn load_snapshots() -> BTreeMap<String, SnapshotCase> {
    let file =
        File::open("tests/fixtures/python_snapshots.json").expect("snapshot fixture present");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("valid snapshot json")
}

fn apply_options(case: &SnapshotCase) -> TabulateOptions {
    use serde_json::Value;

    let mut options = TabulateOptions::new();
    let kwargs = case.kwargs.as_object().expect("kwargs must be an object");

    if let Some(Value::String(fmt)) = kwargs.get("tablefmt") {
        options = options.table_format(fmt.clone());
    }

    if let Some(headers) = kwargs.get("headers") {
        match headers {
            Value::String(s) if s == "firstrow" => {
                options = options.headers(Headers::FirstRow);
            }
            Value::String(s) if s == "keys" => {
                options = options.headers(Headers::Keys);
            }
            Value::Array(values) => {
                let headers: Vec<String> = values
                    .iter()
                    .map(|value| match value {
                        Value::String(s) => s.clone(),
                        other => other.to_string(),
                    })
                    .collect();
                options = options.headers(Headers::Explicit(headers));
            }
            _ => {}
        }
    }

    if let Some(colalign) = kwargs.get("colalign")
        && let Some(array) = colalign.as_array()
    {
        let aligns = array
            .iter()
            .map(|value| value.as_str().and_then(parse_alignment));
        options = options.col_alignments(aligns.collect::<Vec<_>>());
    }

    if let Some(rowalign) = kwargs.get("rowalign") {
        match rowalign {
            Value::String(s) => {
                if let Some(alignment) = parse_row_alignment(s) {
                    options = options.row_global_align(alignment);
                }
            }
            Value::Array(items) => {
                let aligns = items
                    .iter()
                    .map(|value| value.as_str().and_then(parse_row_alignment));
                options = options.row_alignments(aligns.collect::<Vec<_>>());
            }
            _ => {}
        }
    }

    if let Some(Value::String(colglobal)) = kwargs.get("colglobalalign")
        && let Some(alignment) = parse_alignment(colglobal)
    {
        options = options.col_global_align(alignment);
    }

    if let Some(Value::String(headersglobal)) = kwargs.get("headersglobalalign")
        && let Some(alignment) = parse_alignment(headersglobal)
    {
        options = options.headers_global_align(alignment);
    }

    if let Some(Value::Array(headersalign)) = kwargs.get("headersalign") {
        let aligns = headersalign
            .iter()
            .map(|value| value.as_str().and_then(parse_header_alignment));
        options = options.headers_alignments(aligns.collect::<Vec<_>>());
    }

    if let Some(Value::String(numalign)) = kwargs.get("numalign")
        && let Some(alignment) = parse_alignment(numalign)
    {
        options = options.num_align(alignment);
    }

    if let Some(Value::String(stralign)) = kwargs.get("stralign")
        && let Some(alignment) = parse_alignment(stralign)
    {
        options = options.str_align(alignment);
    }

    if let Some(disable_numparse) = kwargs.get("disable_numparse") {
        match disable_numparse {
            Value::Bool(flag) => {
                options = options.disable_numparse(*flag);
            }
            Value::Array(columns) => {
                let indices = columns
                    .iter()
                    .filter_map(|value| value.as_u64().map(|v| v as usize))
                    .collect::<Vec<_>>();
                options = options.disable_numparse_columns(indices);
            }
            _ => {}
        }
    }

    if let Some(Value::Bool(true)) = kwargs.get("preserve_whitespace") {
        options = options.preserve_whitespace(true);
    }

    if let Some(showindex) = kwargs.get("showindex") {
        options = options.show_index(parse_show_index(showindex));
    }

    if let Some(Value::Number(width)) = kwargs.get("maxcolwidths") {
        if let Some(value) = width.as_u64() {
            options = options.max_col_width(value as usize);
        }
    } else if let Some(Value::Array(values)) = kwargs.get("maxcolwidths") {
        let widths = values
            .iter()
            .map(|value| match value {
                Value::Number(n) => n.as_u64().map(|v| v as usize),
                Value::Null => None,
                _ => None,
            })
            .collect::<Vec<_>>();
        options = options.max_col_widths(widths);
    }

    if let Some(Value::Number(width)) = kwargs.get("maxheadercolwidths") {
        if let Some(value) = width.as_u64() {
            options = options.max_header_col_width(value as usize);
        }
    } else if let Some(Value::Array(values)) = kwargs.get("maxheadercolwidths") {
        let widths = values
            .iter()
            .map(|value| match value {
                Value::Number(n) => n.as_u64().map(|v| v as usize),
                Value::Null => None,
                _ => None,
            })
            .collect::<Vec<_>>();
        options = options.max_header_col_widths(widths);
    }

    options
}

fn parse_row_alignment(value: &str) -> Option<RowAlignment> {
    match value {
        "top" => Some(RowAlignment::Top),
        "center" => Some(RowAlignment::Center),
        "bottom" => Some(RowAlignment::Bottom),
        _ => None,
    }
}

fn parse_alignment(value: &str) -> Option<Alignment> {
    match value {
        "left" => Some(Alignment::Left),
        "right" => Some(Alignment::Right),
        "center" => Some(Alignment::Center),
        "decimal" => Some(Alignment::Decimal),
        "default" | "global" | "same" => None,
        _ => None,
    }
}

fn parse_header_alignment(value: &str) -> Option<HeaderAlignment> {
    match value {
        "same" => Some(HeaderAlignment::SameAsColumn),
        "default" | "global" => None,
        other => parse_alignment(other).map(HeaderAlignment::Align),
    }
}

fn parse_show_index(value: &serde_json::Value) -> ShowIndex {
    match value {
        serde_json::Value::String(s) if s == "always" => ShowIndex::Always,
        serde_json::Value::String(s) if s == "never" => ShowIndex::Never,
        serde_json::Value::String(_) => ShowIndex::Default,
        serde_json::Value::Bool(true) => ShowIndex::Always,
        serde_json::Value::Bool(false) => ShowIndex::Never,
        serde_json::Value::Array(items) => ShowIndex::Values(
            items
                .iter()
                .map(|value| {
                    value
                        .as_str()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| value.to_string())
                })
                .collect(),
        ),
        _ => ShowIndex::Default,
    }
}

#[test]
fn python_snapshot_parity() {
    let snapshots = load_snapshots();
    for (name, case) in snapshots {
        let rust_output =
            tabulate(case.data.clone(), apply_options(&case)).expect("render succeeds");
        assert_eq!(rust_output, case.output, "snapshot {name} matches python");
    }
}
