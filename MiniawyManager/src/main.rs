use procfs::process::Process;
use procfs::process::all_processes;
use users::{get_user_by_uid, User, get_group_by_gid, Group};
use std::fs;
use sysinfo::{System, SystemExt, ProcessExt, NetworkExt};

fn main() {

    // let mut system = System::new_all();
    // system.refresh_all();

    for process in all_processes().unwrap() {
        let process = process.unwrap();
        let stat = process.stat().unwrap();
        let status = process.status().unwrap();

        let cputime = stat.utime + stat.stime;

        let starttime = stat.starttime;
        let cpu_usage = 100.0 * cputime as f64/starttime as f64;

        // println!("{:?}", status);
        // println!("");
        // println!("{:?}", stat);
        // println!("");

        println!("Name: {}", stat.comm);
        println!("PID: {}", stat.pid);
        println!("Parent PID: {}", stat.ppid);
        println!("State: {}", stat.state);
        println!("Priority: {}", stat.priority);
        println!("Nice value: {}", stat.nice);
        println!("Virtual memory size: {}", stat.vsize);
        println!("Resident set size: {}", stat.rss);
        println!("Number of threads: {}", stat.num_threads);
        println!("CPU time: {}", cputime);
        println!("CPU usage: {:.4}%", cpu_usage);
        println!("User ID: {}", status.ruid);
        
        match get_user_by_uid(status.ruid) {
            Some(user) => println!("User name: {}", user.name().to_string_lossy()),
            None => println!("User not found for ruid {}", status.ruid),
        }
        println!("Group ID: {}", status.rgid);
        match get_group_by_gid(status.rgid) {
            Some(group) => println!("Group name: {}", group.name().to_string_lossy()),
            None => println!("Group not found for rgid {}", status.rgid),
        }
        println!("");
    }

}


// use sysinfo::{Pid, NetworkExt, NetworksExt,ProcessExt, System, SystemExt};

// fn main() {
//     // Create a new system object
//     let mut system = System::new_all();

//     // Refresh all system information
//     system.refresh_all();

//     for (pid, process) in system.processes() {

//         println!("Process name: {}", process.name());
//         println!("Process ID: {}", pid);
        
//         println!("Process CPU usage: {:.4}%", process.cpu_usage());
//         println!("Process memory usage: {} KB", process.memory());
    
//         let status = process.status();
//         match(status) {
//             sysinfo::ProcessStatus::Run => println!("Process is running"),
//             sysinfo::ProcessStatus::Sleep => println!("Process is sleeping"),
//             sysinfo::ProcessStatus::Stop => println!("Process is stopped"),
//             sysinfo::ProcessStatus::Zombie => println!("Process is zombie"),
//             sysinfo::ProcessStatus::Idle => println!("Process is idle"),
//             other => println!("Process is in unknown state: {:?}", other),
//         }

//         println!("\n");
//     }


// }


