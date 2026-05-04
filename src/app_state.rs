// =============================================
// App state: структура данных и сообщения
// =============================================

use typst_bake::{IntoDict, IntoValue};

// =============================================
// Режим печати: 1 или 2 листа
// =============================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SheetsMode {
    One,
    Two,
}

impl SheetsMode {
    pub fn sheets_count(self) -> i32 {
        match self {
            SheetsMode::One => 1,
            SheetsMode::Two => 2,
        }
    }

    pub fn is_two(self) -> bool {
        matches!(self, SheetsMode::Two)
    }
}

// =============================================
// Глобальное состояние приложения
// =============================================
#[derive(Debug)]
pub struct App {
    pub start_team: i32,
    pub finish_team: i32,
    pub cols: i32,
    pub rows: i32,
    pub landscape: bool,
    pub location: String,
    pub team_prefix: String,
    pub font_name: String,
    pub available_fonts: Vec<String>,
    pub selected_design: String,
    pub available_designs: Vec<String>,
    pub sheets_mode: SheetsMode,

    pub logo_path: String,
    pub team_names: Vec<String>,
    pub teams_file_path: String,

    pub dark_theme: bool,
    pub status_message: String,
    pub svg_pages: Vec<String>,
}

// =============================================
// Входы Typst (попадают в sys.inputs)
// =============================================
#[derive(Clone, Debug, IntoValue, IntoDict)]
pub struct TypstInputs {
    pub start_team: i32,
    pub finish_team: i32,
    pub cols: i32,
    pub rows: i32,
    pub landscape: bool,
    pub location: String,
    pub team_prefix: String,
    pub font_name: String,
    pub logo_path: String,
    pub team_names: Vec<String>,
    pub sheets_per_team: i32,
    pub rows_first_sheet: i32,
    pub rows_second_sheet: i32,
}

impl App {
    // =============================================
    // Деление строк между 1-м и 2-м листом
    // =============================================
    pub fn sheet_rows(&self) -> (i32, i32) {
        if self.sheets_mode == SheetsMode::Two {
            let adjusted_rows = self.rows.max(2);
            let first = (adjusted_rows + 1) / 2;
            let second = adjusted_rows / 2;
            (first, second)
        } else {
            (self.rows.max(1), 0)
        }
    }

    // =============================================
    // Конвертация состояния в TypstInputs
    // =============================================
    pub fn to_inputs(&self) -> TypstInputs {
        let (rows_first_sheet, rows_second_sheet) = self.sheet_rows();
        TypstInputs {
            start_team: self.start_team,
            finish_team: self.finish_team,
            cols: self.cols,
            rows: self.rows,
            landscape: self.landscape,
            location: self.location.clone(),
            team_prefix: self.team_prefix.clone(),
            font_name: self.font_name.clone(),
            logo_path: self.logo_path.clone(),
            team_names: self.team_names.clone(),
            sheets_per_team: self.sheets_mode.sheets_count(),
            rows_first_sheet,
            rows_second_sheet,
        }
    }
}

// =============================================
// Сообщения (события UI и async)
// =============================================
#[derive(Clone, Debug)]
pub enum Message {
    StartTeamChanged(i32),
    FinishTeamChanged(i32),
    ColsChanged(i32),
    RowsChanged(i32),
    LandscapeChanged(bool),
    LocationChanged(String),
    TeamPrefixChanged(String),
    FontChanged(String),
    DesignChanged(String),
    TwoSheetsChanged(bool),
    LogoPathChanged(String),
    DarkThemeChanged(bool),
    RefreshDesigns,

    PickLogoFile,
    LogoFilePicked(Option<String>),

    PickTeamsFile,
    TeamsFilePicked(Option<String>),
    TeamsLoaded(Result<(String, Vec<String>), String>),

    SavePdf,
    PdfPathPicked(Option<String>),
    PdfSaved(Result<String, String>),

    SvgLoaded(Result<Vec<String>, String>),
}
