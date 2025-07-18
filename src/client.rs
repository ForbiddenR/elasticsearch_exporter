#[macro_export]
macro_rules! query {
    ($base:expr $(,$query:expr)?) => {
        reqwest::Client::new()
            .get(format!("{}{}", $base, ENDPOINT))
            .header("Accept", "application/json")
            $(.query($query))?
            .timeout(std::time::Duration::from_secs(1))
            .send()
            .await?
    };
    (json $base:expr $(,$query:expr)?) => {
            query!($base $(,$query)?).json().await?
    };
    (ok $base:expr $(,$query:expr)?) => {{
        let resp = query!($base $(,$query)?);
        resp.status()
            .is_success()
            .then(|| vec![])
            .ok_or(anyhow::anyhow!("request failed: {}", resp.status()))?
    }};
}
