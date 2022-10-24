use crate::chess_app::data::Color;

#[test]
fn test_color_serde() {
    assert_eq!(serde_json::to_string(&Color::Black).unwrap(), "\"Black\"");
}