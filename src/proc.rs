pub struct Proc {
    pid: u32,
    name: String,
}

impl Proc {
    pub fn kill(&self) -> bool {
        let pid = sysinfo::Pid::from_u32(self.pid);
        match sysinfo::System::new_all().process(pid) {
            Some(p) => p.kill(),
            _ => false,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

fn new(pid: &sysinfo::Pid, process: &sysinfo::Process) -> Proc {
    Proc{pid: pid.as_u32(), name: process.name().to_string()}
}

pub fn find_matching(name: &str) -> Option<Proc> {
    let mut proc = sysinfo::System::new();
    proc.refresh_all();
    match proc.processes().iter().find(|p| p.1.name() == name) {
        Some(p) => Some(new(p.0, p.1)),
        _ => None,
    }
}