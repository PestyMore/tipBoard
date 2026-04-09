use crate::types::{ListItem, Mode, Part};
use crate::storage::{load_data, save_data};
use crate::ui;
use crate::config::*; // 引入集中配置项
use eframe::egui;
use uuid::Uuid;

pub struct TipBoardApp {
    pub data: Vec<Part>,
    pub mode: Mode,
    pub delete_confirm: Option<String>, 
    pub edit_target: Option<String>,
    pub focus_requested_id: Option<String>,
    pub is_maximized: bool,
    pub dragging_part: Option<usize>,
    pub dragging_item: Option<(String, usize)>,
}

impl TipBoardApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut fonts = egui::FontDefinitions::default();
        if let Ok(font_data) = std::fs::read("C:\\Windows\\Fonts\\msyh.ttc").or_else(|_| std::fs::read("C:\\Windows\\Fonts\\simsun.ttc")) {
            fonts.font_data.insert("my_font".to_owned(), egui::FontData::from_owned(font_data).into());
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "my_font".to_owned());
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "my_font".to_owned());
        }
        cc.egui_ctx.set_fonts(fonts);

        let mut style = (*cc.egui_ctx.style()).clone();
        let text_color = egui::Color32::from_rgb(88, 110, 117); 
        
        style.visuals.window_fill = egui::Color32::TRANSPARENT; 
        style.visuals.panel_fill = egui::Color32::TRANSPARENT;
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(245, 238, 218); 
        style.visuals.selection.bg_fill = egui::Color32::from_rgb(210, 200, 220); 
        style.visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(108, 91, 123)); 
        
        for widget in[&mut style.visuals.widgets.noninteractive, &mut style.visuals.widgets.inactive, &mut style.visuals.widgets.hovered, &mut style.visuals.widgets.active] {
            widget.bg_fill = egui::Color32::TRANSPARENT;
            widget.bg_stroke.width = 0.0; 
            widget.fg_stroke.color = text_color; 
        }
        
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_black_alpha(10);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_black_alpha(20);
        
        // 应用字体配置
        style.text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(FONT_SIZE_BODY));
        style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::proportional(FONT_SIZE_HEADING));
        style.text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(FONT_SIZE_BODY));
        cc.egui_ctx.set_style(style);

        Self {
            data: load_data(),
            mode: Mode::Read,
            delete_confirm: None,
            edit_target: None,
            focus_requested_id: None,
            is_maximized: false,
            dragging_part: None,
            dragging_item: None,
        }
    }

    fn save_state(&self) { save_data(&self.data); }

    fn draw_top_bar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(ui.available_width(), TOP_BAR_HEIGHT), egui::Sense::click_and_drag());
        if response.drag_started() { ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag); }
        
        ui.painter().line_segment([rect.left_bottom(), rect.right_bottom()], egui::Stroke::new(1.0, egui::Color32::from_black_alpha(10)));
        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "tipBoard v1.0.0", egui::FontId::proportional(FONT_SIZE_BODY), egui::Color32::from_rgb(130, 130, 130));

        ui.allocate_ui_at_rect(rect, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.add_space(15.0);
                let btn_size = egui::vec2(WINDOW_CTRL_BTN_SIZE, WINDOW_CTRL_BTN_SIZE);
                
                let (close_id, close_rect) = ui.allocate_space(btn_size);
                let close_resp = ui.interact(close_rect, close_id, egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.painter().circle_filled(close_rect.center(), WINDOW_CTRL_BTN_RADIUS, if close_resp.hovered() { egui::Color32::from_rgb(255, 95, 86) } else { egui::Color32::from_rgb(255, 120, 113) });
                if close_resp.clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                ui.add_space(6.0);

                let (min_id, min_rect) = ui.allocate_space(btn_size);
                let min_resp = ui.interact(min_rect, min_id, egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.painter().circle_filled(min_rect.center(), WINDOW_CTRL_BTN_RADIUS, if min_resp.hovered() { egui::Color32::from_rgb(255, 189, 46) } else { egui::Color32::from_rgb(255, 200, 80) });
                if min_resp.clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true)); }
                ui.add_space(6.0);

                let (max_id, max_rect) = ui.allocate_space(btn_size);
                let max_resp = ui.interact(max_rect, max_id, egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
                ui.painter().circle_filled(max_rect.center(), WINDOW_CTRL_BTN_RADIUS, if max_resp.hovered() { egui::Color32::from_rgb(39, 201, 63) } else { egui::Color32::from_rgb(80, 210, 100) });
                if max_resp.clicked() { 
                    self.is_maximized = !self.is_maximized;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(self.is_maximized)); 
                }
            });
        });

        ui.allocate_ui_at_rect(rect, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(15.0);
                
                let (mode_uri, mode_img) = if self.mode == Mode::Read { ("bytes://pencil.png", ui::IMG_PENCIL) } else { ("bytes://book.png", ui::IMG_BOOK) };
                if ui::icon_btn(ui, mode_uri, mode_img, ICON_MODE_SWITCH).clicked() {
                    self.mode = if self.mode == Mode::Read { Mode::Edit } else { Mode::Read };
                    self.edit_target = None; 
                    self.save_state();
                }

                if self.mode == Mode::Edit {
                    ui.add_space(10.0);
                    if ui::icon_btn(ui, "bytes://plus.png", ui::IMG_PLUS, ICON_TOP_PLUS).clicked() {
                        let new_id = Uuid::new_v4().to_string();
                        self.data.insert(0, Part { id: new_id.clone(), name: String::new(), items: Vec::new() });
                        self.edit_target = Some(new_id.clone()); 
                        self.focus_requested_id = Some(new_id);
                    }
                }
            });
        });
        ui.add_space(5.0);
    }

    fn draw_read_mode(&mut self, ui: &mut egui::Ui) {
        let mut changed = false;

        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            for part in &mut self.data {
                ui.add_space(15.0);
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(egui::RichText::new(&part.name).heading().color(egui::Color32::from_rgb(108, 91, 123)).strong());
                });
                ui.add_space(8.0);

                for item in &mut part.items {
                    let row_rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(ui.available_width(), 26.0));
                    let is_row_hovered = ui.rect_contains_pointer(row_rect);

                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        
                        let anim_val = ui.ctx().animate_bool(ui.id().with(&item.id), !item.is_folded);
                        let angle = anim_val * std::f32::consts::FRAC_PI_2; 
                        let arrow_tint = if is_row_hovered { egui::Color32::from_gray(160) } else { egui::Color32::TRANSPARENT };
                        let arrow_img = egui::Image::from_bytes("bytes://arrow.png", ui::IMG_ARROW).fit_to_exact_size(egui::vec2(ICON_ARROW, ICON_ARROW)).rotate(angle, egui::vec2(0.5, 0.5)).tint(arrow_tint);
                        let arrow_resp = ui.add(egui::ImageButton::new(arrow_img).frame(false)).on_hover_cursor(egui::CursorIcon::PointingHand);
                        
                        if arrow_resp.clicked() { item.is_folded = !item.is_folded; changed = true; }

                        ui.vertical(|ui| {
                            let tip_text = egui::RichText::new(&item.tip).monospace().size(FONT_SIZE_TIP);
                            let frame = egui::Frame::none().inner_margin(egui::Margin::symmetric(6.0, 3.0)).rounding(6.0);
                            let tip_resp = frame.show(ui, |ui| ui.label(tip_text)).response.interact(egui::Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);

                            if tip_resp.hovered() {
                                ui.painter().rect_filled(tip_resp.rect, 6.0, egui::Color32::from_rgb(235, 228, 235));
                                ui.painter().text(tip_resp.rect.left_top() + egui::vec2(6.0, 3.0), egui::Align2::LEFT_TOP, &item.tip, egui::FontId::monospace(FONT_SIZE_TIP), egui::Color32::from_rgb(88, 110, 117));
                            }

                            if tip_resp.clicked() { ui.output_mut(|o| o.copied_text = item.tip.clone()); }

                            if !item.is_folded && !item.hint.is_empty() {
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new(&item.hint).color(egui::Color32::from_rgb(140, 135, 130)).size(FONT_SIZE_HINT_READ));
                            }
                        });
                    });
                    ui.add_space(8.0);
                }
            }
            ui.add_space(20.0);
        });

        if changed { self.save_state(); }
    }

    fn draw_edit_mode(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let mut changed = false;
        let mut delete_action = None;

        let mut current_edit = self.edit_target.clone();
        let mut current_del = self.delete_confirm.clone();

        let pointer_pos = ctx.pointer_interact_pos();
        let mut part_swap = None;
        let mut item_swap = None;

        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
            for (p_idx, part) in self.data.iter_mut().enumerate() {
                let is_dragging_this_part = self.dragging_part == Some(p_idx);
                let mut part_frame = egui::Frame::none().rounding(8.0);
                if is_dragging_this_part {
                    part_frame = part_frame.fill(egui::Color32::from_black_alpha(15));
                }
                
                part_frame.show(ui, |ui| {
                    ui.add_space(10.0);
                    let part_start_pos = ui.cursor().min;

                    ui.horizontal(|ui| {
                        ui.add_space(15.0);
                        
                        let drag_resp = ui::drag_handle(ui, ICON_PART_DRAG);
                        if drag_resp.drag_started() { self.dragging_part = Some(p_idx); }
                        
                        ui.add_space(5.0);
                        
                        if current_edit.as_deref() == Some(&part.id) {
                            let res = ui.add(egui::TextEdit::singleline(&mut part.name).desired_width(INPUT_WIDTH_PART).margin(egui::vec2(8.0, 4.0)));
                            if self.focus_requested_id.as_deref() == Some(&part.id) {
                                res.request_focus();
                                self.focus_requested_id = None;
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::Enter)) { current_edit = None; changed = true; }
                        } else {
                            ui.label(egui::RichText::new(&part.name).heading().color(egui::Color32::from_rgb(108, 91, 123)).strong());
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(20.0);
                            if ui::icon_btn(ui, "bytes://plus.png", ui::IMG_PLUS, ICON_PART_ACTION).clicked() {
                                let new_id = Uuid::new_v4().to_string();
                                part.items.push(ListItem { id: new_id.clone(), tip: String::new(), hint: String::new(), is_folded: false });
                                current_edit = Some(new_id.clone());
                                self.focus_requested_id = Some(new_id);
                                changed = true;
                            }
                            ui.add_space(10.0);
                            if ui::icon_btn(ui, "bytes://edit.png", ui::IMG_EDIT, ICON_PART_ACTION).clicked() { 
                                current_edit = Some(part.id.clone()); 
                                self.focus_requested_id = Some(part.id.clone()); 
                            }
                            ui.add_space(10.0);
                            if ui::icon_btn(ui, "bytes://delete.png", ui::IMG_DELETE, ICON_PART_ACTION).clicked() { current_del = Some(part.id.clone()); }
                        });
                    });

                    let part_rect = egui::Rect::from_min_max(part_start_pos, egui::pos2(ui.max_rect().right(), ui.cursor().min.y));
                    if let (Some(drag_idx), Some(pos)) = (self.dragging_part, pointer_pos) {
                        if drag_idx != p_idx && part_rect.contains(pos) { part_swap = Some((drag_idx, p_idx)); }
                    }

                    if current_del.as_deref() == Some(&part.id) {
                        ui.horizontal(|ui| {
                            ui.add_space(45.0);
                            ui.label(egui::RichText::new("确定删除?").color(egui::Color32::RED));
                            if ui.add(egui::Button::new("是").frame(false)).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { delete_action = Some(part.id.clone()); current_del = None; }
                            if ui.add(egui::Button::new("否").frame(false)).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { current_del = None; }
                        });
                    }
                    ui.add_space(5.0);

                    for (i_idx, item) in part.items.iter_mut().enumerate() {
                        let is_dragging_this_item = self.dragging_item.as_ref() == Some(&(part.id.clone(), i_idx));
                        let mut item_frame = egui::Frame::none().rounding(6.0);
                        if is_dragging_this_item {
                            item_frame = item_frame.fill(egui::Color32::from_black_alpha(15));
                        }

                        item_frame.show(ui, |ui| {
                            let item_start_pos = ui.cursor().min;
                            
                            ui.horizontal(|ui| {
                                ui.add_space(35.0);
                                
                                let drag_resp = ui::drag_handle(ui, ICON_ITEM_DRAG);
                                if drag_resp.drag_started() { self.dragging_item = Some((part.id.clone(), i_idx)); }

                                ui.add_space(5.0);

                                ui.vertical(|ui| {
                                    if current_edit.as_deref() == Some(&item.id) {
                                        let res1 = ui.add(egui::TextEdit::singleline(&mut item.tip).hint_text("输入 Tip").desired_width(INPUT_WIDTH_ITEM).margin(egui::vec2(6.0, 4.0)));
                                        if self.focus_requested_id.as_deref() == Some(&item.id) {
                                            res1.request_focus();
                                            self.focus_requested_id = None;
                                        }
                                        ui.add_space(2.0);
                                        ui.add(egui::TextEdit::singleline(&mut item.hint).hint_text("输入提示说明").desired_width(INPUT_WIDTH_ITEM).margin(egui::vec2(6.0, 4.0)));
                                        
                                        if ui.input(|i| i.key_pressed(egui::Key::Enter)) { current_edit = None; changed = true; }
                                    } else {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new(&item.tip).monospace());
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.add_space(20.0);
                                                if ui::icon_btn(ui, "bytes://edit.png", ui::IMG_EDIT, ICON_ITEM_ACTION).clicked() { 
                                                    current_edit = Some(item.id.clone()); 
                                                    self.focus_requested_id = Some(item.id.clone()); 
                                                }
                                                ui.add_space(10.0);
                                                if ui::icon_btn(ui, "bytes://delete.png", ui::IMG_DELETE, ICON_ITEM_ACTION).clicked() { current_del = Some(item.id.clone()); }
                                            });
                                        });
                                        ui.add_space(2.0);
                                        ui.label(egui::RichText::new(&item.hint).color(egui::Color32::from_rgb(140, 135, 130)).size(FONT_SIZE_HINT_EDIT));
                                    }

                                    if current_del.as_deref() == Some(&item.id) {
                                        ui.horizontal(|ui| {
                                            ui.label(egui::RichText::new("删除此项?").color(egui::Color32::RED));
                                            if ui.add(egui::Button::new("是").frame(false)).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { delete_action = Some(item.id.clone()); current_del = None; }
                                            if ui.add(egui::Button::new("否").frame(false)).on_hover_cursor(egui::CursorIcon::PointingHand).clicked() { current_del = None; }
                                        });
                                    }
                                });
                            });
                            ui.add_space(10.0);
                            
                            let item_rect = egui::Rect::from_min_max(item_start_pos, egui::pos2(ui.max_rect().right(), ui.cursor().min.y));
                            if let (Some((drag_p_id, drag_idx)), Some(pos)) = (&self.dragging_item, pointer_pos) {
                                if drag_p_id == &part.id && *drag_idx != i_idx && item_rect.contains(pos) { item_swap = Some((part.id.clone(), *drag_idx, i_idx)); }
                            }
                        });
                    }
                });
            }
            ui.add_space(20.0);
        });

        if let Some((from, to)) = part_swap {
            self.data.swap(from, to);
            self.dragging_part = Some(to); 
            changed = true;
        }
        if let Some((pid, from, to)) = item_swap {
            if let Some(p) = self.data.iter_mut().find(|p| p.id == pid) {
                p.items.swap(from, to);
                self.dragging_item = Some((pid, to));
                changed = true;
            }
        }

        self.edit_target = current_edit;
        self.delete_confirm = current_del;

        if let Some(del_id) = delete_action {
            self.data.retain(|p| p.id != del_id);
            for p in &mut self.data { p.items.retain(|i| i.id != del_id); }
            changed = true;
        }

        if changed { self.save_state(); }
    }

    fn handle_hotkeys(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::E)) { self.mode = Mode::Edit; self.edit_target = None; self.save_state(); }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::R)) { self.mode = Mode::Read; self.edit_target = None; self.save_state(); }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::W)) { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) { ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true)); }
    }
}

impl eframe::App for TipBoardApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] { egui::Rgba::TRANSPARENT.to_array() }
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !ctx.input(|i| i.pointer.primary_down()) {
            if self.dragging_part.is_some() || self.dragging_item.is_some() {
                self.dragging_part = None;
                self.dragging_item = None;
                self.save_state();
            }
        }

        self.handle_hotkeys(ctx);

        if ctx.input(|i| i.pointer.any_click()) {
            if ctx.pointer_interact_pos().is_some() {
                let is_clicking_text_edit = ctx.memory(|m| m.focused().is_some());
                if !is_clicking_text_edit {
                    self.edit_target = None;
                    self.save_state();
                }
            }
        }

        let main_frame = egui::Frame::none().fill(egui::Color32::from_rgb(253, 246, 227)).rounding(12.0).inner_margin(0.0);

        egui::CentralPanel::default().frame(main_frame).show(ctx, |ui| {
            self.draw_top_bar(ui, ctx);
            if self.mode == Mode::Read { self.draw_read_mode(ui); } else { self.draw_edit_mode(ui, ctx); }
        });
    }
}
