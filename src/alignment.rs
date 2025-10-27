use crate::width::visible_width;

/// Text alignment options supported by the tabulator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    /// Align text to the left (pad on the right).
    Left,
    /// Align text to the right (pad on the left).
    Right,
    /// Align text to the centre.
    Center,
    /// Align text so that decimal separators line up.
    Decimal,
}

impl Alignment {
    /// Parse an alignment specifier compatible with `python-tabulate`.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "left" => Some(Self::Left),
            "right" => Some(Self::Right),
            "center" => Some(Self::Center),
            "decimal" => Some(Self::Decimal),
            _ => None,
        }
    }
}

/// Layout information for decimal alignment.
#[derive(Clone, Copy, Debug)]
pub struct DecimalLayout {
    pub integer: usize,
    pub fraction: usize,
}

/// Align `cell` so that the visible width equals `width`.
#[allow(clippy::too_many_arguments)]
pub fn align_cell(
    cell: &str,
    width: usize,
    alignment: Alignment,
    decimal_marker: char,
    decimal_layout: Option<DecimalLayout>,
    per_line: bool,
    enforce_left_alignment: bool,
    enable_widechars: bool,
) -> String {
    if per_line {
        let lines: Vec<&str> = cell.split('\n').collect();
        let aligned_lines: Vec<String> = lines
            .into_iter()
            .map(|line| {
                align_line(
                    line,
                    width,
                    alignment,
                    decimal_marker,
                    decimal_layout,
                    enable_widechars,
                )
            })
            .collect();
        aligned_lines.join("\n")
    } else {
        let mut aligned = if matches!(alignment, Alignment::Left) && !enforce_left_alignment {
            cell.to_string()
        } else {
            align_line(
                cell,
                width,
                alignment,
                decimal_marker,
                decimal_layout,
                enable_widechars,
            )
        };
        if matches!(alignment, Alignment::Left) && aligned.contains('\n') {
            let mut lines: Vec<String> = aligned.split('\n').map(|line| line.to_string()).collect();
            let last = lines.len().saturating_sub(1);
            if last > 0 {
                for line in lines.iter_mut().take(last) {
                    let trimmed = line.trim_end();
                    line.truncate(trimmed.len());
                }
            }
            if let Some(last_line) = lines.last_mut() {
                let trimmed = last_line.trim_end();
                last_line.truncate(trimmed.len());
            }
            aligned = lines.join("\n");
        }
        aligned
    }
}

fn align_line(
    line: &str,
    width: usize,
    alignment: Alignment,
    decimal_marker: char,
    decimal_layout: Option<DecimalLayout>,
    enable_widechars: bool,
) -> String {
    match alignment {
        Alignment::Left => align_left_line(line, width, enable_widechars),
        Alignment::Right => align_right_line(line, width, enable_widechars),
        Alignment::Center => align_center_line(line, width, enable_widechars),
        Alignment::Decimal => align_decimal_line(
            line,
            width,
            decimal_marker,
            decimal_layout,
            enable_widechars,
        ),
    }
}

fn align_left_line(line: &str, width: usize, enable_widechars: bool) -> String {
    let cell_width = visible_width(line, enable_widechars);
    if cell_width >= width {
        line.to_string()
    } else {
        format!("{line}{:padding$}", "", padding = width - cell_width)
    }
}

fn align_right_line(line: &str, width: usize, enable_widechars: bool) -> String {
    let cell_width = visible_width(line, enable_widechars);
    if cell_width >= width {
        line.to_string()
    } else {
        format!("{:padding$}{line}", "", padding = width - cell_width)
    }
}

fn align_center_line(line: &str, width: usize, enable_widechars: bool) -> String {
    let cell_width = visible_width(line, enable_widechars);
    if cell_width >= width {
        line.to_string()
    } else {
        let padding = width - cell_width;
        let left = padding / 2;
        let right = padding - left;
        format!(
            "{:left$}{line}{:right$}",
            "",
            "",
            left = left,
            right = right
        )
    }
}

fn align_decimal_line(
    line: &str,
    width: usize,
    decimal_marker: char,
    decimal_layout: Option<DecimalLayout>,
    enable_widechars: bool,
) -> String {
    let layout = decimal_layout.unwrap_or(DecimalLayout {
        integer: width,
        fraction: 0,
    });
    let split_pos = line.find(decimal_marker);

    let (integer_part, fractional_part) = match split_pos {
        Some(pos) => (&line[..pos], Some(&line[pos + decimal_marker.len_utf8()..])),
        None => (line, None),
    };

    let integer_width = visible_width(integer_part, enable_widechars);
    let mut result = String::new();
    if integer_width < layout.integer {
        result.push_str(&" ".repeat(layout.integer - integer_width));
    }
    result.push_str(integer_part);
    let mut fraction_width = 0usize;
    if let Some(fractional) = fractional_part {
        result.push(decimal_marker);
        result.push_str(fractional);
        fraction_width = visible_width(fractional, enable_widechars);
    }

    if layout.fraction > 0 {
        if fractional_part.is_some() {
            if fraction_width < layout.fraction {
                result.push_str(&" ".repeat(layout.fraction - fraction_width));
            }
        } else {
            result.push_str(&" ".repeat(layout.fraction));
        }
    }

    let current_width = visible_width(&result, enable_widechars);
    if current_width < width {
        let mut padded = String::with_capacity(width);
        padded.push_str(&" ".repeat(width - current_width));
        padded.push_str(&result);
        return padded;
    }

    result
}
