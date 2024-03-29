use std::{
    io::Write,
    time,
};

type Result<T> =
    std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn app() -> Result<()> {
    todo!("Implement the application here");
}

fn main() {
    let start = time::Instant::now();

    env_logger::Builder::from_default_env().format(move |buf, rec| {
                                               let t = start.elapsed()
                                                            .as_secs_f32();
                                               writeln!(buf,
                                                        "{:.03} {} [{}] - \
                                                         {}",
                                                        t,
                                                        rec.level(),
                                                        rec.target(),
                                                        rec.args())
                                           })
                                           .init();

    let runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(app()) {
        Ok(_) => println!("Application exited successfully"),
        Err(e) => println!("Application exited with error: {}", e),
    }
}
