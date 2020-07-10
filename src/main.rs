#![feature(external_doc)]

/// a  
/// b  
/// c
#[doc(include = "../about.md")]
#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short)]
    pub x: bool,
}

fn main() {
    use structopt::StructOpt;
    Args::from_args();
}
