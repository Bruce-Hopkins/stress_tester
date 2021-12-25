use core::num;
// use reqwest::header::HeaderMap;
// use reqwest::header::AUTHORIZATION;
// use reqwest::header::CONTENT_TYPE;
use std::time::{Instant};
use hyper::{Client};
use hyper::http::{Request};
use futures::future::join_all;
use tokio::task;

// struct BenchmarkReq {
//     url: String,
//     time: String,
//     open_request: u16,
//     threads:u16
// }
struct BenchmarkResults {
    num_of_requests: i32,
    time: i32,
}
async fn benchmark(url:String, tasks:i32) {

    let mut test_time = 0.0;
    let elapsed_time = Instant::now();
    
 
    let mut req_num = 0.0;
    for _task in 1..tasks {
        let url_clone = String::from(&url);

        let tokio_task_result = tokio::spawn(async move {
            let mut numb_of_req = 0.0;

            let url_str = url_clone.as_str();
            
            let request = Request::builder();
            request.uri(url_str);
            let client = Client::new();
            loop {

                let mut vec:Vec<hyper::client::ResponseFuture>  = Vec::new();
                for _i in 0..1 {
                    let uri = url_str.parse::<hyper::Uri>().unwrap();
                    let req:hyper::client::ResponseFuture = client.get(uri);
                    numb_of_req += 1.0;
                    vec.push(req);
                }
        
                let before_test = time::precise_time_s();
                join_all(vec).await;
                test_time = test_time + ((time::precise_time_s() - before_test) * 1000.0);
        
                if elapsed_time.elapsed().as_secs() > 5 {
                    break
                }
            }
            numb_of_req
        }).await.unwrap();
        req_num += tokio_task_result
    }

    
    println!("Tests ran for: {}ms avg", req_num) ;
    println!("Done");
}
#[tokio::main]
async fn main() {
    benchmark(String::from("http://localhost:5000/api/posts/618045a63a5cc9120c2a855b"), 4).await;
}
