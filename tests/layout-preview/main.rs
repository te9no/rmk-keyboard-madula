fn main() {
    let path = std::env::args().nth(1).expect("keyboard.toml path");
    let source = std::fs::read_to_string(path).unwrap();
    let config: toml::Value = toml::from_str(&source).unwrap();
    let layout_toml = toml::to_string(&config["layout"]).unwrap();
    let layout = rmk_config::layout::layout_info_from_toml(&layout_toml).unwrap().unwrap();
    let blob = rmk_config::layout::layout_blob_from_toml(&layout_toml).unwrap();
    let decoded = rmk_config::layout::LayoutInfo::from_compressed_blob(&blob).unwrap();
    assert_eq!(layout, decoded);
    let variant = &decoded.variants[decoded.default_variant as usize];
    assert_eq!(variant.keys.len(), 64);
    assert_eq!(variant.encoders.len(), 2);
    let lang1 = variant.keys.iter().find(|k| (k.row, k.col) == (3,12)).unwrap();
    assert!((lang1.r + 23.0).abs() < 0.001);
    assert!(lang1.rect.y < 4.7, "LANG1 must rotate back into the thumb cluster");
    let left_thumb = variant.keys.iter().find(|k| (k.row, k.col) == (3,5)).unwrap();
    assert!((left_thumb.r - 23.0).abs() < 0.001);
    println!("{}", serde_json::to_string_pretty(&decoded).unwrap());
}
