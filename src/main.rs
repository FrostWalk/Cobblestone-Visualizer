use common_messages::messages::{Command, Request};

#[tokio::main]
async fn main() {
    let a = Request::new(Command::NextTick);
    println!("{:?}", a);
}
