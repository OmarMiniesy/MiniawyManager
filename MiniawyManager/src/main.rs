use procfs::process::Process;
use procfs::process::all_processes;
use sysinfo::{System, SystemExt};
use std::collections::HashMap;
use std::env;

mod sorting;
use crate::sorting::sorting_functions::*;
mod filtering;
use crate::filtering::filtering_functions::*;
mod printing;
use crate::printing::printing_functions::*;
mod searching;
use crate::searching::searching_functions::*;
mod flags;
use crate::flags::flag_functions::call_function_by_flag;
mod tree;
use crate::tree::tree_functions::tree;
mod activeManagement;
use crate::activeManagement::activeManagement_functions::*;
mod datastore;
use crate::datastore::datastore_functions::*;

pub struct ProcessInfo {
    pid: i32,
    name: String,
    ppid: i32,
    state: char,
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
    
    // data structure
    let mut process_structure: HashMap<u32, ProcessInfo> = HashMap::new();
    //system for sysinfo
    let mut system = System::new_all();
    system.refresh_all();

    // system constants
    let mut total_memory_in_kb = system.total_memory()/1000;
    let mut used_memory_in_kb = system.used_memory()/1000;

    hashmapFill(&mut process_structure, &mut system);

    // total used memory percentage
    // total cpu usage percentage
    // total number of processes
    // total number of threads
    // total number of users

    let args: Vec<String> = env::args().skip(1).collect();
    call_function_by_flag(&mut process_structure, args, &mut system);

}