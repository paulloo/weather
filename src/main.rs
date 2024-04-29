mod handler;
use handler::{print_response, Weather};

use exitfailure::ExitFailure;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Input {
    pub city: String,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let input = Input::from_args();
    match Weather::get(&input.city).await {
        Ok(r) => print_response(&r),
        Err(e) => println!("请求出错，{:?}", &e),
    };

    Ok(())
}
