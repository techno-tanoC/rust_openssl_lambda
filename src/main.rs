use lambda_runtime::{error::HandlerError, lambda, Context};
use simple_error::bail;
use serde_derive::{Serialize, Deserialize};

#[derive(Deserialize, Clone)]
struct CustomEvent {
    url: String
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String
}


fn main() {
    // let body = reqwest::get("https://example.net/").unwrap().text().unwrap();
    // println!("{}", body);
    lambda!(my_handler);
}

fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, HandlerError> {
    if e.url == "" {
        bail!("Empty first name");
    }

    let body = reqwest::get(&e.url).unwrap().text().unwrap();

    Ok(CustomOutput {
        message: body
    })
}
