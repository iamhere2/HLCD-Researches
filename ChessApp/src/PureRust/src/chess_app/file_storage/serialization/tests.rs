use crate::chess_app::data::Color;

#[test]
fn test_color_serde() {
    let s = serde_json::to_string(&Color::Black).unwrap();
    assert_eq!(s, "\"Black\"");

    let c: Color = serde_json::from_str(&s).unwrap();
    assert_eq!(c, Color::Black);
}