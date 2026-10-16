use super::*;

fn choice_style(selected: bool, palette: &Palette) -> Style {
    if selected {
        Style::default()
            .fg(contrast(palette))
            .bg(palette.accent)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(palette.text).bg(palette.panel_bg)
    }
}

fn draw_choice(
    buffer: &mut Buffer,
    rect: Rect,
    label: &str,
    selected: bool,
    current: bool,
    palette: &Palette,
) {
    let style = choice_style(selected, palette);
    buffer.set_style(rect, style);
    let marker = if selected { "▸" } else { " " };
    let current = if current { " ✓" } else { "" };
    put_text(
        buffer,
        rect.x,
        rect.y,
        rect.width,
        &format!(" {marker} {}{current}", crate::ui::bilingual(label)),
        style,
    );
}

pub(super) fn render_settings_overlay(
    buffer: &mut Buffer,
    settings: &ClientSettingsOverlay,
    integration_updates_available: bool,
    palette: &Palette,
) -> Option<OverlayRender> {
    let integration_height = 14u16
        .saturating_add(settings.integrations.len().max(1) as u16)
        .saturating_add(settings.integration_messages.len().min(6) as u16);
    let height = if settings.section == ClientSettingsSection::Integrations {
        integration_height.max(22)
    } else {
        22
    };
    let popup = popup(buffer.area, 100, height)?;
    let inner = panel(buffer, popup, palette.accent, palette.panel_bg)?;
    if inner.width < 20 || inner.height < 8 {
        return None;
    }

    put_text(
        buffer,
        inner.x,
        inner.y,
        inner.width,
        " settings(設定)",
        Style::default()
            .fg(palette.text)
            .bg(palette.panel_bg)
            .add_modifier(Modifier::BOLD),
    );

    let integration_badge = integration_updates_available
        || settings
            .integrations
            .iter()
            .any(|integration| integration.state == crate::api::schema::IntegrationState::Outdated);
    let mut tab_x = inner.x;
    let mut tab_hits = Vec::new();
    for section in ClientSettingsSection::ALL {
        let badge = *section == ClientSettingsSection::Integrations && integration_badge;
        let label = if badge {
            format!(" ● {} ", section.label())
        } else {
            format!(" {} ", section.label())
        };
        let width = display_width(&label).min(inner.right().saturating_sub(tab_x));
        let rect = Rect::new(tab_x, inner.y + 1, width, 1);
        let active = *section == settings.section;
        let style = if active {
            Style::default()
                .fg(contrast(palette))
                .bg(palette.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette.overlay1).bg(palette.panel_bg)
        };
        buffer.set_style(rect, style);
        put_text(buffer, rect.x, rect.y, rect.width, &label, style);
        if badge && !active {
            put_text(
                buffer,
                rect.x.saturating_add(1),
                rect.y,
                rect.width.saturating_sub(1).min(2),
                "● ",
                Style::default()
                    .fg(palette.accent)
                    .bg(palette.panel_bg)
                    .add_modifier(Modifier::BOLD),
            );
        }
        tab_hits.push((rect, *section));
        tab_x = tab_x.saturating_add(width.saturating_add(1));
        if tab_x >= inner.right() {
            break;
        }
    }
    put_text(
        buffer,
        inner.x,
        inner.y + 2,
        inner.width,
        &"─".repeat(inner.width as usize),
        Style::default().fg(palette.surface0).bg(palette.panel_bg),
    );

    let content = Rect::new(
        inner.x,
        inner.y + 4,
        inner.width,
        inner.height.saturating_sub(8),
    );
    let mut choice_hits = Vec::new();
    let content_width = match settings.section {
        ClientSettingsSection::Theme => {
            let visible = usize::from(content.height);
            let scroll = settings.selected.saturating_sub(visible.saturating_sub(1));
            for (visible_index, (index, name)) in crate::config::THEME_NAMES
                .iter()
                .enumerate()
                .skip(scroll)
                .take(visible)
                .enumerate()
            {
                let rect = Rect::new(
                    content.x,
                    content.y + visible_index as u16,
                    content.width,
                    1,
                );
                draw_choice(
                    buffer,
                    rect,
                    name,
                    index == settings.selected,
                    super::super::settings::normalized_theme_name(name)
                        == super::super::settings::normalized_theme_name(
                            &settings.original_theme_name,
                        ),
                    palette,
                );
                choice_hits.push((rect, index));
            }
            usize::from(content.width)
        }
        ClientSettingsSection::Indicators => {
            render_choice_section(
                buffer,
                content,
                "agent status indicators(代理程式狀態指示器)",
                "choose color dots or distinct symbols for each state(選擇彩色圓點或不同符號表示各狀態)",
                &["color dots  ● ● ● ○ ·", "distinct symbols  × ◐ ✓ ○ ·"],
                settings.selected,
                palette,
                &mut choice_hits,
                settings.horizontal_scroll,
            )
        }
        ClientSettingsSection::Sound => {
            render_choice_section(
                buffer,
                content,
                "sound alerts(音效提醒)",
                "play sounds when agents change state in background(代理程式在背景變更狀態時播放音效)",
                &["on", "off"],
                settings.selected,
                palette,
                &mut choice_hits,
                settings.horizontal_scroll,
            )
        }
        ClientSettingsSection::Toast => {
            render_choice_section(
                buffer,
                content,
                "notification popups(彈出通知)",
                "choose where background popup notifications should appear(選擇背景彈出通知的顯示位置)",
                &["off", "inside herdr", "via terminal", "via system"],
                settings.selected,
                palette,
                &mut choice_hits,
                settings.horizontal_scroll,
            )
        }
        ClientSettingsSection::Integrations => {
            render_integrations(buffer, content, settings, palette)
        }
    };

    let max_horizontal_scroll = content_width.saturating_sub(usize::from(content.width));
    let scroll_y = inner.bottom().saturating_sub(3);
    let (scroll_left, scroll_track, scroll_right) = if max_horizontal_scroll > 0 {
        let left = Rect::new(inner.x, scroll_y, 2, 1);
        let right = Rect::new(inner.right().saturating_sub(2), scroll_y, 2, 1);
        let track = Rect::new(left.right(), scroll_y, inner.width.saturating_sub(4), 1);
        put_text(
            buffer,
            left.x,
            left.y,
            left.width,
            "◀ ",
            choice_style(false, palette),
        );
        put_text(
            buffer,
            track.x,
            track.y,
            track.width,
            &"─".repeat(usize::from(track.width)),
            choice_style(false, palette),
        );
        let denominator = usize::from(track.width.saturating_sub(1).max(1));
        let thumb = settings
            .horizontal_scroll
            .min(max_horizontal_scroll)
            .saturating_mul(denominator)
            / max_horizontal_scroll;
        put_text(
            buffer,
            track.x + u16::try_from(thumb).unwrap_or(track.width.saturating_sub(1)),
            track.y,
            1,
            "●",
            Style::default().fg(palette.accent).bg(palette.panel_bg),
        );
        put_text(
            buffer,
            right.x,
            right.y,
            right.width,
            " ▶",
            choice_style(false, palette),
        );
        (left, track, right)
    } else {
        (Rect::default(), Rect::default(), Rect::default())
    };

    let installable = settings
        .integrations
        .iter()
        .any(super::super::settings::integration_needs_install);
    let show_primary = settings.section != ClientSettingsSection::Integrations || installable;
    let labels = if show_primary { vec![18, 18] } else { vec![18] };
    let buttons = row(inner, &labels, 2, inner.height.saturating_sub(1));
    let (primary, close) = if show_primary {
        let primary = buttons[0];
        button(
            buffer,
            primary,
            if settings.section == ClientSettingsSection::Integrations {
                " ↵ install(安裝) "
            } else {
                " ↵ apply(套用) "
            },
            Style::default()
                .fg(contrast(palette))
                .bg(palette.accent)
                .add_modifier(Modifier::BOLD),
        );
        (primary, buttons[1])
    } else {
        (Rect::default(), buttons[0])
    };
    button(
        buffer,
        close,
        " esc close(關閉) ",
        Style::default()
            .fg(palette.text)
            .bg(palette.surface0)
            .add_modifier(Modifier::BOLD),
    );
    put_text(
        buffer,
        inner.x,
        inner.bottom().saturating_sub(2),
        inner.width,
        " ↑↓ select(選取)  tab section(切換區段)",
        Style::default().fg(palette.overlay1).bg(palette.panel_bg),
    );

    Some(OverlayRender {
        area: popup,
        primary,
        cancel: close,
        settings_popup: popup,
        settings_tabs: tab_hits,
        settings_choices: choice_hits,
        settings_scroll_left: scroll_left,
        settings_scroll_track: scroll_track,
        settings_scroll_right: scroll_right,
        settings_max_horizontal_scroll: max_horizontal_scroll,
        ..OverlayRender::default()
    })
}

