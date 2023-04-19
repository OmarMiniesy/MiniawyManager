pub mod sorting_functions {
    use std::collections::HashMap;
    use crate::ProcessInfo;
    
    //helper function to call the desired sort function
    pub fn sort(hashmap: &mut HashMap<u32, ProcessInfo>, column: &str) {
        match column {
            "memory_usage" => sort_memory_usage(hashmap, column),
            "name" => sort_name(hashmap, column),
            "pid" => sort_pid(hashmap, column),
            //"cpu_time" => sort_cpu_time(hashmap, column),
            "cpu_usage" => sort_cpu_usage(hashmap, column),
            _ => println!("Invalid column name"),
        }
    }
    //sort by memory usage
    pub fn sort_memory_usage(hashmap: &mut HashMap<u32, ProcessInfo>, column: &str) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| a.1.memory_usage.cmp(&b.1.memory_usage));
        for (key, value) in vec {
            println!("{}: {}", key, value.memory_usage);
            //println!("{}: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", key, value.memory_usage, value.pid, value.name, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time);
        }
    }
    //sort by name
    pub fn sort_name(hashmap: &mut HashMap<u32, ProcessInfo>, column: &str) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        for (key, value) in vec {
            println!("{}: {}", key, value.name);
            //println!("{}: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", key, value.name, value.pid, value.memory_usage, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time);
        }
    }
    //sort by pid
    pub fn sort_pid(hashmap: &mut HashMap<u32, ProcessInfo>, column: &str) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| a.1.pid.cmp(&b.1.pid));
        for (key, value) in vec {
            println!("{}: {}", key, value.pid);
            //println!("{}: {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", key, value.pid, value.memory_usage, value.name, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time);
        }
    }
    //sort by descending cpu usage
    pub fn sort_cpu_usage(hashmap: &mut HashMap<u32, ProcessInfo>, column: &str) {
        let mut vec: Vec<_> = hashmap.iter().collect();
        vec.sort_by(|a, b| b.1.cpu_usage.partial_cmp(&a.1.cpu_usage).unwrap());
        for (key, value) in vec {
            println!("{}: {:.4}%", key, value.cpu_usage);
        }
    }
    //sort by cpu time
    //pub fn sort_cpu_time(hashmap: &mut HashMap<i32, ProcessInfo>, column: &str) {
    //     let mut vec: Vec<_> = hashmap.iter().collect();
    //     vec.sort_by(|a, b| a.1.cpu_time.cmp(&b.1.cpu_time));
    //     for (key, value) in vec {
    //         println!("{}: {:.4}%", key, value.cpu_time);
    //     }
    // }

}