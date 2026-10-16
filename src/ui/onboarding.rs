use ratatui::layout::Rect;

pub(crate) const ONBOARDING_TITLE: &str = "  herdr";
pub(crate) const ONBOARDING_SUBTITLE: &str =
    "  terminal workspace manager for coding agents(程式代理的終端工作區管理器)";
pub(crate) const ONBOARDING_DESCRIPTION: [&str; 3] = [
    "  this is a mouse-first terminal.(這是以滑鼠操作為優先的終端。)",
    "  click the sidebar to switch workspaces,(點擊側邊欄可切換工作區，)",
    "  drag borders to resize; right-click for menus.(拖曳邊框可調整大小；按右鍵可開啟選單。)",
];
// 這一行會串成 `<prefix> ... · ? ...`，英文加中文共 97 寬，
// 超過歡迎對話框的內寬 65 會被截斷；純中文僅 48 寬。
// 快捷鍵本身保持英文（使用者要照著按），說明用中文。
pub(crate) const ONBOARDING_PREFIX_SUFFIX: &str = " 進入前置鍵模式 · ";
pub(crate) const ONBOARDING_HELP_LABEL: &str = "?";
pub(crate) const ONBOARDING_HELP_SUFFIX: &str = " 顯示快捷鍵與設定";
pub(crate) const ONBOARDING_NEXT: &str =
    "  next: install optional agent integrations(下一步：安裝選用的代理程式整合)";

pub(crate) fn onboarding_welcome_continue_rect(area: Rect) -> Rect {
    super::widgets::continue_button_rect(area)
}
