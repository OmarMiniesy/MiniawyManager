use procfs::process::Process;
use procfs::process::all_processes;
use users::{get_user_by_uid, User, get_group_by_gid, Group};
use std::fs;
use sysinfo::{System, SystemExt, ProcessExt, NetworkExt, Pid};
use libc::{sysconf, _SC_CLK_TCK};
use std::collections::HashMap;

// missing information
// cpu usage in percent
// network usage in bytes

// print table, with rows and columns   -T
// sort table, by column                -S <column>
// search table by name/pid             -F <name/pid>
// filter table by column               -C <column> <value>   // cpu usage, memory usage, user, group, state
// filter and sort table                -C <column> <value> -S <column>

// kill process
// change priority  // only go down

// tree structure

// graphs

struct ProcessInfo {
    pid: i32,
    name: String,
    ppid: i32,      // print tree
    state: char,
    priority: i64,
    nice: i64,
    num_threads: i64, // print tree
    user_id: u32,
    user_name: String,
    group_id: u32,   
    group_name: String,   // no need
    files_opened: i32,
    cpu_usage: f64,
    cpu_time: u64,  // print in ticks 
    memory_usage: i32,
    network_usage: i32,  // we dont have
}


fn main() {
    
    // data structure
    let mut process_structure: HashMap<i32, ProcessInfo> = HashMap::new();
    
    //system for sysinfo
    let mut system = System::new_all();
    system.refresh_all();


    // system constants
    let mut total_memory_in_kb = system.total_memory()/1000;
    let mut used_memory_in_kb = system.used_memory()/1000;
    let clk_tck = unsafe { sysconf(_SC_CLK_TCK) } as f64;
    
    
    //for loop for procfs
    for process in all_processes().unwrap() {
        let process = process.unwrap();
        let stat = process.stat().unwrap();
        let status = process.status().unwrap();

        let utime = stat.utime; 
        let stime = stat.stime; 
        let starttime = stat.starttime as f64;
        let elapsed = system.uptime() as f64 * clk_tck as f64 - starttime;
        let process_usage = utime + stime;
        let cpu_usage = process_usage as f64 * 100 as f64 / elapsed;


        let fd_dir = format!("/proc/{}/fd", stat.pid);
        let count_files = fs::read_dir(fd_dir.clone())
        .map(|entries| entries.count())
        .unwrap_or(0);

        process_structure.insert(stat.pid, ProcessInfo {
            pid: stat.pid,
            name: stat.comm,
            ppid: stat.ppid,
            state: stat.state,
            priority: stat.priority,
            nice: stat.nice,
            num_threads: stat.num_threads,
            user_id: status.ruid,
            user_name: get_user_by_uid(status.ruid).unwrap().name().to_string_lossy().to_string(),
            group_id: status.rgid,
            group_name: get_group_by_gid(status.rgid).unwrap().name().to_string_lossy().to_string(),
            files_opened: count_files as i32,
            cpu_usage: cpu_usage,
            cpu_time: stat.utime + stat.stime,
            memory_usage: 0,
            network_usage: 0,
        });
    
    }

    // for (key,value) in process_structure.iter(){
    //     println!("Process ID: {}", key);
    //     println!("Process name: {}", value.name);
    //     println!("Process parent ID: {}", value.ppid);
    //     println!("Process state: {}", value.state);
    //     println!("Process priority: {}", value.priority);
    //     println!("Process nice: {}", value.nice);
    //     println!("Process number of threads: {}", value.num_threads);
    //     println!("Process user ID: {}", value.user_id);
    //     println!("Process user name: {}", value.user_name);
    //     println!("Process group ID: {}", value.group_id);
    //     println!("Process group name: {}", value.group_name);
    //     println!("Process files opened: {}", value.files_opened);
    //     println!("Process CPU usage: {:.4}%", value.cpu_usage);
    //     println!("Process CPU time: {} ticks", value.cpu_time);
    //     println!("Process memory usage (RSS): {} KB", value.memory_usage);
    //     println!("Process network usage: {} KB", value.network_usage);
    //     println!("");
    // }

    println!("PID \t NAME \t PPID \t STATE \t PRIORITY \t NICE \t NUM_THREADS \t USER_ID \t USER_NAME \t GROUP_ID \t GROUP_NAME \t FILES_OPENED \t CPU_USAGE \t CPU_TIME \t MEMORY_USAGE \t NETWORK_USAGE");
    for (key,value) in process_structure.iter(){
        println!("{} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {:.4} \t {} \t {} \t {} \t  ", key, value.name, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time, value.memory_usage, value.network_usage);
    }

    // //for loop for sysinfo
    // for (pid, process) in system.processes() {
    //     //if(process.pid() == 1.into()) {
    //         println!("Process ID: {}", pid);
    //         println!("Process memory usage (RSS): {} KB", process.memory() / 1000);   //memory in kB
    //         //println!("Process CPU usage: {:.4}%", process.cpu_usage());
    //         //println!("Process network usage: {} KB", process.network_usage().0 / 1000);
    //         println!("");
    //     //}
    // }

}