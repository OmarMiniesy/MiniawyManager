pub mod printing_functions {
    use std::collections::HashMap;
    use crate::sorting::sorting_functions::*;
    use crate::ProcessInfo;

    pub fn print(hashmap: &mut HashMap<u32, ProcessInfo>, printType: &str) {
        match printType {
            "R" => print_resources(hashmap),
            "D" => print_details(hashmap),
            "A" => print_all(hashmap),
            _ => println!("Invalid print type"),
        }
    }

    //prints everything
    pub fn print_all(hashmap: &mut HashMap<u32, ProcessInfo>) {

        println!(" {:<5} {:<20} {:<10} {:<15} {:<10} {:<7} {:<8} {:<10} {:<5} {:<8} {:<8} {:<10} {:<9} {:<10} {:<10}"
        , "PID", "NAME", "CPU_USAGE", "MEMORY_USAGE", "CPU_TIME", "STATE", "PPID", "PRIORITY", "NICE", "THREADS", "USER_ID", "USER_NAME", "GROUP_ID", "GROUP_NAME",
        "FILES_OPENED");
        
        for (key, value) in hashmap { 
            //check if name is more than 15 characters
            if value.name.len() > 20 || value.user_name.len() > 10 || value.group_name.len() > 10{
                if value.name.len() > 20 {
                    let mut name = value.name.clone();
                    name.truncate(17);
                    name.push_str("...");
                    value.name = name;
                }
                if value.user_name.len() > 10 {
                    let mut user_name = value.user_name.clone();
                    user_name.truncate(7);
                    user_name.push_str("...");
                    value.user_name = user_name;
                }
                if value.group_name.len() > 10 {
                    let mut group_name = value.group_name.clone();
                    group_name.truncate(7);
                    group_name.push_str("...");
                    value.group_name = group_name;
                }
            }

            println!(" {:<5} {:<20} {:<10} {:<15} {:<10} {:<7} {:<8} {:<10} {:<5} {:<8} {:<8} {:<10} {:<9} {:<10} {:<10}"
            , key, value.name, value.cpu_usage, value.memory_usage, value.cpu_time, value.state, value.ppid, value.priority, value.nice, value.threads, value.user_id, value.user_name, value.group_id, value.group_name
            ,  value.files_opened);
        }
    }

    //prints basic info (RESOURCE USAGE)
    pub fn print_resources(hashmap: &mut HashMap<u32, ProcessInfo>) {
        println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<20}  {:<15}  {:<15}  {:<15} {:<15}"
        , "PID", "NAME", "STATE", "USER_ID", "USER_NAME", "FILES_OPENED", "CPU_USAGE", "MEMORY_USAGE", "CPU_TIME");
        for (key, value) in hashmap { 
            println!(" {:<10}  {:<40}  {:<15}  {:<15}  {:<20}  {:<15}  {:<15.4}  {:<15} {:<15} "
            , key, value.name, value.state, value.user_id, value.user_name, value.files_opened, value.cpu_usage, value.memory_usage, value.cpu_time);
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