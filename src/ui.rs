use eframe::egui;

pub const IMG_BOOK: &[u8] = include_bytes!("../assets/book.png");
pub const IMG_PENCIL: &[u8] = include_bytes!("../assets/pencil.png");
pub const IMG_PLUS: &[u8] = include_bytes!("../assets/plus.png");
pub const IMG_EDIT: &[u8] = include_bytes!("../assets/edit.png");
pub const IMG_DELETE: &[u8] = include_bytes!("../assets/delete.png");
pub const IMG_ARROW: &[u8] = include_bytes!("../assets/arrow.png");
pub const IMG_DRAG: &[u8] = include_bytes!("../assets/drag.png"); 

pub fn icon_btn(ui: &mut egui::Ui, uri: &'static str, img_data: &'static [u8], size: f32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::click());
    let tint_color = if response.hovered() { egui::Color32::from_rgb(108, 91, 123) } else { egui::Color32::from_rgb(130, 130, 130) };
    let img = egui::Image::from_bytes(uri, img_data).fit_to_exact_size(egui::vec2(size, size)).tint(tint_color);
    img.paint_at(ui, rect);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn drag_handle(ui: &mut egui::Ui, size: f32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::drag());
    let tint_color = if response.hovered() || response.dragged() { egui::Color32::from_rgb(108, 91, 123) } else { egui::Color32::from_gray(190) };
    let img = egui::Image::from_bytes("bytes://drag.png", IMG_DRAG).fit_to_exact_size(egui::vec2(size, size)).tint(tint_color);
    img.paint_at(ui, rect);
    // 强制光标为“小手”而不是默认的拖拽十字
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}
