mod args;
mod conversions;
mod output;

fn main() {
    let args = args::parse_args();
    output::output_conversions(&args);
}
