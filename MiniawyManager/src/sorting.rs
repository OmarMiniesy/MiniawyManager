pub mod sorting_functions {
    use std::collections::HashMap;
    use crate::ProcessInfo;
    use crate::printing::printing_functions::*;
    //helper function to call the desired sort function
    pub fn sort(hashmap: &mut HashMap<u32, ProcessInfo>, column: &str) {
        match column {
            "memory_usage" => sort_memory_usage(hashmap),
            "name" => sort_name(hashmap),
            "pid" => sort_pid(hashmap),
            "cpu_time" => sort_cpu_time(hashmap),
            "cpu_usage" => sort_cpu_usage(hashmap),
            "threads" => sort_threads(hashmap),
            _ => println!("Invalid column name"),
        }
    }
    //sort by memory usage
    pub fn sort_memory_usage(hashmap: &mut HashMap<u32, ProcessInfo>) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| a.1.memory_usage.cmp(&b.1.memory_usage));
        println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}"
        , "PID", "NAME", "MEMORY_USAGE", "CPU_USAGE", "CPU_TIME", "THREADS", "STATE", "USER_ID", "USER_NAME");
        for (key, value) in vec {
            print_process(hashmap, *key);
        }
    }
    //sort by name
    pub fn sort_name(hashmap: &mut HashMap<u32, ProcessInfo>) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}"
        , "PID", "NAME", "MEMORY_USAGE", "CPU_USAGE", "CPU_TIME", "THREADS", "STATE", "USER_ID", "USER_NAME");
        for (key, value) in vec {
            print_process(hashmap, *key);
        }
    }
    //sort by pid
    pub fn sort_pid(hashmap: &mut HashMap<u32, ProcessInfo>) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| a.1.pid.cmp(&b.1.pid));
        println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}"
        , "PID", "NAME", "MEMORY_USAGE", "CPU_USAGE", "CPU_TIME", "THREADS", "STATE", "USER_ID", "USER_NAME");
        for (key, value) in vec {
            print_process(hashmap, *key);
        }
    }
    //sort by descending cpu usage
    pub fn sort_cpu_usage(hashmap: &mut HashMap<u32, ProcessInfo>) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| b.1.cpu_usage.partial_cmp(&a.1.cpu_usage).unwrap());
        println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}"
        , "PID", "NAME", "MEMORY_USAGE", "CPU_USAGE", "CPU_TIME", "THREADS", "STATE", "USER_ID", "USER_NAME");
        for (key, value) in vec {
            print_process(hashmap, *key);
        }
    }
    //sort by cpu time
    pub fn sort_cpu_time(hashmap: &mut HashMap<u32, ProcessInfo>) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| b.1.cpu_time.partial_cmp(&a.1.cpu_time).unwrap());
        println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}"
        , "PID", "NAME", "MEMORY_USAGE", "CPU_USAGE", "CPU_TIME", "THREADS", "STATE", "USER_ID", "USER_NAME");
        for (key, value) in vec {
            print_process(hashmap, *key);
        }
    }

    //sort by threads
    pub fn sort_threads(hashmap: &mut HashMap<u32, ProcessInfo>) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| b.1.threads.cmp(&a.1.threads));
        println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}  {:<15}"
        , "PID", "NAME", "MEMORY_USAGE", "CPU_USAGE", "CPU_TIME", "THREADS", "STATE", "USER_ID", "USER_NAME");
        for (key, value) in vec {
            print_process(hashmap, *key);
        }
    }
}