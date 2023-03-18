use procfs::process::Process;
use procfs::process::all_processes;
use psutil::process::Process as PsutilProcess;
use users::{get_user_by_uid, User, get_group_by_gid, Group};
use std::fs;

//use std::time::{Duration, SystemTime};

fn main() {

    //let mut totalcpu :f64 = 0.0;

    for process in all_processes().unwrap() {
        let process = process.unwrap();
        let stat = process.stat().unwrap();
        let status = process.status().unwrap();
        let cputime = stat.utime + stat.stime;

        // let start_time = Duration::from_nanos(stat.starttime as u64 * 1_000_000);
        // let elapsed_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
        //     .unwrap_or_else(|_| Duration::new(0, 0));
        // let total_time = stat.utime + stat.stime + stat.cutime as u64 + stat.cstime as u64;
        // let cpu_percent = 100.0 * total_time as f64 / elapsed_time.as_secs_f64();
        // totalcpu = totalcpu + cpu_percent;

        // let elapsed_time = SystemTime::now().duration_since(start_time);
        // let total_time = stat.utime + stat.stime;
        // let cpu_percent = 100.0 * total_time as f64 / elapsed_time.as_secs_f64();

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
        println!("Start time: {}", stat.starttime);
        println!("Virtual memory size: {}", stat.vsize);
        println!("Resident set size: {}", stat.rss);
        println!("Number of threads: {}", stat.num_threads);
        println!("CPU time: {}", cputime);
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
        

        //println!("Elapsed time: {:?}", elapsed_time);
        //println!("CPU Percentage: {}%" , cpu_percent);
        println!("");
    }

    //println!("Total CPU Usage: {}%", totalcpu);
}
