pub mod flag_functions {
    use std::collections::HashMap;
    use crate::ProcessInfo;
    use crate::sorting::sorting_functions::sort;
    use crate::filtering::filtering_functions::filter;
    use crate::printing::printing_functions::print;
    use crate::searching::searching_functions::{search_by_name, search_by_pid};
    use crate::tree::tree_functions::tree;

    pub fn call_function_by_flag(process_structure: &mut HashMap<u32, ProcessInfo>, args: Vec<String>){

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
            filter(process_structure, &args[1], &args[2]);
            return;
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

        }
        else if args[0] == "-K" {
            //kills the process with the pid args[1]
            
        }
        else if args[0] == "-cP" {
            //change priority of process with pid args[1] to args[2]
        }
        else {
            println!("Invalid argument");
            return;
        }
    }

}