#[macro_export]
macro_rules! query {
    ($base:expr, $user:expr, $pass:expr $(,$query:expr)? ) => {
        reqwest::Client::new()
            .get(format!("{}{}", $base, ENDPOINT))
            .header("Accept", "application/json")
            .timeout(std::time::Duration::from_secs(1))
            .basic_auth($user, Some($pass))
            $(.query($query))?
            .send()
            .await?
    };
    (json $base:expr, $user:expr, $pass:expr $(,$query:expr)?) => {
            query!($base, $user, $pass $(,$query)?).json().await?
    };
    (ok $base:expr, $user:expr, $pass:expr $(,$query:expr)?) => {{
        let resp = query!($base, $user, $pass $(,$query)?);
        resp.status()
            .is_success()
            .then(|| vec![])
            .ok_or(anyhow::anyhow!("request failed: {}", resp.status()))?
    }};
}
