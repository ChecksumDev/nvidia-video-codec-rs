fn main() {
    let version_features: Vec<&str> = [
        "v8_0", "v8_1", "v8_2", "v9_0", "v9_1", "v10_0", "v11_0", "v11_1", "v12_0", "v12_1",
        "v12_2", "v13_0",
    ]
    .iter()
    .copied()
    .filter(|feat| {
        std::env::var(format!(
            "CARGO_FEATURE_{}",
            feat.to_uppercase().replace('.', "_")
        ))
        .is_ok()
    })
    .collect();

    match version_features.len() {
        0 => panic!(
            "nvidia-video-codec-sys: No SDK version feature enabled. \
             Enable exactly one version feature, e.g. `features = [\"v13_0\"]`."
        ),
        1 => {}
        _ => panic!(
            "nvidia-video-codec-sys: Multiple SDK version features enabled: {:?}. \
             Enable exactly one version feature.",
            version_features
        ),
    }
}
