use reqwest;
use serde_json::{json, Value};
use std::env;

pub async fn get_guild_channels(
    guild_id: String,
    api_token: String,
) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!(
            "https://discord.com/api/guilds/{}/channels",
            guild_id
        ))
        .header("Authorization", "Bot ".to_string() + &api_token)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        //.text()
        .await
        .unwrap();

    //Ok(json!({}))

    Ok(resp)
}

pub async fn send_message(guild_id: String, channel_id: String, message: Value, api_token: String) {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "https://discord.com/api/channels/{}/messages",
            channel_id
        ))
        .header("Authorization", "Bot ".to_string() + &api_token)
        .json(&message)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        //.text()
        .await
        .unwrap();

    println!("{:#?}", resp.clone());
}

async fn create_dm(user_id: String, api_token: String) -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://discord.com/api/users/@me/channels")
        .header("Authorization", "Bot ".to_string() + &api_token)
        .json(&json!({ "recipient_id": user_id }))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        //.text()
        .await
        .unwrap();

    resp["id"].as_str().unwrap().to_string()
}

pub async fn send_dm_message(user_id: String, message: Value, api_token: String) {
    let channel_id = create_dm(user_id, api_token.clone()).await;
    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "https://discord.com/api/channels/{}/messages",
            channel_id
        ))
        .header("Authorization", "Bot ".to_string() + &api_token)
        .json(&message)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        //.text()
        .await
        .unwrap();

    println!("{:#?}", resp.clone());
}

pub fn get_channel_to_send_to(channels: Vec<String>) -> String {
    // Iterate over the array
    for channel in channels.clone() {
        if channel.to_lowercase().contains("chat") {
            return channel;
        }
        if channel.to_lowercase().contains("general") {
            return channel;
        }
    }
    return channels.get(0).unwrap().to_string();
}
