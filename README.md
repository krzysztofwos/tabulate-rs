# tabulate-rs

Rust port of the excellent [python-tabulate](https://github.com/astanin/python-tabulate) library for pretty-printing tabular data.

## Project Goals

- Provide a drop-in textual table renderer for Rust that mirrors the Python project’s behaviour.
- Offer a builder-style API (`TabulateOptions`) that keeps configuration explicit while defaulting to the same semantics as python-tabulate.
- Maintain feature parity with python-tabulate **0.9.0**, covering formats, alignment rules, wrapping, and data normalisation.

## Parity Status (python-tabulate 0.9.0)

The current implementation matches python-tabulate 0.9.0 for:

- **Table formats**: all 30+ built-ins including plain/simple/grid variants, pipe, GitHub/MediaWiki/Textile, HTML/unsafeHTML, LaTeX family, ANSI-aware formats, wide-character grids, ASCII/Unicode outlines, TSV, YouTrack, AsciiDoc, and custom `simple_separated_format`.
- **Formatting features**:
  - Numeric parsing with per-column disable lists (`disable_numparse` and `disable_numparse_columns`).
  - Column and header alignment (global, per-column, decimal alignment, row alignment, “same as column” header semantics).
  - Word wrapping with `max_col_widths` / `max_header_col_widths`, multiline cell handling, and whitespace preservation.
  - Missing value placeholders (single and per-column).
- **Data inputs**: sequences of sequences, dicts and dicts-of-iterables (with padding for uneven columns), namedtuples, dataclasses, NumPy ndarrays and structured/record arrays, pandas DataFrames (including multi-index), and data sources using `SEPARATING_LINE`.

Snapshot fixtures (`tests/fixtures/python_snapshots.json`) are generated directly from python-tabulate 0.9.0 via `scripts/generate_snapshots.py`, and the `python_snapshot_parity` test ensures all formats and options stay in sync.

```bash
python scripts/generate_snapshots.py
```

## Quick Start

```rust
use tabulate::{tabulate, TabulateOptions};

let data = vec![
    vec!["Planet", "Radius (km)", "Mass (10^24 kg)"],
    vec!["Mercury", "2440", "0.330"],
    vec!["Venus", "6052", "4.87"],
];

let table = tabulate(
    data,
    TabulateOptions::new()
        .headers(tabulate::Headers::FirstRow)
        .table_format("grid"),
).unwrap();

println!("{table}");
```

## Development Notes

- Run `cargo test` to ensure unit tests and snapshot parity checks pass.
- Regenerate Python fixtures via `python /tmp/generate_snapshots.py` after adding new cases.
- The crate re-exports the table format registry so custom `TableFormat` instances can be supplied via `TabulateOptions::table_format_custom`.

## License

MIT OR Apache-2.0
