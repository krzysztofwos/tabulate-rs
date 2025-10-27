use std::{borrow::Cow, collections::BTreeMap};

use once_cell::sync::Lazy;

use crate::{alignment::Alignment, width::visible_width};

/// Horizontal line descriptor.
#[derive(Clone, Debug)]
pub struct Line {
    /// Characters printed before the first column.
    pub begin: Cow<'static, str>,
    /// Characters used to fill the gap above/below a column.
    pub fill: Cow<'static, str>,
    /// Characters placed between columns.
    pub separator: Cow<'static, str>,
    /// Characters printed after the last column.
    pub end: Cow<'static, str>,
}

impl Line {
    fn borrowed(
        begin: &'static str,
        fill: &'static str,
        separator: &'static str,
        end: &'static str,
    ) -> Self {
        Self {
            begin: Cow::Borrowed(begin),
            fill: Cow::Borrowed(fill),
            separator: Cow::Borrowed(separator),
            end: Cow::Borrowed(end),
        }
    }
}

/// Row descriptor.
#[derive(Clone, Debug)]
pub struct DataRow {
    /// Characters printed before the first cell.
    pub begin: Cow<'static, str>,
    /// Characters printed in-between adjacent cells.
    pub separator: Cow<'static, str>,
    /// Characters appended after the last cell.
    pub end: Cow<'static, str>,
}

impl DataRow {
    fn borrowed(begin: &'static str, separator: &'static str, end: &'static str) -> Self {
        Self {
            begin: Cow::Borrowed(begin),
            separator: Cow::Borrowed(separator),
            end: Cow::Borrowed(end),
        }
    }
}

/// A function that generates a horizontal rule dynamically.
pub type LineFn = fn(col_widths: &[usize], col_aligns: &[Alignment]) -> String;
/// A function that renders a single row dynamically.
pub type RowFn =
    fn(cell_values: &[String], col_widths: &[usize], col_aligns: &[Alignment]) -> String;

/// Representation of the different ways a table component can be rendered.
#[derive(Clone, Debug, Default)]
pub enum LineFormat {
    /// Do not render the line.
    #[default]
    None,
    /// Render a static [`Line`].
    Static(Line),
    /// Render an inline string as-is.
    Text(Cow<'static, str>),
    /// Render with a dynamic callback.
    Dynamic(LineFn),
}

/// Representation of the different ways a table row can be rendered.
#[derive(Clone, Debug, Default)]
pub enum RowFormat {
    /// Do not render the row.
    #[default]
    None,
    /// Render a static [`DataRow`].
    Static(DataRow),
    /// Render via a dynamic callback.
    Dynamic(RowFn),
}

/// Table format definition mirroring the Python implementation.
#[derive(Clone, Debug)]
pub struct TableFormat {
    /// Line printed above the table.
    pub line_above: LineFormat,
    /// Line between headers and data rows.
    pub line_below_header: LineFormat,
    /// Line between consecutive data rows.
    pub line_between_rows: LineFormat,
    /// Line printed after the final row.
    pub line_below: LineFormat,
    /// Header row template.
    pub header_row: RowFormat,
    /// Data row template.
    pub data_row: RowFormat,
    /// Extra padding applied around each cell.
    pub padding: usize,
    /// Table components that should be hidden when headers are present.
    pub with_header_hide: &'static [&'static str],
}

impl TableFormat {
    /// A helper that returns `true` if the format should hide the requested component.
    pub fn hides(&self, component: &str) -> bool {
        self.with_header_hide.contains(&component)
    }
}

fn line(
    begin: &'static str,
    fill: &'static str,
    separator: &'static str,
    end: &'static str,
) -> Line {
    Line::borrowed(begin, fill, separator, end)
}

fn row(begin: &'static str, separator: &'static str, end: &'static str) -> DataRow {
    DataRow::borrowed(begin, separator, end)
}

fn default_align(aligns: &[Alignment], index: usize) -> Alignment {
    aligns.get(index).copied().unwrap_or(Alignment::Left)
}

