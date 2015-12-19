#[macro_use] extern crate hyper;

use std::io::Read;
use std::env::Vars;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::Headers;

header! { (XStarfigherAuth, "X-Starfighter-Authorization") => [String]  }

fn auth_header() -> Headers {
    let auth = "toast"; // env!("STOCKFIGHTER");
    let mut headers = Headers::new();
    headers.set(XStarfigherAuth(auth.to_owned()));

    return headers;
}

//fn do_get(url, headers) {
//}

//fn quote(ticker) {

//}

//https://api.stockfighter.io/ob/api/venues/TENYEX/stocks/OBOU/quote --header "X-Starfighter-Authorization:bc5bdf51aeaa778c9a1d74eada32f2f4d1490e93"

fn fire() {
    let client = Client::new();

    let mut res = client.get("http://requestb.in/1eel67m1")
        .headers(auth_header())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("status: {}", res.status);
    println!("headers: {}", res.headers);
    println!("version: {}", res.version);
    println!("url: {}", res.url);

    println!("Response: {}", body);
}

fn main() {
    fire();
    //let auth = env!("STOCKFIGHTER");
    //println!("Auth: {}", auth);

    //let mut headers = Headers::new();
    //headers.set(XStarfigherAuth("hi".to_owned()));

    //println!("DONE: {}", headers);

}

