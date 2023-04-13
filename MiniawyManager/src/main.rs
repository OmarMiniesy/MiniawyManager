use procfs::process::Process;
use procfs::process::all_processes;
use users::{get_user_by_uid, User, get_group_by_gid, Group};
use std::fs;
use sysinfo::{System, SystemExt, ProcessExt, NetworkExt, Pid};
use libc::{c_long, sysconf, _SC_CLK_TCK};

// missing information
// cpu usage in percent
// network usage in bytes

// data structure. hashmap of key: id - value: (information)
// print table, with rows and columns   -T
// sort table, by column                -S <column>
// search table by name/pid             -F <name/pid>
// filter table by column               -C <column> <value>   // cpu usage, memory usage, user, group, state
// filter and sort table                -C <column> <value> -S <column>

// kill process
// change priority  // only go down

// tree structure

// graphs

fn main() {

    let mut system = System::new_all();
    system.refresh_all();

    let mut total_memory_in_kb = system.total_memory()/1000;

    let clk_tck = unsafe { sysconf(_SC_CLK_TCK) } as f64;

    let mut var:f64 = 0.0;
    //for loop for procfs
    for process in all_processes().unwrap() {
        let process = process.unwrap();
        let stat = process.stat().unwrap();
        let status = process.status().unwrap();
        
        let utime = stat.utime as f64 / clk_tck;
        let stime = stat.stime as f64 / clk_tck;
        let starttime = stat.starttime as f64 / clk_tck;
        let elapsed = system.uptime() as f64 - starttime;
        let process_usage = utime + stime;
        let cpu_usage = process_usage * 100.0 / elapsed;
        var += cpu_usage;

        println!("Name: {}", stat.comm);
        println!("PID: {}", stat.pid);
        println!("Parent PID: {}", stat.ppid);
        println!("State: {}", stat.state);
        println!("Priority: {}", stat.priority);
        println!("Nice value: {}", stat.nice);
        println!("Number of threads: {}", stat.num_threads);
        println!("User ID: {}", status.ruid);
        println!("CPU usage: {:.4}%", cpu_usage);
        match get_user_by_uid(status.ruid) {
            Some(user) => println!("User name: {}", user.name().to_string_lossy()),
            None => println!("User not found for ruid {}", status.ruid),
        }
        println!("Group ID: {}", status.rgid);
        match get_group_by_gid(status.rgid) {
            Some(group) => println!("Group name: {}", group.name().to_string_lossy()),
            None => println!("Group not found for rgid {}", status.rgid),
        }

        let fd_dir = format!("/proc/{}/fd", stat.pid);
        let count = fs::read_dir(fd_dir.clone())
        .map(|entries| entries.count())
        .unwrap_or(0);
            println!("Files opened: {}", count);

        println!("");

    }

    println!("Total CPU usage: {:.4}%", var);

    //for loop for sysinfo
    // for (pid, process) in system.processes() {
    //     //if(process.pid() == 1.into()) {
    //         println!("Process ID: {}", pid);
    //         println!("Process memory usage (RSS): {} KB", process.memory() / 1000);   //memory in kB
    //         println!("Process CPU usage: {:.4}%", process.cpu_usage());
    //         //println!("Process network usage: {} KB", process.network_usage().0 / 1000);
    //         println!("");
    //     //}
    // }

    let mut used_memory_in_kb = system.used_memory()/1000;
    println!("Used memory: {} kB", used_memory_in_kb);

}