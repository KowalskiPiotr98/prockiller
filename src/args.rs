use std::env::args;

pub struct Args {
    pub proc_name: String,
    pub seconds: u64,
}

pub fn read_args() -> Args {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        panic!("Wrong number of arguments");
    }
    let seconds: u64 = args[2].parse().expect("Seconds must be a number");
    Args{proc_name: args[1].to_string(), seconds}
}
