// use reqwest::header::HeaderMap;
// use reqwest::header::AUTHORIZATION;
// use reqwest::header::CONTENT_TYPE;
// use time;
use hyper::{body::HttpBody as _, Client};
use hyper::http::{Request, Response};
use futures::future::join_all;
// struct BenchmarkReq {
//     url: String,
//     time: String,
//     open_request: u16,
//     threads:u16
// }
async fn benchmark(url:&str) {

    let mut test_time = 0.0;
    let request = Request::builder();
    request.uri(url);
    let client = Client::new();
    for _i in 1..1000 {

        let mut vec:Vec<hyper::client::ResponseFuture>  = Vec::new();
        for i in 0..19 {
            let url = url.parse::<hyper::Uri>().unwrap();
            let req:hyper::client::ResponseFuture = client.get(url);
            vec.push(req); 
        }


        let before_test = time::precise_time_s();
        join_all(vec).await;
        test_time = test_time + ((time::precise_time_s() - before_test) * 1000.0);

    }
    
    println!("Tests ran for: {}ms av", test_time / 1000.0);

    println!("Done");
}
#[tokio::main]
async fn main() {
    /* 
        Make a function to pass the 
            1 url 
            2 the time for requests  
            3 open requests.
            4 number of threads
    */
    benchmark("http://localhost:5000/api/posts/618045a63a5cc9120c2a855b").await;
}

