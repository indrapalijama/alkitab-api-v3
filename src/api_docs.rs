use utoipa::OpenApi;
use crate::models::bible::{BibleMetadata, BibleChapter, Verse, ErrorResponse};

/// OpenAPI documentation for the Bible API
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Bible API",
        version = "1.0.0",
        description = "API for accessing Bible content"
    ),
    paths(
        crate::controllers::bible::find,
        crate::controllers::bible::read
    ),
    components(
        schemas(
            BibleMetadata,
            BibleChapter,
            Verse,
            ErrorResponse
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "accesskey",
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Header(utoipa::openapi::security::ApiKeyValue::new("accesskey"))
            )
        );
    }
} 