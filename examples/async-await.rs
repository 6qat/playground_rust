use futures::executor::block_on;

async fn greeter() {
    println!("First greeter");
}

async fn second_greet() {
    println!("Second greeter");
}

async fn async_main() {
    greeter().await;
    second_greet().await;
}

fn main() {
    block_on(async_main());
}