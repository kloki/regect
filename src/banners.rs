use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};
const HEADER: &str = r"┏┓┏┓┏┓┏┓┏╋
┛ ┗ ┗┫┗ ┗┗
";
const FOOTER: &str =
    r"^x toggle match/substitution, ^e export regex, ^o export output, ^h quick reference, ^q quit";

const HELP: &str = r"
Match                               Quantifiers                         Groups & Substitution
.              any char except \n   x*             zero or more of x    (exp)          numbered capture group
\d             digit                x+             one or more of x     (?<name>exp)   named capture group
\D             not digit            x?             zero or one of x     (?:exp)        non-capturing group
\s             whitespace           x{n,m}         at least n x and     (?flags)       set flags within current group
\S             not whitespace                      at most m x          (?flags:exp)   set flags for exp (non-capturing)
\w             word character       x{n,}          at least n x         i              case-insensitive
\W             not word character   x{n}           exactly n x          m              multi-line mode:
\n             new line                                                                ^ and $ match begin/end of line
[xyz]          matching either                                          $0             Complete match
               x, y or z (union).                                       $1             Contents of the first group
[^xyz]         matching all except                                      $foo           Contents of the group named foo
               x, y and z.
[a-z]          matching in range a-z.
^              match begin haystack
$              the end of a haystack
";

pub fn header() -> impl Widget {
    Paragraph::new(HEADER)
        .centered()
        .style(Style::default().fg(Color::Cyan))
}
pub fn footer() -> impl Widget {
    Paragraph::new(FOOTER).right_aligned()
}

pub fn help() -> impl Widget {
    Paragraph::new(HELP).block(
        Block::new()
            .border_type(BorderType::Rounded)
            .border_style(Style::default())
            .borders(Borders::ALL)
            .title("Quick Reference"),
    )
}
