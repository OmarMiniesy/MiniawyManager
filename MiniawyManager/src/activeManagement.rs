pub mod activeManagement_functions{
    use sysinfo::{System, ProcessExt, Pid, SystemExt, PidExt};
    use crate::ProcessInfo;
    use users::{get_user_by_uid, User, all_users, os::unix::UserExt, get_current_username};
    use std::collections::HashMap;

    pub fn check_permission(process_structure: &mut HashMap<u32, ProcessInfo>, pid: &u32) -> bool{
        let mut name = String::new();
        name = process_structure[pid].user_name.clone();
        if let Some(username) = get_current_username(){
            if username.to_string_lossy().to_string() == name || username == "root"{
                return true;
            }
        }
        return false;
    }


    pub fn kill_process(system: &mut System, pid: &String, process_structure: &mut HashMap<u32, ProcessInfo>){
        let x = pid.parse::<u32>().unwrap();
        let p = Pid::from_u32(x);
        if check_permission(process_structure, &x) {
            if let Some(process) = system.process(p){
                process.kill();
            }
        }else {
            println!("You don't have permission to kill this process");
        }
        
    }
//function to kill a process and all its children recursively 
    pub fn recursive_kill(system: &mut System, pid: &String, process_structure: &mut HashMap<u32, ProcessInfo>) {
        let x = pid.parse::<u32>().unwrap();
        let p = Pid::from_u32(x);
        if check_permission(process_structure, &x) {
            
            let mut children: Vec<i32> = Vec::new();
            for (_, v) in process_structure.iter() {
                if v.ppid == x as i32 {
                    children.push(v.pid);
                }
            }
            if let Some(process) = system.process(p){
                process.kill();
            }
            while let Some(child) = children.pop() {
                recursive_kill(system, &child.to_string(), process_structure);
            }
        } else {
            println!("You don't have permission to kill this process");
        }
    }
}