use procfs::process::Process;
use procfs::process::all_processes;
use users::{get_user_by_uid, User, get_group_by_gid, Group};
use std::fs;
use sysinfo::{System, SystemExt, ProcessExt, NetworkExt, Pid, PidExt, ProcessStatus};
use libc::{sysconf, _SC_CLK_TCK};
use std::collections::HashMap;

mod sorting;
use crate::sorting::sorting_functions::*;

mod filtering;
use crate::filtering::filtering_functions::*;

mod printing;
use crate::printing::printing_functions::*;


pub struct ProcessInfo {
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
    let mut process_structure: HashMap<u32, ProcessInfo> = HashMap::new();
    
    //system for sysinfo
    let mut system = System::new_all();
    system.refresh_all();

    // system constants
    let mut total_memory_in_kb = system.total_memory()/1000;
    let mut used_memory_in_kb = system.used_memory()/1000;
    let clk_tck = unsafe { sysconf(_SC_CLK_TCK) } as f64;

    // //for loop for procfs
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

        process_structure.insert(stat.pid as u32, ProcessInfo {
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
            memory_usage: 0 as i32,
            network_usage: 0,
        });
    }
    
    // for loop for sysinfo
    for (pid, process) in system.processes() {
        let val:u32 = process.pid().as_u32();
        let mut mem = process.memory() as f64 / 1000.0;  // in KB
        if let Some(entry) = process_structure.get_mut(&val) {
            entry.memory_usage = mem as i32;
        }
    }

    //print_resources(&mut process_structure);
    filter(&mut process_structure, "name", "firefox");
}