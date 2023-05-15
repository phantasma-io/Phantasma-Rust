use crate::PhantasmaAPI::Account::Account;
use reqwest::{self, Error};
pub const API_URL: &str = "http://pharpc1.phantasma.io:7077/";

pub async fn get_account(address: String) -> String {
    let url = API_URL.to_string() + "api/v1/GetAccount?account=" + address.as_str();
    println!("{}", url);
    match http_get(url).await {
        Ok(res) => {
            let account: Account = serde_json::from_str(&res).unwrap();
            println!("Body:\n{:#?}", account);
            return res;
        }
        Err(e) => {
            println!("Error: {}", e);
            return "Error".to_string();
        }
    }
    //let json = httpGet(url);

    //let account = Account.fromJSON(json);
    return "tes".to_string();
}

async fn http_get(url: String) -> Result<String, Error> {
    println!("Making request: {}", url);
    let res = reqwest::get(url).await?.text().await?;
    println!("Body:\n{}", res);
    return Ok(res);
    //let res = client.get(url).send().unwrap();
    //let body = res.text().unwrap();
}
