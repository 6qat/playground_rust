use futures::executor::block_on;

async fn task1() -> i32 {
    println!("First greeter");
    1
}

async fn task2() -> i32 {
    println!("Second greeter");
    2
}

async fn async_main() {
    // greeter().await;
    // second_greet().await;
    let (result1, result2) = futures::join!(task1(), task2());
    println!("{} {}", result1, result2);
}

fn main() {
    block_on(async_main());
    block_on(async {
        println!("Async block");
    });
    let b = async {
        println!("Another async block");
    };
    block_on(b);
}
