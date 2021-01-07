use hyper::Client;
//use hyper::body::HttpBody as _;
use hyper::{Body, Method, Request, Uri};
use std::env;
use std::fs;
use uuid::Uuid;

use tokio::io::{stdout, AsyncWriteExt as _};

use serde_json;

use serde_json::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let filename = "/Users/vishwakapoor/Code/Rust/FileDbClient/target/debug/movies.json";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let array: Vec<serde_json::Value> = serde_json::from_str(&contents)?;

    //let title = &array[0]["title"];

    //let serialized = serde_json::to_string(&array[0])?;

    //println!("Title: {}", title);
    //println!("serialized: {}", serialized);

    for elem in &array {
        let value = serde_json::to_string(&array[0])?.replace(r#"""#, r#"\""#);
        let my_uuid = Uuid::new_v4();
        let mut foo = r#"[{"key":"#.to_string();
        foo.push_str(&my_uuid.to_string());
        foo.push_str(r#"","value": ""#);
        foo.push_str(&value);
        foo.push_str(r#""}]"#);
        println!("{}", foo);
        //let json_element =  & my_uuid & r#","value": "this is some texty text "}]"#;

        let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:5000/db/Movies")
        .header("content-type", "application/json")
        .body(Body::from(foo))?;

        let client = Client::new();

        // POST it...
        let resp = client.request(req).await?;
        
        println!("Response: {0} {1}", resp.status(), elem["title"]);
    }
    

    

    // This is where we will setup our HTTP client requests.

    // let client = Client::new();

    // let mut res = client.get("http://localhost:5000/db/Movies/12348".parse()?).await?;

    // println!("Response: {}", res.status());
    // println!("Headers: {:#?}\n", res.headers());

    // // Stream the body, writing each chunk to stdout as we get it
    // // (instead of buffering and printing at the end).
    // while let Some(chunk) = res.body_mut().data().await {
    //     stdout().write_all(&chunk?).await?;
    // }
    
    Ok(())
}