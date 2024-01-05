#[derive(Clone)]
pub enum SearchAuthenticationMethod {
    ApiKey(String),
    TokenCredential(std::sync::Arc<dyn azure_core::auth::TokenCredential>),
}

pub async fn add_auth_header(
    req: &mut azure_core::Request,
    auth_method: &SearchAuthenticationMethod,
    scopes: &[&str],
) -> azure_core::Result<()> {
    match auth_method {
        SearchAuthenticationMethod::ApiKey(api_key) => {
            req.insert_header("api-key", api_key);
        }
        SearchAuthenticationMethod::TokenCredential(credential) => {
            let bearer_token = credential.get_token(scopes).await?;

            req.insert_header(
                azure_core::headers::AUTHORIZATION,
                format!("Bearer {}", bearer_token.token.secret()),
            );
        }
    }

    Ok(())
}