fn pipe_segment_with_colons(align: Alignment, width: usize) -> String {
    let width = width.max(1);
    match align {
        Alignment::Right | Alignment::Decimal => {
            if width == 1 {
                ":".to_string()
            } else {
                format!("{:-<w$}:", "-", w = width - 1)
            }
        }
        Alignment::Center => {
            if width <= 2 {
                "::".chars().take(width).collect()
            } else {
                format!(":{}:", "-".repeat(width - 2))
            }
        }
        Alignment::Left => {
            if width == 1 {
                ":".to_string()
            } else {
                format!(":{:-<w$}", "-", w = width - 1)
            }
        }
    }
}

fn pipe_line_with_colons(col_widths: &[usize], col_aligns: &[Alignment]) -> String {
    if col_aligns.is_empty() {
        let line = col_widths
            .iter()
            .map(|w| "-".repeat(*w))
            .collect::<Vec<_>>()
            .join("|");
        return format!("|{}|", line);
    }

    let segments = col_widths
        .iter()
        .enumerate()
        .map(|(idx, width)| pipe_segment_with_colons(default_align(col_aligns, idx), *width))
        .collect::<Vec<_>>();
    format!("|{}|", segments.join("|"))
}

fn mediawiki_row_with_attrs(
    separator: &str,
    cell_values: &[String],
    col_aligns: &[Alignment],
) -> String {
    let mut values = Vec::with_capacity(cell_values.len());
    for (idx, cell) in cell_values.iter().enumerate() {
        let align = match default_align(col_aligns, idx) {
            Alignment::Right | Alignment::Decimal => r#" align="right"| "#,
            Alignment::Center => r#" align="center"| "#,
            Alignment::Left => "",
        };
        let mut value = String::new();
        if align.is_empty() {
            value.push(' ');
            value.push_str(cell);
            value.push(' ');
        } else {
            value.push_str(align);
            value.push_str(cell);
        }
        values.push(value);
    }
    let mut result = String::new();
    let colsep = separator.repeat(2);
    result.push_str(separator);
    result.push_str(&values.join(&colsep));
    while result.ends_with(char::is_whitespace) {
        result.pop();
    }
    result
}

fn mediawiki_header_row(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    let _ = col_widths;
    mediawiki_row_with_attrs("!", cell_values, col_aligns)
}

fn mediawiki_data_row(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    let _ = col_widths;
    mediawiki_row_with_attrs("|", cell_values, col_aligns)
}

fn moin_row_with_attrs(
    celltag: &str,
    header: Option<&str>,
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    let mut out = String::new();
    for (idx, value) in cell_values.iter().enumerate() {
        let alignment = default_align(col_aligns, idx);
        let align_attr = match alignment {
            Alignment::Right | Alignment::Decimal => r#"<style="text-align: right;">"#,
            Alignment::Center => r#"<style="text-align: center;">"#,
            Alignment::Left => "",
        };
        let total_width = col_widths.get(idx).copied().unwrap_or(value.len());
        let inner_width = total_width.saturating_sub(2);
        let mut core = value.clone();
        let current_width = visible_width(&core, false);
        if current_width < inner_width {
            let padding = inner_width - current_width;
            let (left_pad, right_pad) = match alignment {
                Alignment::Right | Alignment::Decimal => (padding, 0),
                Alignment::Center => (padding / 2, padding - (padding / 2)),
                Alignment::Left => (0, padding),
            };
            core = format!("{}{}{}", " ".repeat(left_pad), core, " ".repeat(right_pad));
        }
        let mut base = String::new();
        if let Some(marker) = header {
            base.push_str(marker);
        }
        base.push_str(&core);
        if let Some(marker) = header {
            base.push_str(marker);
        }
        out.push_str(celltag);
        out.push_str(align_attr);
        out.push(' ');
        out.push_str(&base);
        out.push(' ');
    }
    out.push_str("||");
    out
}

fn moin_header_row(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    moin_row_with_attrs("||", Some("'''"), cell_values, col_widths, col_aligns)
}

