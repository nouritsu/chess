use std::path::Path;

pub trait AsAssetPath {
    fn as_asset_path(&self) -> &'static Path;
}
