use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{HeaderMap, request::Parts},
};

#[derive(Debug)]
pub struct ExplicitHeader(Option<()>);

impl ExplicitHeader {
    pub fn is_present(&self) -> bool {
        self.0.is_some()
    }
}

impl<S> FromRequestParts<S> for ExplicitHeader
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(ExplicitHeader(
            parts
                .extract::<HeaderMap>()
                .await
                .map_err(|_| ())?
                .get("ENABLED_EXPORTED")
                .and_then(|_| Some(())),
        ))
    }
}
