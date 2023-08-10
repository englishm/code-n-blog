pub async fn think_about_blogging() {
    println!("thinking about blogging");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await
}
pub async fn write_blogpost() {
    println!("writing a blog post");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await
}
pub async fn read_blogpost() {
    println!("reading the blog post");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await
}
pub async fn revise_blogpost() {
    println!("revising the blog post");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await
}
pub async fn publish_blogpost() {
    println!("publishing the blog post");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await
}
