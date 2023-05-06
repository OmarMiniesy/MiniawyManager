
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

#[derive(Serialize, Deserialize)]
pub struct cpuStruct {
    pid: i32,
    name: String,
    cpu_usage: f64,
}

#[derive(Serialize, Deserialize)]
pub struct memStruct {
    pid: u32,
    name: String,
    memory_usage: i32,
}

#[tauri::command]
fn getSortedCpuUsage() -> Vec<cpuStruct> {
    let mut processVec: Vec<cpuStruct> = Vec::new();
    let mut system = System::new_all();
    system.refresh_all();
    let clk_tck = unsafe { sysconf(_SC_CLK_TCK) } as f64;
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

        let float_var: f64 = cpu_usage / system.cpus().len() as f64;
        let formatted_str = format!("{:.2}", float_var);
        let cpu_usage = formatted_str.parse::<f64>().unwrap();

        processVec.push(cpuStruct {
            pid: stat.pid,
            name: stat.comm,
            cpu_usage: cpu_usage,
        });
    }
    processVec.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    //take only 10 elements
    processVec.truncate(10);
    processVec
}

#[tauri::command]
fn getSortedMemUsage() -> Vec<memStruct>{
    //loop over sysinfo and get the memory usage per process
    let mut processVec: Vec<memStruct> = Vec::new();
    let mut system = System::new_all();
    //total memory
    let total_mem = system.total_memory() as f64 / 1000.0 / 1000.0;
    system.refresh_all();
    for (pid, process) in system.processes() {
        let mut mem = process.memory() as f64 / 1000.0;  // in KB
        mem = mem / 1000.0; // in MB
        mem = mem / total_mem * 100.0; // in %
        processVec.push(memStruct {
            pid: process.pid().as_u32(),
            name: process.name().to_string(),
            memory_usage: mem as i32,
        });
    }
    processVec.sort_by(|a, b| b.memory_usage.partial_cmp(&a.memory_usage).unwrap());
    //take only 10 elements
    processVec.truncate(10);
    processVec
}

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
            ppid: (stat.ppid as u32).to_string(),
            state: state.to_string(),
            priority: (stat.priority as i32).to_string(),
            nice: (stat.nice as i32).to_string(),
            threads: (stat.num_threads as i32).to_string(),
            user_id: (status.ruid as u32).to_string(),
            user_name: get_user_by_uid(status.ruid).unwrap().name().to_string_lossy().to_string(),
            group_id: (status.rgid as u32).to_string(),
            group_name: get_group_by_gid(status.rgid).unwrap().name().to_string_lossy().to_string(),
            files_opened: (count_files as i32).to_string(),
            cpu_usage: (cpu_usage as f64).to_string(),
            cpu_time: ((stat.utime + stat.stime) as f64 / 100.0).to_string(),
            memory_usage: (0 as i32).to_string(),
            network_usage: (0 as i32).to_string(),
        });
        
    }
    
    //for loop for sysinfo
    for (pid, process) in system.processes() {
        let val:u32 = process.pid().as_u32();
        let mut mem = process.memory() as f64 / 1000.0;  // in KB
        if let Some(entry) = hashmap.get_mut(&val) {
            entry.memory_usage = (mem as i32).to_string();
        }
    }

    //convert this hashmap to a vector
    let mut vec: Vec<ProcessInfo> = hashmap.into_iter().map(|(_, v)| v).collect();
    vec.sort_by(|a, b| a.pid.cmp(&b.pid));
    vec
}

#[tauri::command]
fn getCPU() -> Vec<f32> {
    let mut system = System::new_all();
    system.refresh_all();
    let mut cpu_usage: Vec<f32> = Vec::new();
    for x in (1..3){
        system.refresh_cpu(); // Refreshing CPU information.
        if x == 2{
            for cpu in system.cpus() {
                cpu_usage.push(cpu.cpu_usage());
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    cpu_usage
}

#[tauri::command]
fn getMemory() -> f64 {
    let mut system = System::new_all();
    system.refresh_all();
    let mut total_memory_in_kb = system.total_memory()/1000;
    let mut used_memory_in_kb = system.used_memory()/1000;
    let mut total_memory_consumed_percentage = used_memory_in_kb as f64 / total_memory_in_kb as f64 * 100.0;
    total_memory_consumed_percentage
}

#[tauri::command]
fn killProc(pid: u32) -> bool{
    let mut system = System::new_all();
    system.refresh_all();
    let p = Pid::from_u32(pid);

    // get current process user
    let process_user = getProcessFromTable(pid as i32);

    // check if current user is root or process user
    if let Some(username) = get_current_username(){
        if username.to_string_lossy().to_string() == "root" || username.to_string_lossy().to_string() == process_user {
            if let Some(process) = system.process(p) {
                process.kill();
                return true;
            }
        }
    }
    
    
    return false;
}

fn getProcessFromTable(pid: i32) -> String{

    let mut username = String::new();

    for process in all_processes().unwrap() {
        let process = process.unwrap();
        let stat = process.stat().unwrap();
        let status = process.status().unwrap();
        if(stat.pid == pid){
            username = get_user_by_uid(status.ruid).unwrap().name().to_string_lossy().to_string();
            return username;
        }
    }
    return username;
}


#[derive(Serialize, Deserialize)]
pub struct ProcessInfo {
    pid: i32,
    name: String,
    ppid: String,
    state: String,
    priority: String,
    nice: String,
    threads: String,
    user_id: String,
    user_name: String,
    group_id: String,   
    group_name: String, 
    files_opened: String,
    cpu_usage: String,
    cpu_time: String, 
    memory_usage: String,
    network_usage: String,
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hashmapFill, getCPU, getMemory, getSortedCpuUsage, getSortedMemUsage, killProc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
