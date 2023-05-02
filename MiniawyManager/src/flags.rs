pub mod flag_functions {
    use std::collections::HashMap;
    use crate::ProcessInfo;
    use sysinfo::System;
    use crate::sorting::sorting_functions::sort;
    use crate::filtering::filtering_functions::filter;
    use crate::printing::printing_functions::*;
    use crate::searching::searching_functions::{search_by_name, search_by_pid};
    use crate::tree::tree_functions::tree;
    use crate::activeManagement::activeManagement_functions::*;
    use crate::datastore::datastore_functions::*;

    pub fn call_function_by_flag(process_structure: &mut HashMap<u32, ProcessInfo>, args: Vec<String>, system: &mut System){

        if args.len() == 0 {
            println!("No arguments provided");
            return;
        }
        else if args[0] == "-T" {
            //call the tree function
            tree(process_structure, 0, 0);
            return;
        }
        else if args[0] == "-S" {
            //call the sort function with args[1] as the sorting column
            sort(process_structure, &args[1]);
            return;
        }
        else if args[0] == "-F" {
            //call the filter function with args[1] as the filter column and args[2] as the filter value
            filter(process_structure, &args[1], &args[2], 1);
            return;
        }
        else if args[0] == "-FS"{
            //call the filter function with args[1] as the filter column and args[2] as the filter value and args[3] as the sorting column
            filter(process_structure, &args[1], &args[2], 0);
            sort(process_structure, &args[3]);
        }
        else if args[0] == "-P" {
            //call the print function with args[1] as the different print function
            print(process_structure, &args[1]);
            return;
        }
        else if args[0] == "-A" {
            //call the search function with args[1] as the pid
            search_by_pid(process_structure, &args[1]);
            return;
        }
        else if args[0] == "-N" {
            //call the search function with args[1] as the name
            search_by_name(process_structure, &args[1]);
            return;
        }
        else if args[0] == "-O" {
            //prints the overall system information and consumption
            overall_stats(process_structure, system);
        }
        else if args[0] == "-K" {
            //kills the process with the pid args[1]
            kill_process(system, &args[1], process_structure);
        }
        else if args[0] == "-KR" {
            //kills the process with the pid args[1] and all its children recursively
            recursive_kill(system, &args[1], process_structure);
        }
        else if args[0] == "-cP" {
            //change priority of process with pid args[1] to args[2]
            change_priority(system, &args[1], &args[2], process_structure);
        }
        // else if args[0] == "-SP"{
        //     //suspends the process with pid args[1]
        //     suspend_process(system, &args[1], process_structure);
        // }
        // else if arg[0]== "-Re"{
        //     //resumes the process with pid args[1]
        //     resume_process(system, &args[1], &args[2],process_structure);
        // }
        else if args[0] == "-H" {
            //prints the help menu
            println!("Miniawy Manager is a process manager that allows you to manage your processes in a simple and easy way.");
            println!("Flags: ");
            println!("-T: Prints the processes in a tree format");
            println!("-S <column>: Sorts the processes by the column provided");
            println!("-F <column> <value>: Filters the processes by the column provided and the value provided");
            println!("-FS <column> <value> <sort_column>: Filters the processes by the column provided and the value provided and then sorts the processes by the column provided");
            println!("-P <R/D>: Prints the processes by the column provided");
            println!("-A <pid>: Searches for the process with the pid provided");
            println!("-N <name>: Searches for the process with the name provided");
            println!("-O: Prints the overall system information and consumption");
            println!("-K <pid>: Kills the process with the pid provided");
            println!("-cP <pid> <priority>: Changes the priority of the process with the pid provided to the priority provided");
            println!("-H: Prints the help menu");
            return;
        }
        else {
            println!("Invalid argument");
            return;
        }
    }

}