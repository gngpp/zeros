use serde::{Deserialize, Serialize};

const TOKEN: &str = "RVGLKJtXV7N8wN9S5lseeZBvNt2olq1j";
const NETWORK_ID: &str = "6ab565387aac61a6";

fn main() -> zerotier::Result<()>{
    let zerotier_url = zerotier::api::format_zerotier_member_api(String::from(NETWORK_ID));
    let token = zerotier::api::format_bearer_token(String::from(TOKEN));
    println!("{}", zerotier_url);
    println!("{}", token);
    let client = reqwest::Client::new();
    let result = client.get(&zerotier_url)
        .header(reqwest::header::AUTHORIZATION, token)
        .send()?
        .json::<Vec<zerotier::model::member::MemberResult>>()?;
    println!("{:#?}", result);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
