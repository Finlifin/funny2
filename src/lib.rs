#[allow(dead_code, unused)]
use std::fmt::{Debug, Display};

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

pub async fn get_top(url: &str) {
    let response = reqwest::get(url)
        .await
        .unwrap_or_else(|err| err_and_exit(err))
        .text()
        .await
        .unwrap();

    let result: ResponseBody = serde_json::from_str(&response).unwrap();
    print!("{}", &result);
    for user in result.data.iter() {
        println!("{}:{}", &user, user.total_time.as_ref().unwrap());
    }
}

pub fn get_config_id() -> Result<String, std::io::Error> {
    let binding = dir::home_dir().unwrap();
    let path = format!("{}/.config/funny.conf", binding.to_str().unwrap());
    let config_file = std::fs::File::open(path).unwrap();
    let id = std::io::read_to_string(config_file).unwrap();

    Ok(id)
}

pub async fn sign_out(client: &reqwest::Client, url: &str, id: &str) -> String {
    let body = format!("{{ \"userId\": {}}}", id);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // println!("{}", &body);
    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    response
}

pub async fn sign_in(client: &reqwest::Client, url: &str, id: &str) -> String {
    let body = format!("{{ \"userId\": {}}}", id);
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    response
}

pub async fn get_online_list(url: &str) {
    let response = reqwest::get(url)
        .await
        .unwrap_or_else(|err| err_and_exit(err))
        .text()
        .await
        .unwrap_or("Fail to get online list\n".to_owned());
    let result: ResponseBody =
        serde_json::from_str(&response).unwrap_or_else(|err| err_and_exit(err));
    print!("{}", &result);
    for user in result.data.iter() {
        println!("{}", user);
    }
}

fn err_and_exit<T>(err: T) -> !
where
    T: Debug + Display,
{
    eprintln!("{}", err);
    std::process::exit(1);
}

#[derive(Serialize, Deserialize)]
struct User {
    #[serde(alias = "userId")]
    id: u64,
    #[serde(alias = "userName")]
    name: String,
    #[serde(alias = "userDept")]
    dept: String,
    #[serde(alias = "userLocation")]
    location: String,
    #[serde(alias = "totalTime")]
    total_time: Option<String>,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t   =>{}\t", &self.dept, &self.name)
    }
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    data: Vec<User>,
    msg: String,
    code: i32,
}

impl Display for ResponseBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", &self.msg)
    }
}