fn render_choice_section(
    buffer: &mut Buffer,
    area: Rect,
    title: &str,
    description: &str,
    choices: &[&str],
    selected: usize,
    palette: &Palette,
    hits: &mut Vec<(Rect, usize)>,
    horizontal_scroll: usize,
) -> usize {
    put_text(
        buffer,
        area.x,
        area.y,
        area.width,
        title,
        Style::default()
            .fg(palette.text)
            .bg(palette.panel_bg)
            .add_modifier(Modifier::BOLD),
    );
    render_scrolled_line(
        buffer,
        Rect::new(area.x, area.y + 1, area.width, 1),
        description,
        horizontal_scroll,
        Style::default().fg(palette.overlay1).bg(palette.panel_bg),
    );
    let row_gap = u16::from(choices.len() > 2);
    for (index, choice) in choices.iter().enumerate() {
        let y = area.y + 3 + index as u16 * (1 + row_gap);
        if y >= area.bottom() {
            break;
        }
        let rect = Rect::new(area.x, y, area.width, 1);
        draw_choice(buffer, rect, choice, index == selected, false, palette);
        hits.push((rect, index));
    }
    display_width(description) as usize
}

fn render_integrations(
    buffer: &mut Buffer,
    area: Rect,
    settings: &ClientSettingsOverlay,
    palette: &Palette,
) -> usize {
    put_text(
        buffer,
        area.x,
        area.y,
        area.width,
        "agent integrations(代理程式整合)",
        Style::default()
            .fg(palette.text)
            .bg(palette.panel_bg)
            .add_modifier(Modifier::BOLD),
    );
    let description = "enable session restore and, where supported, direct status updates(啟用工作階段還原，並在支援的情況下直接接收狀態更新)";
    render_scrolled_line(
        buffer,
        Rect::new(area.x, area.y + 1, area.width, 1),
        description,
        settings.horizontal_scroll,
        Style::default().fg(palette.overlay1).bg(palette.panel_bg),
    );
    if settings.loading_integrations {
        put_text(
            buffer,
            area.x,
            area.y + 3,
            area.width,
            " loading integrations…(正在載入整合……)",
            Style::default().fg(palette.overlay1).bg(palette.panel_bg),
        );
        return display_width(description) as usize;
    }
    if settings.integrations.is_empty() {
        put_text(
            buffer,
            area.x,
            area.y + 3,
            area.width,
            " no integration targets available(沒有可用的整合目標)",
            Style::default().fg(palette.overlay1).bg(palette.panel_bg),
        );
        return display_width(description) as usize;
    }
    for (index, integration) in settings.integrations.iter().enumerate() {
        let y = area.y + 3 + index as u16;
        if y >= area.bottom() {
            break;
        }
        let (marker, color, status) = match integration.state {
            crate::api::schema::IntegrationState::Current => {
                ("✓", palette.green, crate::ui::bilingual("installed"))
            }
            crate::api::schema::IntegrationState::Outdated => (
                "↻",
                palette.yellow,
                crate::ui::bilingual("update available"),
            ),
            crate::api::schema::IntegrationState::NotInstalled if integration.available => {
                ("+", palette.accent, crate::ui::bilingual("available"))
            }
            crate::api::schema::IntegrationState::NotInstalled => {
                ("–", palette.overlay0, crate::ui::bilingual("not found"))
            }
        };
        put_text(
            buffer,
            area.x,
            y,
            3,
            &format!(" {marker}"),
            Style::default().fg(color).bg(palette.panel_bg),
        );
        put_text(
            buffer,
            area.x + 3,
            y,
            11.min(area.width.saturating_sub(3)),
            &format!("{:<9}", integration.label),
            Style::default().fg(palette.subtext0).bg(palette.panel_bg),
        );
        put_text(
            buffer,
            area.x + 14,
            y,
            area.width.saturating_sub(14),
            status,
            Style::default().fg(palette.overlay1).bg(palette.panel_bg),
        );
    }
    let message_y = area
        .y
        .saturating_add(4)
        .saturating_add(settings.integrations.len() as u16);
    for (offset, message) in settings.integration_messages.iter().take(6).enumerate() {
        let y = message_y.saturating_add(offset as u16);
        if y >= area.bottom() {
            break;
        }
        render_scrolled_line(
            buffer,
            Rect::new(area.x, y, area.width, 1),
            message,
            settings.horizontal_scroll,
            Style::default().fg(palette.overlay1).bg(palette.panel_bg),
        );
    }
    if settings.installing_integrations && message_y < area.bottom() {
        put_text(
            buffer,
            area.x,
            message_y,
            area.width,
            " installing…(正在安裝……)",
            Style::default().fg(palette.overlay1).bg(palette.panel_bg),
        );
    }
    settings
        .integration_messages
        .iter()
        .map(|message| display_width(message) as usize)
        .chain(std::iter::once(display_width(description) as usize))
        .max()
        .unwrap_or(usize::from(area.width))
}

fn render_scrolled_line(
    buffer: &mut Buffer,
    area: Rect,
    text: &str,
    horizontal_scroll: usize,
    style: Style,
) {
    ratatui::widgets::Widget::render(
        ratatui::widgets::Paragraph::new(ratatui::text::Line::styled(text.to_owned(), style))
            .scroll((0, u16::try_from(horizontal_scroll).unwrap_or(u16::MAX))),
        area,
        buffer,
    );
}
