use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="preach", version="0.1.0", about="say it again for those on stdout")]
struct PreachArgs {
    /// Vec<String> to print
    strings: Vec<String>,

    /// Omit new line
    #[arg(short = 'n', name = "omit_newline")]
    omit_newline: bool
}

/// Burl implementation of "echo".
fn main() {
    let args = PreachArgs::parse();
    let output: String = args.strings.join(" ");

    let ending = if args.omit_newline { "" } else { "\n" };
    print!("{}{}", output, ending);
}
