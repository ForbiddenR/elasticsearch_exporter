use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{HeaderMap, request::Parts},
};

use crate::config::Mode;

#[derive(Debug)]
pub struct ExplicitHeader(pub Option<Mode>);

impl<S> FromRequestParts<S> for ExplicitHeader
where
    S: Send + Sync,
{
    type Rejection = &'static str;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(ExplicitHeader(
            parts
                .extract::<HeaderMap>()
                .await
                .map_err(|_| "extract headers failed")?
                .get("EXPORTER_MODE")
                .map(|h| h.try_into())
                .transpose()?,
        ))
    }
}
