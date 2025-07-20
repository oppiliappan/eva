use std::borrow::Cow::{self, Owned};
use std::path::PathBuf;

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::{Builder, ColorMode, CompletionType, EditMode};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::history::FileHistory;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

use directories::ProjectDirs;

use regex::Regex;

use eva::error::CalcError;
use eva::eval_expr;
use eva::lex::{FunctionContext, CONSTANTS, FUNCTIONS};

pub struct RLHelper {
    completer: FilenameCompleter,
    highlighter: LineHighlighter,
    hinter: HistoryHinter,
}

struct LineHighlighter {
    ctx: FunctionContext,
    fix: usize,
}
impl Highlighter for LineHighlighter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("\x1b[90m{}\x1b[0m", hint))
    }
    fn highlight<'l>(&self, line: &'l str, _: usize) -> Cow<'l, str> {
        use std::fs::OpenOptions;
        use std::io::{BufRead, BufReader};

        let eva_dirs = ProjectDirs::from("com", "NerdyPepper", "eva").unwrap();
        let eva_data_dir = eva_dirs.data_dir();
        let mut previous_ans_path = PathBuf::from(eva_data_dir);
        previous_ans_path.push("previous_ans.txt");

        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&previous_ans_path)
            .unwrap();

        let rdr = BufReader::new(file);
        let lines = rdr.lines().map(|l| l.unwrap());
        let prev_ans = match lines.last() {
            Some(val) => val.parse::<f64>().ok(),
            None => None,
        };
        let op = eval_expr(&self.ctx, self.fix, line, prev_ans);
        match op {
            Ok(_) => {
                let constants = CONSTANTS.keys();
                let functions = FUNCTIONS.keys();
                let ops = Regex::new(r"(?P<o>[\+-/\*%\^!])").unwrap();
                let mut coloured: String = ops.replace_all(line, "\x1b[35m$o\x1b[0m").into();

                for c in constants {
                    // This regex consists of the following pieces:
                    // * the constant (`o`) to highlight (to be substituted as `{}` via `format!`),
                    //   e.g. `e`  or `pi`.
                    // * (optionally) an ANSI escape-code (`\x1b\[35m`) that is used to highlight
                    //   a binary operator (e.g. `+`/`-`/...). With this one it is ensured that
                    //   binary operators are always correctly detected after a constant
                    //   (see the next bullet-point for why that's needed).
                    // * the following operator (e.g. `+`/`-`/...), a space or the end
                    //   of the expression (to highlight e.g. `1+e` correctly). This is
                    //   required to distinguish a constant in an expression from a function-call,
                    //   e.g. `e+1` from `exp(1)`, without this matching logic, the `e` from
                    //   `exp` would be improperly interpreted as constant.
                    //
                    // To make sure none of existing highlighting (i.e. highlighting
                    // of binary operators that happens before) breaks, the escape-codes & operator
                    // (called `r`) are appended after the highlighted constant.
                    let re = Regex::new(
                        format!("(?P<o>{})(?P<r>(\x1b\\[35m)?([\\+-/\\*%\\^! ]|$))", c).as_str(),
                    )
                    .unwrap();
                    coloured = re.replace_all(&coloured, "\x1b[33m$o\x1b[0m$r").into();
                }
                for f in functions {
                    let re = Regex::new(format!("(?P<o>{})(?P<r>(\\(|$))", f).as_str()).unwrap();
                    coloured = re.replace_all(&coloured, "\x1b[34m$o\x1b[0m$r").into();
                }
                Owned(coloured)
            }
            Err(CalcError::Help) => Owned(line.replace("help", "\x1b[36mhelp\x1b[0m")),
            Err(_) => Owned(format!("\x1b[31m{}\x1b[0m", line)),
        }
    }
}

impl Highlighter for RLHelper {
    fn highlight_char(&self, _: &str, _: usize, _: CmdKind) -> bool {
        true
    }
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        self.highlighter.highlight_hint(hint)
    }
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }
}

impl Completer for RLHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for RLHelper {
    type Hint = String;
    fn hint(&self, line: &str, a: usize, b: &Context) -> Option<Self::Hint> {
        self.hinter.hint(line, a, b)
    }
}

impl Validator for RLHelper {}

impl Helper for RLHelper {}

pub fn create_readline(ctx: FunctionContext, fix: usize) -> Editor<RLHelper, FileHistory> {
    let config_builder = Builder::new();
    let config = config_builder
        .color_mode(ColorMode::Enabled)
        .edit_mode(EditMode::Emacs)
        .history_ignore_space(true)
        .completion_type(CompletionType::Circular)
        .max_history_size(1000).expect("REASON")
        .build();
    let mut rl = Editor::with_config(config).unwrap();
    let h = RLHelper {
        completer: FilenameCompleter::new(),
        highlighter: LineHighlighter { ctx, fix },
        hinter: HistoryHinter {},
    };
    rl.set_helper(Some(h));
    rl
}
