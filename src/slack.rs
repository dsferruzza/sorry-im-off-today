use chrono::{DateTime, Utc};

pub fn set_status(token: &String, status_text: &str, status_emoji: &str, status_expiration: Option<DateTime<Utc>>) -> Result<slack_api::users_profile::SetResponse, slack_api::users_profile::SetError<slack_api::requests::Error>> {
    let client = slack_api::default_client().unwrap();
    test_auth(token, &client);

    let new_profile_str = mk_profile_str(status_text, status_emoji, status_expiration.map(|d| d.timestamp().to_string()));
    let set_request = slack_api::users_profile::SetRequest { profile: Some(new_profile_str.as_str()), user: None, name: None, value: None };
    println!("[Slack] Set status to: {}", new_profile_str);
    slack_api::users_profile::set(&client, &token, &set_request)
}

fn test_auth(token: &String, client: &slack_api::requests::Client) {
    let auth_test = slack_api::auth::test(client, &token).expect("The provided Slack API token doesn't work.");
    println!("[Slack] Authenticated in {} ({}) as {} ({})", auth_test.team.unwrap(), auth_test.team_id.unwrap(), auth_test.user.unwrap(), auth_test.user_id.unwrap());
}

fn mk_profile_str(status_text: &str, status_emoji: &str, status_expiration: Option<String>) -> String {
    let items = json!({
        "status_text": status_text,
        "status_emoji": status_emoji,
        "status_expiration": status_expiration,
    });
    items.to_string()
}