fn moin_data_row(cell_values: &[String], col_widths: &[usize], col_aligns: &[Alignment]) -> String {
    moin_row_with_attrs("||", None, cell_values, col_widths, col_aligns)
}

fn html_begin_table_without_header(_col_widths: &[usize], _col_aligns: &[Alignment]) -> String {
    "<table>\n<tbody>".to_string()
}

fn html_escape(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#39;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

fn html_row_with_attrs(
    celltag: &str,
    unsafe_mode: bool,
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    let _ = col_widths;
    let mut values = Vec::with_capacity(cell_values.len());
    for (idx, cell) in cell_values.iter().enumerate() {
        let align = match default_align(col_aligns, idx) {
            Alignment::Right | Alignment::Decimal => r#" style="text-align: right;""#,
            Alignment::Center => r#" style="text-align: center;""#,
            Alignment::Left => "",
        };
        let content = if unsafe_mode {
            cell.clone()
        } else {
            html_escape(cell)
        };
        let value = format!("<{celltag}{align}>{content}</{celltag}>");
        values.push(value);
    }
    let mut rowhtml = format!("<tr>{}</tr>", values.join(""));
    rowhtml.truncate(rowhtml.trim_end().len());
    if celltag == "th" {
        format!("<table>\n<thead>\n{rowhtml}\n</thead>\n<tbody>")
    } else {
        rowhtml
    }
}

fn html_header_row_safe(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    html_row_with_attrs("th", false, cell_values, col_widths, col_aligns)
}

fn html_data_row_safe(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    html_row_with_attrs("td", false, cell_values, col_widths, col_aligns)
}

fn html_header_row_unsafe(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    html_row_with_attrs("th", true, cell_values, col_widths, col_aligns)
}

fn html_data_row_unsafe(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    html_row_with_attrs("td", true, cell_values, col_widths, col_aligns)
}

fn latex_line_begin_tabular(_col_widths: &[usize], col_aligns: &[Alignment]) -> String {
    latex_line_begin_tabular_internal(col_aligns, false, false)
}

fn latex_line_begin_tabular_booktabs(_col_widths: &[usize], col_aligns: &[Alignment]) -> String {
    latex_line_begin_tabular_internal(col_aligns, true, false)
}

fn latex_line_begin_tabular_longtable(_col_widths: &[usize], col_aligns: &[Alignment]) -> String {
    latex_line_begin_tabular_internal(col_aligns, false, true)
}

fn latex_line_begin_tabular_internal(
    col_aligns: &[Alignment],
    booktabs: bool,
    longtable: bool,
) -> String {
    let columns: String = col_aligns
        .iter()
        .map(|align| match align {
            Alignment::Right | Alignment::Decimal => 'r',
            Alignment::Center => 'c',
            Alignment::Left => 'l',
        })
        .collect();
    let begin = if longtable {
        "\\begin{longtable}{"
    } else {
        "\\begin{tabular}{"
    };
    let mut result = String::new();
    result.push_str(begin);
    result.push_str(&columns);
    result.push_str("}\n");
    result.push_str(if booktabs { "\\toprule" } else { "\\hline" });
    result
}

const LATEX_ESCAPE_RULES: &[(char, &str)] = &[
    ('&', r"\&"),
    ('%', r"\%"),
    ('$', r"\$"),
    ('#', r"\#"),
    ('_', r"\_"),
    ('^', r"\^{}"),
    ('{', r"\{"),
    ('}', r"\}"),
    ('~', r"\textasciitilde{}"),
    ('\\', r"\textbackslash{}"),
    ('<', r"\ensuremath{<}"),
    ('>', r"\ensuremath{>}"),
];

fn latex_escape(value: &str, escape: bool) -> String {
    if !escape {
        return value.to_string();
    }
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if let Some((_, replacement)) = LATEX_ESCAPE_RULES
            .iter()
            .find(|(candidate, _)| *candidate == ch)
        {
            out.push_str(replacement);
        } else {
            out.push(ch);
        }
    }
    out
}

fn latex_row(cell_values: &[String], col_widths: &[usize], col_aligns: &[Alignment]) -> String {
    latex_row_internal(cell_values, col_widths, col_aligns, true)
}

