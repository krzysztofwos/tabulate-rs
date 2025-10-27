//! Pretty-print tabular data, ported from the [`python-tabulate`](https://github.com/astanin/python-tabulate)
//! project.
//!
//! The crate exposes a single entry point – [`tabulate`] – similar to the original Python
//! implementation. The API is intentionally builder-based to make configuration explicit while
//! staying close to the reference behaviour.

#![deny(missing_docs)]

mod alignment;
mod constants;
mod format;
mod options;
mod table;
mod width;

pub use alignment::Alignment;
pub use constants::SEPARATING_LINE;
pub use format::{DataRow, Line, TableFormat, simple_separated_format, tabulate_formats};
pub use options::{
    FormatSpec, HeaderAlignment, Headers, MissingValues, RowAlignment, ShowIndex, TabulateOptions,
};
pub use table::{TabulateError, tabulate};
