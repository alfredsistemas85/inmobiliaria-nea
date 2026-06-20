use async_trait::async_trait;

#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// Uploads a document and returns the path
    async fn upload_document(
        &self,
        bucket: &str,
        path: &str,
        content: Vec<u8>,
        content_type: &str,
    ) -> Result<String, String>;

    /// Generates a signed URL for a specific path
    async fn generate_signed_url(
        &self,
        bucket: &str,
        path: &str,
        expires_in_seconds: u32,
    ) -> Result<String, String>;
}

pub mod supabase;
