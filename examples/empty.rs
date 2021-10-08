use clap::Clap;
use klask::Settings;

#[derive(Debug, Clap)]
struct Opts {
    #[clap(long)]
    opt1: Option<String>,
    #[clap(long)]
    opt2: Option<String>,
}

fn main() {
    klask::run_derived::<Opts, _>(Settings::default(), |opt| println!("{:?}", opt));
}
