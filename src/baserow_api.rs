use std::collections::HashMap;

use reqwest;
use serde_json::{json, Value};

pub async fn get_groups_not_notified(api_token: String) -> Result<Value, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://baserow.clanny.systems/api/database/rows/table/551/?user_field_names=true&filter__field_4748__boolean=true&include=Group ID,Requestor,Discord Server ID,Added,User Notified,Account&filter__field_4744__boolean=false")
        .header("Authorization", "Token ".to_string() + &api_token)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        //.text()
        .await
        .unwrap();

    println!("{:#?}", resp.clone());

    Ok(resp)
}

pub fn get_YYYY_MM_DD() -> String {
    let now = chrono::Local::now();
    let year = now.format("%Y").to_string();
    let month = now.format("%m").to_string();
    let day = now.format("%d").to_string();
    return year + "-" + &month + "-" + &day;
}

pub async fn set_group_notified(api_token: String, row_id: i64) {
    let client = reqwest::Client::new();
    let resp = client
        .patch(format!(
            "https://baserow.clanny.systems/api/database/rows/table/551/{}/?user_field_names=true",
            row_id
        ))
        .header("Authorization", "Token ".to_string() + &api_token)
        .json::<Value>(&json!({
            "User Notified": true,
            "User Notified At": get_YYYY_MM_DD()
        }))
        .send()
        .await
        .unwrap()
        .json::<Value>()
        //.text()
        .await
        .unwrap();

    println!("{:#?}", resp);
}
