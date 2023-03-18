use procfs::process::Process;
use procfs::process::all_processes;

fn main() {

    for process in all_processes().unwrap() {
        let process = process.unwrap();
        let stat = process.stat().unwrap();
        let status = process.status().unwrap();
        println!("{:?}", status);
        // println!("Name: {}", stat.comm);
        // println!("PID: {}", stat.pid);
        // println!("Parent PID: {}", stat.ppid);
        // println!("State: {}", stat.state);
        // println!("Priority: {}", stat.priority);
        // println!("Nice value: {}", stat.nice);
        // println!("Start time: {}", stat.starttime);
        // println!("Virtual memory size: {}", stat.vsize);
        // println!("Resident set size: {}", stat.rss);
        // println!("Number of threads: {}", stat.num_threads);
        // println!("CPU time: {}s", stat.utime + stat.stime);
        // //user of a process
        // // println!("User: {}", stat.uid);
        // println!("");
    }
}
