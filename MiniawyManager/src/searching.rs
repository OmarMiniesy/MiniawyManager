pub mod searching_functions {
    use std::collections::HashMap;
    use crate::ProcessInfo;
    use crate::printing::printing_functions::print_resources;

    pub fn search_by_name(hashmap: &mut HashMap<u32, ProcessInfo>, name: &str) {
        hashmap.retain(|_, v| v.name == name);
        print_resources(hashmap);
    }

    pub fn search_by_pid(hashmap: &mut HashMap<u32, ProcessInfo>, pid: &str) {
        let pid = pid.parse::<i32>().unwrap();
        hashmap.retain(|_, v| v.pid == pid);
        print_resources(hashmap);
    }
}