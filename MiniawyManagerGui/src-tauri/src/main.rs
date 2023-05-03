
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::vec;
use std::env;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use procfs::process::Process;
use procfs::process::all_processes;
use sysinfo::{System, ProcessExt, Pid, SystemExt, PidExt, CpuExt};
use users::{get_user_by_uid, User, all_users, get_group_by_gid, Group, get_current_uid, get_current_username, get_current_gid, get_current_groupname, os::unix::UserExt};
use std::fs::{File, read_dir};
use std::io::{self, BufRead, BufReader};    
use libc::{sysconf, _SC_CLK_TCK};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn hashmapFill() -> Vec<ProcessInfo>{

    let mut hashmap: HashMap<u32, ProcessInfo> = HashMap::new();
    let mut system = System::new_all();
    system.refresh_all();
    let clk_tck = unsafe { sysconf(_SC_CLK_TCK) } as f64;
    
    // //for loop for procfs
    for process in all_processes().unwrap() {
        let process = process.unwrap();
        let stat = process.stat().unwrap();
        let status = process.status().unwrap();

        let utime_sec = stat.utime as f64 / clk_tck;
        let stime_sec = stat.stime as f64 / clk_tck;
        let starttime_sec = stat.starttime as f64 / clk_tck;
        let elapsed_sec = system.uptime() as f64 - starttime_sec;
        let usage_sec = utime_sec + stime_sec;
        let cpu_usage = 100.0 * usage_sec / elapsed_sec;

        let float_var: f64 = cpu_usage;
        let formatted_str = format!("{:.2}", float_var);
        let cpu_usage = formatted_str.parse::<f64>().unwrap();


        let fd_dir = format!("/proc/{}/fd", stat.pid);
        let count_files = read_dir(fd_dir.clone())
        .map(|entries| entries.count())
        .unwrap_or(0);

        let mut state: &str;
        //match statements for the state
        match stat.state {
            'R' => state = "Running",
            'S' => state = "Sleeping",
            'D' => state = "Waiting",
            'I' => state = "Idle",
            'Z' => state = "Zombie",
            'T' => state = "Stopped",
            _ => state = "Unknown",
        }

        hashmap.insert(stat.pid as u32, ProcessInfo {
            pid: stat.pid,
            name: stat.comm,
            ppid: stat.ppid,
            state: state.to_string(),
            priority: stat.priority,
            nice: stat.nice,
            threads: stat.num_threads,
            user_id: status.ruid,
            user_name: get_user_by_uid(status.ruid).unwrap().name().to_string_lossy().to_string(),
            group_id: status.rgid,
            group_name: get_group_by_gid(status.rgid).unwrap().name().to_string_lossy().to_string(),
            files_opened: count_files as i32,
            cpu_usage: cpu_usage,
            cpu_time: (stat.utime + stat.stime) as f64 / 100.0,
            memory_usage: 0,
            network_usage: 0,
        });
    }
    
    // for loop for sysinfo
    for (pid, process) in system.processes() {
        let val:u32 = process.pid().as_u32();
        let mut mem = process.memory() as f64 / 1000.0;  // in KB
        if let Some(entry) = hashmap.get_mut(&val) {
            entry.memory_usage = mem as i32;
        }
    }

    //convert this hashmap to a vector
    let mut vec: Vec<ProcessInfo> = hashmap.into_iter().map(|(_, v)| v).collect();
    vec.sort_by(|a, b| a.pid.cmp(&b.pid));
    vec
}

#[derive(Serialize, Deserialize)]
pub struct ProcessInfo {
    pid: i32,
    name: String,
    ppid: i32,
    state: String,
    priority: i64,
    nice: i64,
    threads: i64,
    user_id: u32,
    user_name: String,
    group_id: u32,   
    group_name: String, 
    files_opened: i32,
    cpu_usage: f64,
    cpu_time: f64, 
    memory_usage: i32,
    network_usage: i32,
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hashmapFill])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
