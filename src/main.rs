//! Meow provides an executable as a wrapper to the library defined in
//! `lib.rs`. This has three main functionalities: executing a string, a file,
//! and a REPL (Read–Eval–Print Loop). The CLI arguments can be seen with the
//! command `meow --help`.

use ansi_term::Colour::Red;
use anyhow::Result;
use clap::Parser;
use meow::errors::{ErrorKind, Label, Responder};
use meow::{run, run_from_file};
use std::process;

#[derive(Parser)]
#[clap(version)]
struct Args {
    /// the path to the file to execute
    #[clap(short, long)]
    file: Option<String>,

    /// the string to execute
    #[clap(short, long)]
    string: Option<String>,
}

fn main() -> Result<()> {
    use meow::errors::Label;
    use meow::lexer::token::TokenKind;
    use meow::parser::Parser;

    let mut parser = Parser::new("let x 10", "main.mw");

    parser.advance();
    let t = parser.advance();

    let label = Label::new(0, 8, "Unexpected token");
    parser.consume(TokenKind::Equal, "Expected to find token `=`", vec![label]);
    // let args = Args::parse();

    // if args.file.is_some() && args.string.is_some() {
    //     eprintln!(
    //         "{}: please input either a file or a string, not both",
    //         Red.paint("error")
    //     );
    //     process::exit(1);
    // } else if let Some(string) = args.string {
    //     run(&string)
    // } else if let Some(file) = args.file {
    //     run_from_file(&file)?;
    // } else {
    //     // add repl logic here
    //     // run(input_or_whatever)
    // }

    Ok(())
}