fn latex_row_raw(cell_values: &[String], col_widths: &[usize], col_aligns: &[Alignment]) -> String {
    latex_row_internal(cell_values, col_widths, col_aligns, false)
}

fn latex_row_internal(
    cell_values: &[String],
    _col_widths: &[usize],
    _col_aligns: &[Alignment],
    escape: bool,
) -> String {
    let escaped = cell_values
        .iter()
        .map(|cell| latex_escape(cell, escape))
        .collect::<Vec<_>>();
    format!("{}\\\\", escaped.join("&"))
}

fn textile_row_with_attrs(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    let _ = col_widths;
    if cell_values.is_empty() {
        return String::from("||");
    }
    let mut values = cell_values.to_vec();
    if let Some(first) = values.first_mut() {
        first.push(' ');
    }
    let mut parts = Vec::with_capacity(values.len());
    for (idx, value) in values.into_iter().enumerate() {
        let align_prefix = match default_align(col_aligns, idx) {
            Alignment::Left => "<.",
            Alignment::Right | Alignment::Decimal => ">.",
            Alignment::Center => "=.",
        };
        parts.push(format!("{}{}", align_prefix, value));
    }
    format!("|{}|", parts.join("|"))
}

fn asciidoc_alignment_code(align: Alignment) -> char {
    match align {
        Alignment::Left => '<',
        Alignment::Right | Alignment::Decimal => '>',
        Alignment::Center => '^',
    }
}

fn asciidoc_make_header_line(
    is_header: bool,
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    let mut column_specifiers = Vec::new();
    for (idx, width) in col_widths.iter().enumerate() {
        let align_char = asciidoc_alignment_code(default_align(col_aligns, idx));
        column_specifiers.push(format!("{width}{align_char}"));
    }
    let mut header_entries = vec![format!("cols=\"{}\"", column_specifiers.join(","))];
    if is_header {
        header_entries.push("options=\"header\"".to_string());
    }
    format!("[{}]\n|====", header_entries.join(","))
}

fn asciidoc_line_above(col_widths: &[usize], col_aligns: &[Alignment]) -> String {
    asciidoc_make_header_line(false, col_widths, col_aligns)
}

fn asciidoc_header_row(
    cell_values: &[String],
    col_widths: &[usize],
    col_aligns: &[Alignment],
) -> String {
    let header = asciidoc_make_header_line(true, col_widths, col_aligns);
    let data_line = format!("|{}", cell_values.join("|"));
    format!("{header}\n{data_line}")
}

fn asciidoc_data_row(
    cell_values: &[String],
    _col_widths: &[usize],
    _col_aligns: &[Alignment],
) -> String {
    format!("|{}", cell_values.join("|"))
}

