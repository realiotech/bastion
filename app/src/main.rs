#![crate_name = "app"]

use app::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
