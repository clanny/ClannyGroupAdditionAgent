// Only include in debug builds
#[cfg(debug_assertions)]
extern crate dotenv;
#[cfg(debug_assertions)]
use dotenv::dotenv;
use serde_json::json;
use std::{thread, time::Duration};

mod baserow_api;
mod discord_api;

use std;

#[tokio::main]
async fn main() {
    // Only include in debug builds
    #[cfg(debug_assertions)]
    dotenv().ok();

    let baserow_token = std::env::var("BASEROW_TOKEN").expect("BASEROW_TOKEN must be set");
    let discord_token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    let _groups_to_notify = &baserow_api::get_groups_not_notified(baserow_token.clone())
        .await
        .unwrap()["results"];
    let groups_to_notify = _groups_to_notify.as_array();

    if (groups_to_notify.is_none()) {
        println!("No groups to notify");
        return;
    }

    // Iterate over the array
    for group in groups_to_notify.unwrap() {
        if (group["Added"] == true) && (group["User Notified"] == false) {
            let groups_to_notify = groups_to_notify.clone();
            let group_id = group["Group ID"].as_str().unwrap();
            let row_id = group["id"].as_i64().unwrap();
            let requestor = group["Requestor"].as_str().unwrap();
            let discord_server_id = group["Discord Server ID"].as_str().unwrap();

            let _channels = &discord_api::get_guild_channels(
                discord_server_id.to_string(),
                discord_token.clone(),
            )
            .await
            .unwrap();
            let channels = _channels.as_array();

            let channel_names = channels
                .unwrap()
                .iter()
                .map(|channel| channel["name"].as_str().unwrap().to_string())
                .collect::<Vec<String>>();

            // Iterate over the array
            for channel in channels.unwrap() {
                let channel_id = channel["id"].as_str().unwrap();
                let channel_name = channel["name"].as_str().unwrap();

                println!("{}: {}", channel_id, channel_name);
            }

            let channel_to_send_to = discord_api::get_channel_to_send_to(channel_names);

            // get channel id
            let channel_id = channels
                .unwrap()
                .iter()
                .find(|channel| channel["name"].as_str().unwrap() == channel_to_send_to)
                .unwrap()["id"]
                .as_str()
                .unwrap();

            let embed = json!(
            {
                "embeds": [
                    {
                        "title": "The account has been added!",
                        "color": 3319890,
                        "type": "rich",
                        "description": format!(
                            "The account **{}** has been added to your group!\nYou should run </account:995722308670992517> with account ID being **{}** (the name) to configure Clanny to use the account!",
                            group["Account"].to_string(),
                            group["Account"].to_string()
                        )
                    }
                    ]
                });

            discord_api::send_message(
                discord_server_id.to_string(),
                channel_id.to_string(),
                embed.clone(),
                discord_token.clone(),
            )
            .await;

            discord_api::send_dm_message(requestor.to_string(), embed, discord_token.clone()).await;

            baserow_api::set_group_notified(baserow_token.clone(), row_id).await;
        }
    }

    //.map(|v| v["name"].as_str().unwrap().to_string().unwrap()),

    thread::sleep(Duration::from_secs(900));

    println!("ive finished, pls restart me");
}
