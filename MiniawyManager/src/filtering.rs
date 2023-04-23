pub mod filtering_functions {
    use std::collections::HashMap;
    use crate::ProcessInfo;
    use crate::printing::printing_functions::print_details;

    //function to filter the hashmap by column
    pub fn filter(hashmap: &mut HashMap<u32, ProcessInfo>, column: &str, value: &str) {
        match column {
            "memory_usage" => {
                let value = value.parse::<i32>().unwrap();
                hashmap.retain(|_, v| v.memory_usage == value);
            },
            "name" => {
                hashmap.retain(|_, v| v.name == value);
            },
            "pid" => {
                let value = value.parse::<i32>().unwrap();
                hashmap.retain(|_, v| v.pid == value);
            },
            "cpu_time" => {
                let value = value.parse::<u64>().unwrap();
                hashmap.retain(|_, v| v.cpu_time == value);
            },
            "ppid" => {
                let value = value.parse::<i32>().unwrap();
                hashmap.retain(|_, v| v.ppid == value);
            },
            "state" => {
                hashmap.retain(|_, v| v.state == value.chars().next().unwrap());
            },
            "priority" => {
                let value = value.parse::<i64>().unwrap();
                hashmap.retain(|_, v| v.priority == value);
            },
            "user_id" => {
                let value = value.parse::<u32>().unwrap();
                hashmap.retain(|_, v| v.user_id == value);
            },
            "user_name" => {
                hashmap.retain(|_, v| v.user_name == value);
            },
            "group_id" => {
                let value = value.parse::<u32>().unwrap();
                hashmap.retain(|_, v| v.group_id == value);
            },
            "group_name" => {
                hashmap.retain(|_, v| v.group_name == value);
            },
            _ => println!("Invalid column name"),
        }
        print_details(hashmap);
    }   

}