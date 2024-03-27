use oauth2::{
  basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};

use crate::{error::Error, settings::SETTINGS};

pub fn get_client() -> Result<BasicClient, Error> {
  let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
    .expect("OAuth: invalid authorization endpoint URL");
  let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
    .expect("OAuth: invalid token endpoint URL");

  let redirect_url = SETTINGS.oauth_redirect_uri.clone().to_owned();

  let client_id = ClientId::new(SETTINGS.google_client_id.clone());
  let client_secret = ClientSecret::new(SETTINGS.google_client_secret.clone());

  // Set up the config for the Google OAuth2 process.
  let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
    .set_redirect_uri(RedirectUrl::new(redirect_url).expect("OAuth: invalid redirect URL"))
    .set_revocation_uri(
      RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
        .expect("OAuth: invalid revocation endpoint URL"),
    );
  Ok(client)
}
