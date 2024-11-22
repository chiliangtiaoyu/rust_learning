use std::process::id;

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 2;
    let mut i = 2u8;
    loop {
        let c = a + b;
        a = b;
        b = c;  // 这里不会导致“移动”
        println!("next val is {}", c);
        i += 1;
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 2, 2u8);
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", c);
        i += 1;
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 2);
    for i in 0..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next i is {},{}", i, c);
    }
}

fn matchTest(event: &Event) {
    match event {
        Event::Join((_, tpId)) => println!("tp:{:?}", tpId),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}

use std::fs;

// main 函数现在返回一个 Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);

    Ok(())
}

