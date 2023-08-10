mod coding;
use crate::coding::*;

mod blogging;
use crate::blogging::*;

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = code() => {println!("finished writing code")}
        _ = blog() => {println!("finished writing blog posts")}
    }
}

async fn code() {
    loop {
        think_about_code();
        write_code();
        debug_code();
        rewrite_code();
        release_code();
    }
}
async fn blog() {
    loop {
        think_about_blogging().await;
        write_blogpost().await;
        read_blogpost().await;
        revise_blogpost().await;
        publish_blogpost().await;
    }
}
