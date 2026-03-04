
use ratatui::style::{Color, Modifier, Style};
use tui_markdown::StyleSheet;
use tui_markdown::{DefaultStyleSheet, Options, from_str_with_options};
use ratatui_themes::{Theme, ThemeName, ThemePalette};

#[derive(Debug, Clone)]
pub struct AppStyleSheet {
    palette: ThemePalette,
}

impl AppStyleSheet {
    pub fn new(theme: Theme) -> Self {
        Self { palette: theme.palette() }
    }
}

impl StyleSheet for AppStyleSheet {
    fn heading(&self, level: u8) -> Style {
        let orig_accent = Color::from_u32(0x009679bd);
        let accent = self.palette.accent;
        match level {
            1 => Style::new().fg(accent).reversed(),
            2 => Style::new().fg(accent).bold(),
            3 => Style::new().fg(accent),
            4 => Style::new().fg(accent).italic(),
            _ => Style::new().fg(accent),
        }
    }

    fn code(&self) -> Style {
        Style::new().fg(self.palette.fg)
    }

    fn link(&self) -> Style {
        Style::new().fg(self.palette.accent).underlined()
    }

    fn blockquote(&self) -> Style {
        Style::new().fg(self.palette.secondary)
    }

    fn heading_meta(&self) -> Style {
        Style::new().fg(self.palette.muted)
    }

    fn metadata_block(&self) -> Style {
        Style::new().fg(self.palette.info)
    }

    fn image_alt(&self) -> Style {
        Style::new().fg(self.palette.muted).italic()
    }

    fn table_header(&self) -> Style {
        Style::new().bold().fg(self.palette.secondary)
    }

    fn table_border(&self) -> Style {
        Style::new().fg(self.palette.muted)
    }
}

pub fn get_theme() -> Theme {
    Theme::new(ThemeName::OneDarkPro)
}

pub fn get_md_options() -> Options<AppStyleSheet> {
    Options::new(AppStyleSheet::new(get_theme()))
        .with_show_math_marks(false)
        .with_show_header_marks(false)
        .with_show_code_fence(false)
        .with_show_code_line_numbers(false)
}
