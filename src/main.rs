pub mod unescape;
use clap::Parser;
use crate::unescape::unescape;

#[derive(Parser)]
#[command(version = "1.0", author = "namnam", about = "Text echoing written in rust.")]
struct Args {
    /// The text to echo
    text: Vec<String>,
    /// Format the output [escape sequences]
    #[arg(short, long)]
    format: bool
}

/// Entry point, parses the arguments and prints out the text.
/// I cant fix the error with shells parsing \n or anything else. So use quotes instead.
fn main() {
    let args = Args::parse();
    let text = args.text.join(" ");
    // Use the unescape crate to format the text, otherwise just print out normally.
    if args.format {
        // Error handling
        if let Some(out) = unescape(&text) {
            println!("{}\x1b[0m", out); // Reset all escapes in case you would just make everything colorful.
        } else{
            eprintln!("\x1b[1m\x1b[31mFatal.\x1b[39m Error formatting text.\x1b[0m");
        }
        return;
    }
    // Print out the text
    println!("{}", &text);
}