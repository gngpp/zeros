use serde::{Deserialize, Serialize};

const TOKEN: &str = "qqg6mFDnTHeLXqAAQkTB22NPO4Qel27R";
const NETWORK_ID: &str = "6ab565387aac61a6";

fn main() -> zerotier::Result<()>{
    // let zerotier = zerotier::Zerotier::new(String::from(TOKEN),vec![String::from(NETWORK_ID)]);
    // let result = zerotier.get_member_result(&String::from(NETWORK_ID));
    // println!("{:#?}", result?);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
