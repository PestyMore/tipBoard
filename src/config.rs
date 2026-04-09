// ==========================================
// tipBoard 全局 UI 尺寸与样式配置
// 在这里修改数字，即可全局改变软件的各个大小
// ==========================================

// --- 1. 窗口整体尺寸 ---
pub const WINDOW_WIDTH: f32 = 450.0;     // 窗口初始宽度
pub const WINDOW_HEIGHT: f32 = 700.0;    // 窗口初始高度

// --- 2. 核心字体大小 ---
pub const FONT_SIZE_HEADING: f32 = 22.0;   // 分区名称的大标题字号
pub const FONT_SIZE_BODY: f32 = 18.0;      // 基础正文字号 (包含顶部标题、按钮等)
pub const FONT_SIZE_TIP: f32 = 18.0;       // 列表项代码/命令 (Tip) 字号
pub const FONT_SIZE_HINT_READ: f32 = 16.0; // 阅读模式下的提示说明 (Hint) 字号
pub const FONT_SIZE_HINT_EDIT: f32 = 15.0; // 编辑模式下的提示说明 (Hint) 字号

// --- 3. 顶部导航栏配置 ---
pub const TOP_BAR_HEIGHT: f32 = 44.0;         // 顶部可拖拽标题栏的高度
pub const WINDOW_CTRL_BTN_SIZE: f32 = 14.0;   // 左上角“红黄绿”三个小圆圈的占位大小
pub const WINDOW_CTRL_BTN_RADIUS: f32 = 7.0;  // “红黄绿”小圆圈的实际绘制半径

// --- 4. 图标尺寸配置 ---
// 右上角系统级图标
pub const ICON_MODE_SWITCH: f32 = 24.0; // 右上角 铅笔/书本(切换模式) 图标大小
pub const ICON_TOP_PLUS: f32 = 20.0;    // 右上角 加号(新增分区) 图标大小

// 分区 (Part) 层级的图标
pub const ICON_PART_DRAG: f32 = 16.0;   // 分区名称前的 三条杠(拖拽) 图标大小
pub const ICON_PART_ACTION: f32 = 16.0; // 分区名称后的 加号/编辑/删除 图标大小

// 列表项 (Item) 层级的图标
pub const ICON_ITEM_DRAG: f32 = 14.0;   // 列表项前的 三条杠(拖拽) 图标大小
pub const ICON_ITEM_ACTION: f32 = 14.0; // 列表项后的 编辑/删除 图标大小
pub const ICON_ARROW: f32 = 12.0;       // 阅读模式下，最左侧折叠/展开的 箭头 大小

// --- 5. 输入框宽度配置 ---
// (当字号变大时，如果输入框太窄会导致字显示不全，可在这里调宽)
pub const INPUT_WIDTH_PART: f32 = 210.0; // 编辑模式下，分区标题输入框的宽度
pub const INPUT_WIDTH_ITEM: f32 = 230.0; // 编辑模式下，提示命令输入框的宽度
