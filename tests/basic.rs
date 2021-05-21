use attr::Attributes;
//big mistake every set should return same value if changed
#[cfg(test)]
#[test]
fn bounding_rectangle_color() {
    let mut a = Attributes::new();
    let val = a.get_bounding_rectangle_color();
    //check deffault value
    assert_eq!(val,"red");
    a.set_bounding_rectangle_color("strangeText".to_string());
    let val_two = a.get_bounding_rectangle_color();
    assert_eq!(val_two,"strangeText");

}
#[test]
fn shadow_color() {
    let mut a = Attributes::new();
    a.set_shadow_color("strangeText".to_string());
    let val = a.get_shadow_color();
    assert_eq!(val,"strangeText");
}
#[test]
fn name() {
    let mut a = Attributes::new();
    a.set_name("strangeText".to_string());
    let val = a.get_name();
    assert_eq!(val,"strangeText");
}
#[test]
fn color() {
    let mut a = Attributes::new();
    a.set_color("strangeText".to_string());
    let val = a.get_color();
    assert_eq!(val,"strangeText");
}
#[test]
fn clockwise() {
    let mut a = Attributes::new();
    a.set_clockwise(false);
    let val = a.get_clockwise();
    assert_eq!(val,false);
}
#[test]
fn filled() {
    let mut a = Attributes::new();
    a.set_filled(false);
    let val = a.get_filled();
    assert_eq!(val,false);
}
#[test]
fn draw_bounding_rectangle() {
    let mut a = Attributes::new();
    a.set_draw_bounding_rectangle(false);
    let val = a.get_draw_bounding_rectangle();
    assert_eq!(val,false);
}
#[test]
fn opacity() {
    let mut a = Attributes::new();
    a.set_opacity(666);
    let val = a.get_opacity();
    assert_eq!(val,666);
}
#[test]
fn x() {
    let mut a = Attributes::new();
    a.set_x(666);
    let val = a.get_x();
    assert_eq!(val,666);
}
#[test]
fn y() {
    let mut a = Attributes::new();
    a.set_y(666);
    let val = a.get_y();
    assert_eq!(val,666);
}
#[test]
fn width() {
    let mut a = Attributes::new();
    a.set_width(666);
    let val = a.get_width();
    assert_eq!(val,666);
}
#[test]
fn height() {
    let mut a = Attributes::new();
    a.set_height(666);
    let val = a.get_height();
    assert_eq!(val,666);
}
#[test]
fn start_angle() {
    let mut a = Attributes::new();
    a.set_start_angle(666);
    let val = a.get_start_angle();
    assert_eq!(val,666);
}
#[test]
fn line_width() {
    let mut a = Attributes::new();
    a.set_line_width(666);
    let val = a.get_line_width();
    assert_eq!(val,666);
}

#[test]
fn shadow_blur() {
    let mut a = Attributes::new();
    a.set_shadow_blur(666);
    let val = a.get_shadow_blur();
    assert_eq!(val,666);
}
#[test]
fn shadow_offset_x() {
    let mut a = Attributes::new();
    a.set_shadow_offset_x(666);
    let val = a.get_shadow_offset_x();
    assert_eq!(val,666);
}
#[test]
fn shadow_offset_y() {
    let mut a = Attributes::new();
    a.set_shadow_offset_y(666);
    let val = a.get_shadow_offset_y();
    assert_eq!(val,666);
}
#[test]
fn line_dash_size() {
    let mut a = Attributes::new();
    a.set_line_dash_size(666);
    let val = a.get_line_dash_size();
    assert_eq!(val,666);
}
#[test]
fn line_dash_gap() {
    let mut a = Attributes::new();
    a.set_line_dash_gap(666);
    let val = a.get_line_dash_gap();
    assert_eq!(val,666);
}
#[test]
fn bounding_rectangle_padding() {
    let mut a = Attributes::new();
    a.set_bounding_rectangle_padding(666);
    let val = a.get_bounding_rectangle_padding();
    assert_eq!(val,666);
}








