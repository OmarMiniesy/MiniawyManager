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

//merge sort by memory usage
fn merge_sort_memory_usage(hashmap: &mut HashMap<i32, ProcessInfo>, column: &str) {
    let mut vec: Vec<_> = hashmap.iter().collect();
    vec.sort_by(|a, b| a.1.memory_usage.cmp(&b.1.memory_usage));
    for (key, value) in vec {
        println!("{}: {}", key, value.memory_usage);
        //println!("{}: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", key, value.memory_usage, value.pid, value.name, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time);
    }
}
//merge sort by name
fn merge_sort_name(hashmap: &mut HashMap<i32, ProcessInfo>, column: &str) {
    let mut vec: Vec<_> = hashmap.iter().collect();
    vec.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    for (key, value) in vec {
        println!("{}: {}", key, value.name);
        //println!("{}: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", key, value.name, value.pid, value.memory_usage, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time);
    }
}
//merge sort by pid
fn merge_sort_pid(hashmap: &mut HashMap<i32, ProcessInfo>, column: &str) {
    let mut vec: Vec<_> = hashmap.iter().collect();
    vec.sort_by(|a, b| a.1.pid.cmp(&b.1.pid));
    for (key, value) in vec {
        println!("{}: {}", key, value.pid);
        //println!("{}: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", key, value.pid, value.memory_usage, value.name, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time);
    }
}
//merge sort by cpu time
// fn merge_sort_cpu_time(hashmap: &mut HashMap<i32, ProcessInfo>, column: &str) {
//     let mut vec: Vec<_> = hashmap.iter().collect();
//     vec.sort_by(|a, b| a.1.cpu_time.cmp(&b.1.cpu_time));
//     for (key, value) in vec {
//         println!("{}: {:.4}%", key, value.cpu_time);
//     }
// }

//helper function to call the desired sort function
fn sort(hashmap: &mut HashMap<i32, ProcessInfo>, column: &str) {
    match column {
        "memory_usage" => merge_sort_memory_usage(hashmap, column),
        "name" => merge_sort_name(hashmap, column),
        "pid" => merge_sort_pid(hashmap, column),
        //"cpu_time" => merge_sort_cpu_time(hashmap, column),
        _ => println!("Invalid column name"),
    }
}
//function to filter the hashmap by column
fn filter(hashmap: &mut HashMap<i32, ProcessInfo>, column: &str, value: &str) {
    match column {
        "memory_usage" => {
            let value = value.parse::<i32>().unwrap();
            hashmap.retain(|_, v| v.memory_usage == value);
        },
        "name" => {
            hashmap.retain(|_, v| v.name == value);
        },
        "pid" => {
            let value = value.parse::<i32>().unwrap();
            hashmap.retain(|_, v| v.pid == value);
        },
        "cpu_time" => {
            let value = value.parse::<u64>().unwrap();
            hashmap.retain(|_, v| v.cpu_time == value);
        },
        "ppid" => {
            let value = value.parse::<i32>().unwrap();
            hashmap.retain(|_, v| v.ppid == value);
        },
        "state" => {
            hashmap.retain(|_, v| v.state == value.chars().next().unwrap());
        },
        "priority" => {
            let value = value.parse::<i64>().unwrap();
            hashmap.retain(|_, v| v.priority == value);
        },
        "user_id" => {
            let value = value.parse::<u32>().unwrap();
            hashmap.retain(|_, v| v.user_id == value);
        },
        "user_name" => {
            hashmap.retain(|_, v| v.user_name == value);
        },
        "group_id" => {
            let value = value.parse::<u32>().unwrap();
            hashmap.retain(|_, v| v.group_id == value);
        },
        "group_name" => {
            hashmap.retain(|_, v| v.group_name == value);
        },
        _ => println!("Invalid column name"),
    }
    //print the filtered hashmap
    for (key, value) in hashmap {
        println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", value.pid, value.memory_usage, value.name, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time);
}
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
            memory_usage: used_memory_in_kb as i32,
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

    // // print table
    // println!("PID \t NAME \t PPID \t STATE \t PRIORITY \t NICE \t NUM_THREADS \t USER_ID \t USER_NAME \t GROUP_ID \t GROUP_NAME \t FILES_OPENED \t CPU_USAGE \t CPU_TIME \t MEMORY_USAGE \t NETWORK_USAGE");
    // for (key,value) in process_structure.iter(){
    //     println!("{} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {} \t {:.4} \t {} \t {} \t {} \t  ", key, value.name, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time, value.memory_usage, value.network_usage);
    // }
    //sort(&mut process_structure, "memory_usage");
    filter(&mut process_structure, "user_name", "colord");

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