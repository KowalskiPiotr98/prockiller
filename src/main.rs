mod proc;
mod args;
mod sleeper;

fn main() {
    let args = args::read_args();

    let proc = match proc::find_matching(args.proc_name.as_str()) {
        Some(p) => {
            println!("Found process {}", p.name());
            p
        }
        None => {
            eprintln!("Process not found");
            return;
        }
    };

    sleeper::sleep(args.seconds);

    if !proc.kill() {
        println!("Failed to kill process!");
    } else {
        println!("Process {} killed successfully!", args.proc_name);
    }
}
