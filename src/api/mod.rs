mod api_models;
mod cached_client;
mod client;

pub mod cache;

pub use cached_client::{CachedSpotifyClient, SpotifyApiClient};
pub use client::SpotifyApiError;

pub async fn _clear_old_cache() -> Option<()> {
    let cache = cache::CacheManager::new(&[]).unwrap();
    let img_cache = regex::Regex::new(r"^[0-9]{15,30}\.jpg$").unwrap();
    cache
        .clear_cache_pattern("net", &*cached_client::ALL_CACHE)
        .await
        .ok()?;
    cache.clear_cache_pattern("img", &img_cache).await.ok()?;
    Some(())
}
