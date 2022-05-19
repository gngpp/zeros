pub fn format_zerotier_member_api(network_id: String) -> String {
    format!("https://my.zerotier.com/api/network/{}/member", network_id)
}

pub fn format_bearer_token(token: String) -> String {
    format!("Bearer {}", token)
}