fn build_formats() -> BTreeMap<&'static str, TableFormat> {
    let mut formats = BTreeMap::new();
    formats.insert(
        "asciidoc",
        TableFormat {
            line_above: LineFormat::Dynamic(asciidoc_line_above),
            line_below_header: LineFormat::None,
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("|====", "", "", "")),
            header_row: RowFormat::Dynamic(asciidoc_header_row),
            data_row: RowFormat::Dynamic(asciidoc_data_row),
            padding: 1,
            with_header_hide: &["lineabove"],
        },
    );
    formats.insert(
        "colon_grid",
        TableFormat {
            line_above: LineFormat::Static(line("", "-", "  ", "")),
            line_below_header: LineFormat::Static(line("", "-", "  ", "")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("", "-", "  ", "")),
            header_row: RowFormat::Static(row("", "  ", "")),
            data_row: RowFormat::Static(row("", "  ", "")),
            padding: 0,
            with_header_hide: &["lineabove", "linebelow"],
        },
    );
    formats.insert(
        "double_grid",
        TableFormat {
            line_above: LineFormat::Static(line("\u{2554}", "\u{2550}", "\u{2566}", "\u{2557}")),
            line_below_header: LineFormat::Static(line(
                "\u{2560}", "\u{2550}", "\u{256c}", "\u{2563}",
            )),
            line_between_rows: LineFormat::Static(line(
                "\u{2560}", "\u{2550}", "\u{256c}", "\u{2563}",
            )),
            line_below: LineFormat::Static(line("\u{255a}", "\u{2550}", "\u{2569}", "\u{255d}")),
            header_row: RowFormat::Static(row("\u{2551}", "\u{2551}", "\u{2551}")),
            data_row: RowFormat::Static(row("\u{2551}", "\u{2551}", "\u{2551}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "double_outline",
        TableFormat {
            line_above: LineFormat::Static(line("\u{2554}", "\u{2550}", "\u{2566}", "\u{2557}")),
            line_below_header: LineFormat::Static(line(
                "\u{2560}", "\u{2550}", "\u{256c}", "\u{2563}",
            )),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\u{255a}", "\u{2550}", "\u{2569}", "\u{255d}")),
            header_row: RowFormat::Static(row("\u{2551}", "\u{2551}", "\u{2551}")),
            data_row: RowFormat::Static(row("\u{2551}", "\u{2551}", "\u{2551}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "fancy_grid",
        TableFormat {
            line_above: LineFormat::Static(line("\u{2552}", "\u{2550}", "\u{2564}", "\u{2555}")),
            line_below_header: LineFormat::Static(line(
                "\u{255e}", "\u{2550}", "\u{256a}", "\u{2561}",
            )),
            line_between_rows: LineFormat::Static(line(
                "\u{251c}", "\u{2500}", "\u{253c}", "\u{2524}",
            )),
            line_below: LineFormat::Static(line("\u{2558}", "\u{2550}", "\u{2567}", "\u{255b}")),
            header_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            data_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "fancy_outline",
        TableFormat {
            line_above: LineFormat::Static(line("\u{2552}", "\u{2550}", "\u{2564}", "\u{2555}")),
            line_below_header: LineFormat::Static(line(
                "\u{255e}", "\u{2550}", "\u{256a}", "\u{2561}",
            )),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\u{2558}", "\u{2550}", "\u{2567}", "\u{255b}")),
            header_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            data_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "github",
        TableFormat {
            line_above: LineFormat::Static(line("|", "-", "|", "|")),
            line_below_header: LineFormat::Static(line("|", "-", "|", "|")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("|", "|", "|")),
            data_row: RowFormat::Static(row("|", "|", "|")),
            padding: 1,
            with_header_hide: &["lineabove"],
        },
    );
    formats.insert(
        "grid",
        TableFormat {
            line_above: LineFormat::Static(line("+", "-", "+", "+")),
            line_below_header: LineFormat::Static(line("+", "=", "+", "+")),
            line_between_rows: LineFormat::Static(line("+", "-", "+", "+")),
            line_below: LineFormat::Static(line("+", "-", "+", "+")),
            header_row: RowFormat::Static(row("|", "|", "|")),
            data_row: RowFormat::Static(row("|", "|", "|")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "heavy_grid",
        TableFormat {
            line_above: LineFormat::Static(line("\u{250f}", "\u{2501}", "\u{2533}", "\u{2513}")),
            line_below_header: LineFormat::Static(line(
                "\u{2523}", "\u{2501}", "\u{254b}", "\u{252b}",
            )),
            line_between_rows: LineFormat::Static(line(
                "\u{2523}", "\u{2501}", "\u{254b}", "\u{252b}",
            )),
            line_below: LineFormat::Static(line("\u{2517}", "\u{2501}", "\u{253b}", "\u{251b}")),
            header_row: RowFormat::Static(row("\u{2503}", "\u{2503}", "\u{2503}")),
            data_row: RowFormat::Static(row("\u{2503}", "\u{2503}", "\u{2503}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "heavy_outline",
        TableFormat {
            line_above: LineFormat::Static(line("\u{250f}", "\u{2501}", "\u{2533}", "\u{2513}")),
            line_below_header: LineFormat::Static(line(
                "\u{2523}", "\u{2501}", "\u{254b}", "\u{252b}",
            )),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\u{2517}", "\u{2501}", "\u{253b}", "\u{251b}")),
            header_row: RowFormat::Static(row("\u{2503}", "\u{2503}", "\u{2503}")),
            data_row: RowFormat::Static(row("\u{2503}", "\u{2503}", "\u{2503}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "html",
        TableFormat {
            line_above: LineFormat::Dynamic(html_begin_table_without_header),
            line_below_header: LineFormat::Text(Cow::Borrowed("")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("</tbody>\n</table>", "", "", "")),
            header_row: RowFormat::Dynamic(html_header_row_safe),
            data_row: RowFormat::Dynamic(html_data_row_safe),
            padding: 0,
            with_header_hide: &["lineabove"],
        },
    );
    formats.insert(
        "jira",
        TableFormat {
            line_above: LineFormat::None,
            line_below_header: LineFormat::None,
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("||", "||", "||")),
            data_row: RowFormat::Static(row("|", "|", "|")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "latex",
        TableFormat {
            line_above: LineFormat::Dynamic(latex_line_begin_tabular),
            line_below_header: LineFormat::Static(line("\\hline", "", "", "")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\\hline\n\\end{tabular}", "", "", "")),
            header_row: RowFormat::Dynamic(latex_row),
            data_row: RowFormat::Dynamic(latex_row),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "latex_booktabs",
        TableFormat {
            line_above: LineFormat::Dynamic(latex_line_begin_tabular_booktabs),
            line_below_header: LineFormat::Static(line("\\midrule", "", "", "")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\\bottomrule\n\\end{tabular}", "", "", "")),
            header_row: RowFormat::Dynamic(latex_row),
            data_row: RowFormat::Dynamic(latex_row),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "latex_longtable",
        TableFormat {
            line_above: LineFormat::Dynamic(latex_line_begin_tabular_longtable),
            line_below_header: LineFormat::Static(line("\\hline\n\\endhead", "", "", "")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\\hline\n\\end{longtable}", "", "", "")),
            header_row: RowFormat::Dynamic(latex_row),
            data_row: RowFormat::Dynamic(latex_row),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "latex_raw",
        TableFormat {
            line_above: LineFormat::Dynamic(latex_line_begin_tabular),
            line_below_header: LineFormat::Static(line("\\hline", "", "", "")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\\hline\n\\end{tabular}", "", "", "")),
            header_row: RowFormat::Dynamic(latex_row_raw),
            data_row: RowFormat::Dynamic(latex_row_raw),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "mediawiki",
        TableFormat {
            line_above: LineFormat::Static(line(
                "{| class=\"wikitable\" style=\"text-align: left;\"",
                "",
                "",
                "\n|+ <!-- caption -->\n|-",
            )),
            line_below_header: LineFormat::Static(line("|-", "", "", "")),
            line_between_rows: LineFormat::Static(line("|-", "", "", "")),
            line_below: LineFormat::Static(line("|}", "", "", "")),
            header_row: RowFormat::Dynamic(mediawiki_header_row),
            data_row: RowFormat::Dynamic(mediawiki_data_row),
            padding: 0,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "mixed_grid",
        TableFormat {
            line_above: LineFormat::Static(line("\u{250d}", "\u{2501}", "\u{252f}", "\u{2511}")),
            line_below_header: LineFormat::Static(line(
                "\u{251d}", "\u{2501}", "\u{253f}", "\u{2525}",
            )),
            line_between_rows: LineFormat::Static(line(
                "\u{251c}", "\u{2500}", "\u{253c}", "\u{2524}",
            )),
            line_below: LineFormat::Static(line("\u{2515}", "\u{2501}", "\u{2537}", "\u{2519}")),
            header_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            data_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "mixed_outline",
        TableFormat {
            line_above: LineFormat::Static(line("\u{250d}", "\u{2501}", "\u{252f}", "\u{2511}")),
            line_below_header: LineFormat::Static(line(
                "\u{251d}", "\u{2501}", "\u{253f}", "\u{2525}",
            )),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\u{2515}", "\u{2501}", "\u{2537}", "\u{2519}")),
            header_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            data_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "moinmoin",
        TableFormat {
            line_above: LineFormat::None,
            line_below_header: LineFormat::None,
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Dynamic(moin_header_row),
            data_row: RowFormat::Dynamic(moin_data_row),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "orgtbl",
        TableFormat {
            line_above: LineFormat::None,
            line_below_header: LineFormat::Static(line("|", "-", "+", "|")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("|", "|", "|")),
            data_row: RowFormat::Static(row("|", "|", "|")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "outline",
        TableFormat {
            line_above: LineFormat::Static(line("+", "-", "+", "+")),
            line_below_header: LineFormat::Static(line("+", "=", "+", "+")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("+", "-", "+", "+")),
            header_row: RowFormat::Static(row("|", "|", "|")),
            data_row: RowFormat::Static(row("|", "|", "|")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "pipe",
        TableFormat {
            line_above: LineFormat::Dynamic(pipe_line_with_colons),
            line_below_header: LineFormat::Dynamic(pipe_line_with_colons),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("|", "|", "|")),
            data_row: RowFormat::Static(row("|", "|", "|")),
            padding: 1,
            with_header_hide: &["lineabove"],
        },
    );
    formats.insert(
        "plain",
        TableFormat {
            line_above: LineFormat::None,
            line_below_header: LineFormat::None,
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("", "  ", "")),
            data_row: RowFormat::Static(row("", "  ", "")),
            padding: 0,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "presto",
        TableFormat {
            line_above: LineFormat::None,
            line_below_header: LineFormat::Static(line("", "-", "+", "")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("", "|", "")),
            data_row: RowFormat::Static(row("", "|", "")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "pretty",
        TableFormat {
            line_above: LineFormat::Static(line("+", "-", "+", "+")),
            line_below_header: LineFormat::Static(line("+", "-", "+", "+")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("+", "-", "+", "+")),
            header_row: RowFormat::Static(row("|", "|", "|")),
            data_row: RowFormat::Static(row("|", "|", "|")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "psql",
        TableFormat {
            line_above: LineFormat::Static(line("+", "-", "+", "+")),
            line_below_header: LineFormat::Static(line("|", "-", "+", "|")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("+", "-", "+", "+")),
            header_row: RowFormat::Static(row("|", "|", "|")),
            data_row: RowFormat::Static(row("|", "|", "|")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "rounded_grid",
        TableFormat {
            line_above: LineFormat::Static(line("\u{256d}", "\u{2500}", "\u{252c}", "\u{256e}")),
            line_below_header: LineFormat::Static(line(
                "\u{251c}", "\u{2500}", "\u{253c}", "\u{2524}",
            )),
            line_between_rows: LineFormat::Static(line(
                "\u{251c}", "\u{2500}", "\u{253c}", "\u{2524}",
            )),
            line_below: LineFormat::Static(line("\u{2570}", "\u{2500}", "\u{2534}", "\u{256f}")),
            header_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            data_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "rounded_outline",
        TableFormat {
            line_above: LineFormat::Static(line("\u{256d}", "\u{2500}", "\u{252c}", "\u{256e}")),
            line_below_header: LineFormat::Static(line(
                "\u{251c}", "\u{2500}", "\u{253c}", "\u{2524}",
            )),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\u{2570}", "\u{2500}", "\u{2534}", "\u{256f}")),
            header_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            data_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "rst",
        TableFormat {
            line_above: LineFormat::Static(line("", "=", "  ", "")),
            line_below_header: LineFormat::Static(line("", "=", "  ", "")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("", "=", "  ", "")),
            header_row: RowFormat::Static(row("", "  ", "")),
            data_row: RowFormat::Static(row("", "  ", "")),
            padding: 0,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "simple",
        TableFormat {
            line_above: LineFormat::Static(line("", "-", "  ", "")),
            line_below_header: LineFormat::Static(line("", "-", "  ", "")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("", "-", "  ", "")),
            header_row: RowFormat::Static(row("", "  ", "")),
            data_row: RowFormat::Static(row("", "  ", "")),
            padding: 0,
            with_header_hide: &["lineabove", "linebelow"],
        },
    );
    formats.insert(
        "simple_grid",
        TableFormat {
            line_above: LineFormat::Static(line("\u{250c}", "\u{2500}", "\u{252c}", "\u{2510}")),
            line_below_header: LineFormat::Static(line(
                "\u{251c}", "\u{2500}", "\u{253c}", "\u{2524}",
            )),
            line_between_rows: LineFormat::Static(line(
                "\u{251c}", "\u{2500}", "\u{253c}", "\u{2524}",
            )),
            line_below: LineFormat::Static(line("\u{2514}", "\u{2500}", "\u{2534}", "\u{2518}")),
            header_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            data_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "simple_outline",
        TableFormat {
            line_above: LineFormat::Static(line("\u{250c}", "\u{2500}", "\u{252c}", "\u{2510}")),
            line_below_header: LineFormat::Static(line(
                "\u{251c}", "\u{2500}", "\u{253c}", "\u{2524}",
            )),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("\u{2514}", "\u{2500}", "\u{2534}", "\u{2518}")),
            header_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            data_row: RowFormat::Static(row("\u{2502}", "\u{2502}", "\u{2502}")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "textile",
        TableFormat {
            line_above: LineFormat::None,
            line_below_header: LineFormat::None,
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("|_. ", "|_.", "|")),
            data_row: RowFormat::Dynamic(textile_row_with_attrs),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "tsv",
        TableFormat {
            line_above: LineFormat::None,
            line_below_header: LineFormat::None,
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("", "\t", "")),
            data_row: RowFormat::Static(row("", "\t", "")),
            padding: 0,
            with_header_hide: &[],
        },
    );
    formats.insert(
        "unsafehtml",
        TableFormat {
            line_above: LineFormat::Dynamic(html_begin_table_without_header),
            line_below_header: LineFormat::Text(Cow::Borrowed("")),
            line_between_rows: LineFormat::None,
            line_below: LineFormat::Static(line("</tbody>\n</table>", "", "", "")),
            header_row: RowFormat::Dynamic(html_header_row_unsafe),
            data_row: RowFormat::Dynamic(html_data_row_unsafe),
            padding: 0,
            with_header_hide: &["lineabove"],
        },
    );
    formats.insert(
        "youtrack",
        TableFormat {
            line_above: LineFormat::None,
            line_below_header: LineFormat::None,
            line_between_rows: LineFormat::None,
            line_below: LineFormat::None,
            header_row: RowFormat::Static(row("|| ", " || ", " || ")),
            data_row: RowFormat::Static(row("| ", " | ", " |")),
            padding: 1,
            with_header_hide: &[],
        },
    );
    formats
}

static TABLE_FORMATS: Lazy<BTreeMap<&'static str, TableFormat>> = Lazy::new(build_formats);

static TABLE_FORMAT_NAMES: Lazy<Vec<&'static str>> =
    Lazy::new(|| TABLE_FORMATS.keys().copied().collect());

/// Retrieve a table format by name.
pub fn table_format(name: &str) -> Option<&'static TableFormat> {
    TABLE_FORMATS.get(name)
}

/// Return the list of available table format identifiers.
pub fn tabulate_formats() -> &'static [&'static str] {
    TABLE_FORMAT_NAMES.as_slice()
}

/// Construct a simple column-separated [`TableFormat`].
pub fn simple_separated_format<S: Into<String>>(separator: S) -> TableFormat {
    let separator: Cow<'static, str> = Cow::Owned(separator.into());
    TableFormat {
        line_above: LineFormat::None,
        line_below_header: LineFormat::None,
        line_between_rows: LineFormat::None,
        line_below: LineFormat::None,
        header_row: RowFormat::Static(DataRow {
            begin: Cow::Borrowed(""),
            separator: separator.clone(),
            end: Cow::Borrowed(""),
        }),
        data_row: RowFormat::Static(DataRow {
            begin: Cow::Borrowed(""),
            separator,
            end: Cow::Borrowed(""),
        }),
        padding: 0,
        with_header_hide: &[],
    }
}
