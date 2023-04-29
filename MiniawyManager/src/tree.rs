pub mod tree_functions {
    use std::collections::HashMap;
    use crate::ProcessInfo;


    //recursive function to print the process name and id in tree format based on ppid
    pub fn tree(hashmap: &mut HashMap<u32, ProcessInfo>, pid: i32, level: i32) {
        let mut children: Vec<i32> = Vec::new();
        for (_, v) in hashmap.iter() {
            if v.ppid == pid {
                children.push(v.pid);
            }
        }
        for child in children {
            for _ in 0..level {
                print!("|   ");
            }
           // println!("|--{}", hashmap.get(&(child as u32)).unwrap().name);
            println!("({}) {} [{}]", child, hashmap.get(&(child as u32)).unwrap().name, hashmap.get(&(child as u32)).unwrap().threads);

            tree(hashmap, child, level + 1);
        }
    }
}