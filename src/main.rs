mod proc;

fn main() {
    let proc = match proc::find_matching("yes") {
        Some(p) => {
            println!("Found process {}", p.name());
            p
        }
        None => {
            eprintln!("Process not found");
            return;
        }
    };
    if !proc.kill() {
        println!("Failed to kill process!");
    }
}
