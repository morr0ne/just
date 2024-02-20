use {clap::Parser, regex::Regex};

#[derive(Parser)]
struct Arguments {
  #[arg(long)]
  reference: String,
}

fn main() {
  let arguments = Arguments::parse();

  let regex = Regex::new("^refs/tags/[[:digit:]]+[.][[:digit:]]+[.][[:digit:]]+$")
    .expect("Failed to compile release regex");

  let value = if regex.is_match(&arguments.reference) {
    "release"
  } else {
    "other"
  };

  eprintln!("ref: {}", arguments.reference);
  eprintln!("value: {value}");

  println!("value={value}");
}
