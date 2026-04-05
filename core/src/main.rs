use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// Define what language to target for compilation
    #[arg(short, long)]
    comp_target: Option<Vec<String>>
}

fn main() {
    println!("OH HELLO NEW WORLD");

    let _args = Args::parse();
}
