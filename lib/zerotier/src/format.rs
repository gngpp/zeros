pub struct BaseReqFormat;

impl BaseReqFormat {
    pub fn format_bearer_token(token: &String) -> String {
        format!("Bearer {}", token)
    }
}
