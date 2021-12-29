// Cargo
use std::io;
use std::env;


// Local Modules
use::stress_tester::benchmarks::benchmark;

#[tokio::main]
async fn main() {
        // Parse input
    let input_vec:Vec<String> = env::args().collect();


    // Get manga id
    let mut args = Vec::new();

    for i in 1..input_vec.len() {
        args.push(input_vec[i].clone());
    }
    match args[1].as_str() {
        "benchmark" => {
            let mut tasks:i32 = 2;
            let mut connections:i32 = 4;
            let mut time_limit:u64 = 10;
            let mut did_fail:bool = false;


            for i in 2..args.len() {

                if args[i].contains("-t") {

                    let inputed_tasks =  args[i].replace("-t", "");
                    tasks = match inputed_tasks.clone().parse() {
                        Ok(result) => result,
                        Err(_) => {
                            println!("Tasks were not created. {} is not a number.", inputed_tasks);
                            did_fail = true;
                            2
                        }
                    }
                }
                if args[i].contains("-c") {
                    let inputed_tasks =  args[i].replace("-c", "");
                    connections = match inputed_tasks.clone().parse() {
                        Ok(result) => result,
                        Err(_) => {
                            println!("Tasks were not created. {} is not a number.", inputed_tasks);
                            did_fail = true;
                            2
                        }
                    }
                }
                if args[i].contains("-d") {
                    let inputed_tasks =  args[i].replace("-d", "");
                    time_limit = match inputed_tasks.clone().parse() {
                        Ok(result) => result,
                        Err(_) => {
                            println!("Tasks were not created. {} is not a number.", inputed_tasks);
                            did_fail = true;
                            2
                        }
                    }
                }
                match did_fail {
                    true => {}
                    false => {
                        benchmark(String::from("http://localhost:5000/api/posts/618045a63a5cc9120c2a855b"), tasks, connections, time_limit).await;
                    }
                }


            }
        

        },
        _ => {println!("Command is not understood")} 
    }

    
}
