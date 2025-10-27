use crate::{alignment::Alignment, format::TableFormat};

/// Header specification for the table.
#[derive(Clone, Debug, Default)]
pub enum Headers {
    /// No headers.
    #[default]
    None,
    /// Use the first row of the data as a header row.
    FirstRow,
    /// Use the keys/index positions detected in the data source.
    Keys,
    /// Use explicitly provided header labels.
    Explicit(Vec<String>),
    /// Map column keys to header labels for dict-like rows.
    Mapping(Vec<(String, String)>),
}

/// Format specifier for numeric columns.
#[derive(Clone, Debug, Default)]
pub enum FormatSpec {
    /// Use the default formatting rules from `python-tabulate`.
    #[default]
    Default,
    /// Reuse the same format string for all columns.
    Fixed(String),
    /// Set the format on a per-column basis.
    PerColumn(Vec<String>),
}

/// Vertical alignment applied when rendering multi-line rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowAlignment {
    /// Align content to the top (extra blank lines at the bottom).
    Top,
    /// Align content to the centre.
    Center,
    /// Align content to the bottom (extra blank lines at the top).
    Bottom,
}

/// Header alignment spec, including the python-tabulate "same" sentinel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeaderAlignment {
    /// Explicit alignment choice.
    Align(Alignment),
    /// Reuse the column alignment (`"same"` in python-tabulate).
    SameAsColumn,
}

impl From<Alignment> for HeaderAlignment {
    fn from(value: Alignment) -> Self {
        HeaderAlignment::Align(value)
    }
}

/// Placeholder values for missing entries.
#[derive(Clone, Debug)]
pub enum MissingValues {
    /// Use the same placeholder for each column.
    Single(String),
    /// Provide per-column placeholders. Missing entries are extended by repeating the last value.
    PerColumn(Vec<String>),
}

impl Default for MissingValues {
    fn default() -> Self {
        Self::Single(String::new())
    }
}

/// Controls whether an index column is shown.
#[derive(Clone, Debug, Default)]
pub enum ShowIndex {
    /// Follow the behaviour of `python-tabulate`: only show an index for data sources that
    /// naturally provide one (e.g. pandas DataFrame).
    #[default]
    Default,
    /// Always show an index column using the implicit row number.
    Always,
    /// Never show an index column.
    Never,
    /// Show an index column with the provided labels.
    Values(Vec<String>),
}

/// Builder-style configuration for [`tabulate`](crate::tabulate).
#[derive(Clone, Debug)]
pub struct TabulateOptions {
    /// Header configuration.
    pub headers: Headers,
    /// Table format selection (either a named format or a custom specification).
    pub table_format: TableFormatChoice,
    /// Floating point number formatting.
    pub float_format: FormatSpec,
    /// Integer number formatting.
    pub int_format: FormatSpec,
    /// Alignment for numeric columns.
    pub num_align: Option<Alignment>,
    /// Alignment for string columns.
    pub str_align: Option<Alignment>,
    /// Replacement value for `None`/missing data.
    pub missing_values: MissingValues,
    /// Index column behaviour.
    pub show_index: ShowIndex,
    /// Disable automatic detection of numeric values.
    pub disable_numparse: bool,
    /// Disable numeric parsing for specific columns (0-indexed).
    pub disable_numparse_columns: Option<Vec<usize>>,
    /// Preserve the original whitespace provided in the data.
    pub preserve_whitespace: bool,
    /// Treat East Asian wide characters using double-width measurements.
    pub enable_widechars: bool,
    /// Maximum column widths (if any).
    pub max_col_widths: Option<Vec<Option<usize>>>,
    /// Maximum header column widths (if any).
    pub max_header_col_widths: Option<Vec<Option<usize>>>,
    /// Alignment override applied to every column.
    pub col_global_align: Option<Alignment>,
    /// Alignment overrides per column.
    pub col_align: Vec<Option<Alignment>>,
    /// Alignment override for header row.
    pub headers_global_align: Option<Alignment>,
    /// Alignment overrides per-header cell.
    pub headers_align: Vec<Option<HeaderAlignment>>,
    /// Alignment override applied to rows.
    pub row_align: Vec<Option<RowAlignment>>,
    /// Alignment override applied to all rows.
    pub row_global_align: Option<RowAlignment>,
    /// Control how long words are wrapped.
    pub break_long_words: bool,
    /// Control whether words can be wrapped on hyphens.
    pub break_on_hyphens: bool,
}

impl Default for TabulateOptions {
    fn default() -> Self {
        Self {
            headers: Headers::default(),
            table_format: TableFormatChoice::Name("simple".to_string()),
            float_format: FormatSpec::Default,
            int_format: FormatSpec::Default,
            num_align: None,
            str_align: None,
            missing_values: MissingValues::default(),
            show_index: ShowIndex::default(),
            disable_numparse: false,
            disable_numparse_columns: None,
            preserve_whitespace: false,
            enable_widechars: false,
            max_col_widths: None,
            max_header_col_widths: None,
            col_global_align: None,
            col_align: Vec::new(),
            headers_global_align: None,
            headers_align: Vec::new(),
            row_align: Vec::new(),
            row_global_align: None,
            break_long_words: true,
            break_on_hyphens: true,
        }
    }
}

