// feature_fn is only compiled when my_feature is enabled.
// It appears in the charon output only if [cargo] flags in Charon.toml
// correctly forwarded --features my_feature to cargo build.
#[cfg(feature = "my_feature")]
pub fn feature_fn() -> u32 {
    42
}

fn main() {}
