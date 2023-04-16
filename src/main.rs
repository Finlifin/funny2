use clap::Command;
use funny2::get_online_list;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let id = funny2::get_config_id().unwrap_or("2200310717".to_owned());

    let matches = clap::command!()
        .subcommand(Command::new("online").about("Get the online list"))
        .subcommand(Command::new("in").about("Sign in"))
        .subcommand(Command::new("out").about("Sign out"))
        .subcommand(Command::new("top").about("Get top five"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("in") {
        let result = funny2::sign_in(
            &client,
            "https://at.kexie.space/api/user/signIn",
            id.as_str(),
        )
        .await;
        println!("{result}");
    }

    if let Some(_) = matches.subcommand_matches("out") {
        let result = funny2::sign_in(
            &client,
            "https://at.kexie.space/api/user/signOut",
            id.as_str(),
        )
        .await;
        println!("{result}");
    }

    if let Some(_) = matches.subcommand_matches("top") {
        get_online_list("https://at.kexie.space/api/record/topFive").await;
    }

    if let Some(_) = matches.subcommand_matches("online") {
        get_online_list("https://at.kexie.space/api/record/online").await;
    }
    Ok(())
}
