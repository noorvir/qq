use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(name = "text", default_value = "default text")]
    text: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.text);
}
