use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
struct Opt {
    #[cfg(not(test))]
    arg: String,
}

fn main() {
    #[cfg(not(test))]
    let opt = Opt::from_args();
    #[cfg(not(test))]
    println!("arg {}", opt.arg);
}