impl TabulateOptions {
    /// Construct a new options builder with default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set explicit headers.
    pub fn headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }

    /// Set table format by name.
    pub fn table_format<S: Into<String>>(mut self, format: S) -> Self {
        self.table_format = TableFormatChoice::Name(format.into());
        self
    }

    /// Supply a custom [`TableFormat`].
    pub fn table_format_custom(mut self, format: TableFormat) -> Self {
        self.table_format = TableFormatChoice::Custom(Box::new(format));
        self
    }

    /// Set floating point format.
    pub fn float_format(mut self, format: FormatSpec) -> Self {
        self.float_format = format;
        self
    }

    /// Set integer format.
    pub fn int_format(mut self, format: FormatSpec) -> Self {
        self.int_format = format;
        self
    }

    /// Set numeric column alignment.
    pub fn num_align(mut self, align: Alignment) -> Self {
        self.num_align = Some(align);
        self
    }

    /// Override alignment for all columns.
    pub fn col_global_align(mut self, align: Alignment) -> Self {
        self.col_global_align = Some(align);
        self
    }

    /// Set string column alignment.
    pub fn str_align(mut self, align: Alignment) -> Self {
        self.str_align = Some(align);
        self
    }

    /// Set the placeholder for missing values.
    pub fn missing_value<S: Into<String>>(mut self, value: S) -> Self {
        self.missing_values = MissingValues::Single(value.into());
        self
    }

    /// Set per-column placeholders for missing values.
    pub fn missing_values<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let collected = values.into_iter().map(Into::into).collect();
        self.missing_values = MissingValues::PerColumn(collected);
        self
    }

    /// Control whether tabulate should attempt to parse numeric values.
    pub fn disable_numparse(mut self, disable: bool) -> Self {
        self.disable_numparse = disable;
        self
    }

    /// Disable numeric parsing for specific columns (0-indexed).
    pub fn disable_numparse_columns<I>(mut self, columns: I) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let collected = columns.into_iter().collect::<Vec<_>>();
        self.disable_numparse_columns = Some(collected);
        self
    }

    /// Returns true if numeric parsing is disabled for the provided column.
    pub fn is_numparse_disabled(&self, column: Option<usize>) -> bool {
        if self.disable_numparse {
            return true;
        }
        match (column, &self.disable_numparse_columns) {
            (Some(idx), Some(columns)) => columns.contains(&idx),
            _ => false,
        }
    }

    /// Control whitespace preservation.
    pub fn preserve_whitespace(mut self, preserve: bool) -> Self {
        self.preserve_whitespace = preserve;
        self
    }

    /// Enable width calculations that treat East Asian wide characters as double width.
    pub fn enable_widechars(mut self, enable: bool) -> Self {
        self.enable_widechars = enable;
        self
    }

    /// Configure how the index column is displayed.
    pub fn show_index(mut self, show_index: ShowIndex) -> Self {
        self.show_index = show_index;
        self
    }

    /// Set per-column maximum widths. Use `None` for columns without limits.
    pub fn max_col_widths(mut self, widths: Vec<Option<usize>>) -> Self {
        self.max_col_widths = Some(widths);
        self
    }

    /// Set the same maximum width for all columns.
    pub fn max_col_width(mut self, width: usize) -> Self {
        self.max_col_widths = Some(vec![Some(width)]);
        self
    }

    /// Set per-column maximum header widths. Use `None` for unlimited columns.
    pub fn max_header_col_widths(mut self, widths: Vec<Option<usize>>) -> Self {
        self.max_header_col_widths = Some(widths);
        self
    }

    /// Set the same maximum width for all header columns.
    pub fn max_header_col_width(mut self, width: usize) -> Self {
        self.max_header_col_widths = Some(vec![Some(width)]);
        self
    }

    /// Provide a mapping from column keys to display headers.
    pub fn headers_mapping<I, K, V>(mut self, mapping: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let collected = mapping
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self.headers = Headers::Mapping(collected);
        self
    }

    /// Provide per-column alignment overrides.
    pub fn col_alignments<I>(mut self, aligns: I) -> Self
    where
        I: IntoIterator<Item = Option<Alignment>>,
    {
        self.col_align = aligns.into_iter().collect();
        self
    }

    /// Override alignment for all headers.
    pub fn headers_global_align(mut self, align: Alignment) -> Self {
        self.headers_global_align = Some(align);
        self
    }

    /// Provide per-header alignment overrides.
    pub fn headers_alignments<I>(mut self, aligns: I) -> Self
    where
        I: IntoIterator<Item = Option<HeaderAlignment>>,
    {
        self.headers_align = aligns.into_iter().collect();
        self
    }

    /// Override the default alignment for all data rows.
    pub fn row_global_align(mut self, align: RowAlignment) -> Self {
        self.row_global_align = Some(align);
        self
    }

    /// Provide per-row alignment overrides.
    pub fn row_alignments<I>(mut self, aligns: I) -> Self
    where
        I: IntoIterator<Item = Option<RowAlignment>>,
    {
        self.row_align = aligns.into_iter().collect();
        self
    }

    pub(crate) fn table_format_name(&self) -> Option<&str> {
        match &self.table_format {
            TableFormatChoice::Name(name) => Some(name.as_str()),
            TableFormatChoice::Custom(_) => None,
        }
    }
}

/// Specifies whether to use a named or custom table format.
#[derive(Clone, Debug)]
pub enum TableFormatChoice {
    /// Use one of the built-in formats by name.
    Name(String),
    /// Use a bespoke [`TableFormat`] instance.
    Custom(Box<TableFormat>),
}
