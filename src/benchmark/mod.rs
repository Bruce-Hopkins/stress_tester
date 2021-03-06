pub mod benchmarks {
    use std::time::{Instant};
    use hyper::{Client};
    use hyper::http::{Request};
    use futures::future::join_all;

    struct BenchmarkResults {
        num_of_requests: f64,
        time: f64,
    }
    
    pub async fn benchmark(url:String, tasks:i32, connections:i32, time_limit:u64) {
        if tasks > connections {
            println!("Connections cannot be less than the tasks.");
            return;
        }
    
        let elapsed_time = Instant::now();
        let cal_conn:i32 = connections / tasks;
        let mut benchmark_results = BenchmarkResults {
            num_of_requests: 0.0,
            time: 0.0,    
        };
    
        let mut c_tasks_vec = Vec::new();
        for _task in 0..tasks {
    
            let url_clone = String::from(&url);
            let c_task = tokio::spawn(async move {
                let mut numb_of_req = 0.0;
                let mut test_time = 0.0;
    
                let url_str = url_clone.as_str();                    
                let request = Request::builder();
                request.uri(url_str);
                let client = Client::new();        
    
    
                loop {
                    let mut vec:Vec<hyper::client::ResponseFuture>  = Vec::new();
                    for _i in 0..cal_conn {
                        let uri = url_str.parse::<hyper::Uri>().unwrap();
                        let req:hyper::client::ResponseFuture = client.get(uri);
                        vec.push(req);
                    }
    
    
            
                    let before_test = time::precise_time_s();
                    join_all(vec).await;
                    numb_of_req += 1.0;
                    test_time = test_time + ((time::precise_time_s() - before_test) * 1000.0);
    
            
                    if elapsed_time.elapsed().as_secs() > time_limit {
                        break
                    }
                }
                 BenchmarkResults {
                    num_of_requests: numb_of_req,
                    time: test_time    
                }
            });
            c_tasks_vec.push(c_task);   
    
        }
        for i in join_all(c_tasks_vec).await {
            let i = i.unwrap(); 
            benchmark_results.num_of_requests = benchmark_results.num_of_requests + i.num_of_requests;
            benchmark_results.time = benchmark_results.time + i.time;
        }
    
        // We multiplie by the connections because we make the requests in group
        println!("{} Total requests", benchmark_results.num_of_requests * connections as f64 );
        println!("{} Req/Sec", benchmark_results.num_of_requests * connections as f64 / time_limit as f64);
    
        println!("Tests ran for: {}ms avg", benchmark_results.time / benchmark_results.num_of_requests);
        println!("Done");
    }
    
    

}
