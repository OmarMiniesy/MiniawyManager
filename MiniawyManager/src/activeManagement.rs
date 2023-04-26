pub mod activeManagement_functions{
    use sysinfo::{System, ProcessExt, Pid, SystemExt, PidExt};

    pub fn kill_process(system: &mut System, pid: &String){
        let x = pid.parse::<u32>().unwrap();
        let p = Pid::from_u32(x);
        if let Some(process) = system.process(p){
            process.kill();
        }        
    }

}