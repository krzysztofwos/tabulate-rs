use once_cell::sync::Lazy;
use regex::Regex;
use unicode_width::UnicodeWidthChar;

static ANSI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\x1b(?:(?:\[[0-9;]*[A-Za-z])|(?:\][^\x1b]*\x1b\\))").expect("invalid ANSI regex")
});

/// Remove ANSI escape sequences from `input`.
pub fn strip_ansi(input: &str) -> String {
    ANSI_RE.replace_all(input, "").into_owned()
}

/// Compute the display width of a string, accounting for ANSI escape sequences.
pub fn visible_width(input: &str, enable_widechars: bool) -> usize {
    let stripped = strip_ansi(input);
    if enable_widechars {
        stripped
            .chars()
            .map(|ch| UnicodeWidthChar::width(ch).unwrap_or(0))
            .sum()
    } else {
        stripped.chars().count()
    }
}

/// Compute the display width of each line in a potentially multiline cell.
#[allow(dead_code)]
pub fn multiline_widths(input: &str, enable_widechars: bool) -> Vec<usize> {
    input
        .split('\n')
        .map(|line| visible_width(line, enable_widechars))
        .collect::<Vec<_>>()
}
