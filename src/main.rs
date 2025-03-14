use clap::Parser;
use unescape::unescape;

#[derive(Parser)]
#[command(version = "1.0", author = "namnam", about = "Text echoing written in rust.")]
struct Args {
    /// The text to echo
    text: Vec<String>,
    /// Format the output [escape sequences]
    #[arg(short, long)]
    format: bool
}


fn main() {
    let args = Args::parse();
    let text = args.text.join(" ");
    // Use the unescape crate to format the text, otherwise just print out normally.
    if args.format {
        // Error handling
        if let Some(out) = unescape(&text) {
            println!("{}", out);
        } else{
            eprintln!("\x1b[1m\x1b[31mFatal.\x1b[39m Error formatting text.\x1b[0m");
        }
        return;
    }
    // Print out the text
    println!("{}", &text);
}