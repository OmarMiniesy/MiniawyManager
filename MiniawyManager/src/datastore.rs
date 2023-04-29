pub mod datastore_functions {
    
    use std::collections::HashMap;
    use sysinfo::{System, ProcessExt, Pid, SystemExt, PidExt, CpuExt};
    use crate::ProcessInfo;
    use users::{get_user_by_uid, User, all_users, get_group_by_gid, Group, get_current_uid, get_current_username, get_current_gid, get_current_groupname, os::unix::UserExt};
    use std::fs::{File, read_dir};
    use std::io::{self, BufRead, BufReader};    
    use procfs::process::Process;
    use procfs::process::all_processes;
    use libc::{sysconf, _SC_CLK_TCK};


    pub fn hashmapFill(hashmap: &mut HashMap<u32, ProcessInfo>, system: &mut System){
        
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

            let fd_dir = format!("/proc/{}/fd", stat.pid);
            let count_files = read_dir(fd_dir.clone())
            .map(|entries| entries.count())
            .unwrap_or(0);

            hashmap.insert(stat.pid as u32, ProcessInfo {
                pid: stat.pid,
                name: stat.comm,
                ppid: stat.ppid,
                state: stat.state,
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
    }

    pub fn overall_stats(hashmap: &mut HashMap<u32, ProcessInfo>, system: &mut System){

        // MEMORY
        let mut total_memory_in_kb = system.total_memory()/1000;
        let mut used_memory_in_kb = system.used_memory()/1000;
        let mut total_memory_consumed_percentage = used_memory_in_kb as f64 / total_memory_in_kb as f64 * 100.0;
        println!("Memory: {}% ({}/{})", total_memory_consumed_percentage, used_memory_in_kb, total_memory_in_kb);

        // CPU
        system.refresh_cpu(); // Refreshing CPU information.
        let mut i = 1;
        for cpu in system.cpus() {
            println!("CPU {i} {}% ", cpu.cpu_usage());
            i += 1;
        }

        // PROCESSES
        let mut total_processes = hashmap.keys().count();
        println!("Processes: {}", total_processes);

        // THREADS
        let mut total_threads = 0;
        for (key,val) in hashmap.iter(){
            total_threads += val.threads;
        }
        println!("Threads: {}", total_threads);

        // USERS
        let file = File::open("/etc/passwd").expect("Failed to open file");
        let reader = BufReader::new(file);
        let num_users = reader.lines().count();
        println!("There are {} users on the system", num_users);
        
    }

}