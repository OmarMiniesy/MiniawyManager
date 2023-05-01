pub mod printing_functions {
    use std::collections::HashMap;
    use crate::ProcessInfo;

    pub fn print(hashmap: &mut HashMap<u32, ProcessInfo>, printType: &str) {
        match printType {
            "R" => print_resources(hashmap),
            "D" => print_details(hashmap),
            //"A" => print_all(hashmap),
            _ => println!("Invalid print type"),
        }
    }

    //prints everything
    //pub fn print_all(hashmap: &mut HashMap<i32, ProcessInfo>) {
    //     println!("PID \t NAME \t\t\t PPID \t STATE \t PRIORITY \t NICE \t NUM_THREADS \t USER_ID \t USER_NAME \t\t\t GROUP_ID \t GROUP_NAME \t FILES_OPENED \t CPU_USAGE \t CPU_TIME \t MEMORY_USAGE \t NETWORK_USAGE");
    //     for (key, value) in hashmap { 
    //         println!("{} \t {} \t\t\t {} \t {} \t {} \t {} \t {} \t {} \t {} \t\t\t {} \t {} \t {} \t {:.4} \t {} \t {} \t {} \t  ", key, value.name, value.ppid, value.state, value.priority, value.nice, value.num_threads, value.user_id, value.user_name, value.group_id, value.group_name, value.files_opened, value.cpu_usage, value.cpu_time, value.memory_usage, value.network_usage);
    //     }
    // }

    //prints basic info (RESOURCE USAGE)
    pub fn print_resources(hashmap: &mut HashMap<u32, ProcessInfo>) {
        println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<20}  {:<15}  {:<15}  {:<15}  {:<15} "
        , "PID", "NAME", "STATE", "USER_ID", "USER_NAME", "FILES_OPENED", "CPU_USAGE", "MEMORY_USAGE", "NETWORK_USAGE");
        for (key, value) in hashmap { 
            println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<20}  {:<15}  {:<15.4}  {:<15} {:<15}"
            , key, value.name, value.state, value.user_id, value.user_name, value.files_opened, value.cpu_usage, value.memory_usage, value.network_usage);
        }
    
    }
    //prints other details (PROCESS INFO)
    pub fn print_details(hashmap: &mut HashMap<u32, ProcessInfo>){
        println!(" {:<10}  {:<20}  {:<15}  {:<15}  {:<15}  {:<15}  {:<10}  {:<15}  {:<15} {:<12}  {:<10} "
        , "PID", "NAME", "STATE", "PPID", "PRIORITY", "NICE", "THREADS", "USER_ID", "USER_NAME", "GROUP_ID", "GROUP_NAME");
        for (key, value) in hashmap { 
            println!(" {:<10}  {:<20}  {:<15}  {:<15}  {:<15}  {:<15}  {:<10}  {:<15}  {:<15} {:<12}  {:<10}"
            , key, value.name, value.state, value.ppid, value.priority, value.nice, value.threads, value.user_id, value.user_name,
             value.group_id, value.group_name);
        }

    }

    pub fn print_process(hashmap: & HashMap<u32, ProcessInfo>, pid: u32) {
        for (key, value) in hashmap { 
            if *key == pid {
                println!(" {:<10}  {:<40}  {:<15.4}  {:<15.4}  {:<15.2}  {:<15}  {:<15}  {:<15}  {:<15} ", 
                key, value.name, value.memory_usage, value.cpu_usage, value.cpu_time, value.threads, value.state, value.user_id, value.user_name);
            }
        }
    }

}