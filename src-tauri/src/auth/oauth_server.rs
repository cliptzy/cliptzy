use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use crate::error::CliptzyError;

pub const OAUTH_REDIRECT_PORT: u16 = 54321;

/// Binds a local TCP listener and waits for the OAuth redirect callback.
/// Returns the authorization code extracted from the callback URL query string.
pub async fn listen_for_auth_code() -> Result<String, CliptzyError> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", OAUTH_REDIRECT_PORT))
        .await
        .map_err(|e| CliptzyError::Supabase(format!("failed to bind oauth listener: {}", e)))?;

    let (mut stream, _) = listener
        .accept()
        .await
        .map_err(|e| CliptzyError::Supabase(format!("failed to accept oauth callback: {}", e)))?;

    let mut buffer = [0; 4096];
    let bytes_read = stream
        .read(&mut buffer)
        .await
        .map_err(|e| CliptzyError::Supabase(format!("failed to read oauth callback: {}", e)))?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    let auth_code = parse_auth_code_from_request(&request);

    let response = if auth_code.is_some() {
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Login Berhasil!</h1><p>Anda dapat menutup jendela ini.</p>"
    } else {
        "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\n<h1>Login Gagal</h1><p>Kode otorisasi tidak ditemukan.</p>"
    };
    let _ = stream.write_all(response.as_bytes()).await;

    auth_code.ok_or_else(|| CliptzyError::Supabase("auth code not found in request".into()))
}

fn parse_auth_code_from_request(request: &str) -> Option<String> {
    let line = request.lines().next()?;
    if !line.starts_with("GET ") {
        return None;
    }
    let path = line.split_whitespace().nth(1)?;
    let url = url::Url::parse(&format!("http://localhost{}", path)).ok()?;
    for (key, val) in url.query_pairs() {
        if key == "code" {
            return Some(val.into_owned());
        }
    }
    None
}

pub fn build_google_auth_url(supabase_url: &str, code_challenge: &str) -> String {
    format!(
        "{}/auth/v1/authorize?provider=google&redirect_to=http://localhost:{}&code_challenge={}&code_challenge_method=s256&scopes=https://www.googleapis.com/auth/youtube.upload",
        supabase_url, OAUTH_REDIRECT_PORT, code_challenge
    )
}
