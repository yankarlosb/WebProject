//! Excel Export Module
//! 
//! Generates Excel files matching the Balance de Carga template format.
//! Structure:
//! - Single sheet per academic year with weekly distribution grid
//! - Includes regular weeks, Consultas (4 cols), and Exámenes Finales (5 cols)

use rust_xlsxwriter::{
    Workbook, Format, FormatAlign, FormatBorder, Color,
    XlsxError,
};
use chrono::{NaiveDate, Duration, Datelike, Weekday};
use serde_json::Value;

/// Activity types used in the balance
/// C - Conferencia, CP - Clase Práctica, S - Seminario, PL - Práctica de Laboratorio
/// TE - Tarea Extraclase, T - Taller, PP - Prueba Parcial, EC - Examen Comprobatorio
pub const ACTIVITY_TYPES: &[&str] = &["C", "CP", "S", "PL", "TE", "T", "PP", "EC"];

/// Critical activity types that should be colored red
/// These types cannot have 3 or more in the same week
pub const CRITICAL_TYPES: &[&str] = &["T", "TE", "PP"];

/// Data for a single fragment/subject in the balance
#[derive(Debug, Clone)]
pub struct FragmentExportData {
    pub name: String,
    pub hours: i32,
    /// Weekly data: week_number -> day_number -> activity_type (e.g., "C", "CP")
    pub weekly_data: Vec<Vec<Option<String>>>,
    /// Consultas data (4 cells)
    pub consultas_data: Vec<Option<String>>,
    /// Exámenes Finales data (5 cells)
    pub examenes_data: Vec<Option<String>>,
    /// Planned hours by activity type
    pub plan: ActivityPlan,
}

/// Planned hours for each activity type
#[derive(Debug, Clone, Default)]
pub struct ActivityPlan {
    pub c: i32,
    pub cp: i32,
    pub s: i32,
    pub pl: i32,
    pub te: i32,
    pub t: i32,
    pub pp: i32,
    pub ec: i32,
    pub tc: i32,
    pub ef: i32,
}

/// Balance export configuration
#[derive(Debug, Clone)]
pub struct BalanceExportConfig {
    pub academic_year: String,      // "1ro", "2do", "3ro", "4to"
    pub period: String,             // "1ero", "2do"
    pub academic_year_text: String, // "2025-2026"
    pub start_date: NaiveDate,
    pub weeks: i32,
    pub fragments: Vec<FragmentExportData>,
    /// Non-academic periods (vacations, etc.)
    pub non_academic_periods: Vec<(NaiveDate, NaiveDate, String)>,
}

/// Week date information for display
#[derive(Debug, Clone)]
struct WeekDateInfo {
    pub start_date: NaiveDate,
    #[allow(dead_code)] // May be used for display in future
    pub display_range: String,
}

// ============================================================================
// DATE CALCULATION HELPERS (matching frontend logic)
// ============================================================================

/// Check if a date falls within any non-academic period
fn is_in_non_academic_period(date: NaiveDate, periods: &[(NaiveDate, NaiveDate, String)]) -> bool {
    for (start, end, _) in periods {
        if date >= *start && date <= *end {
            return true;
        }
    }
    false
}

/// Check if a date is a weekend (Saturday or Sunday)
fn is_weekend(date: NaiveDate) -> bool {
    matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

/// Get the next working day (not weekend, not in non-academic period)
fn get_next_working_day(date: NaiveDate, non_academic_periods: &[(NaiveDate, NaiveDate, String)]) -> NaiveDate {
    let mut next = date;
    while is_weekend(next) || is_in_non_academic_period(next, non_academic_periods) {
        next += Duration::days(1);
    }
    next
}

/// Calculate week dates, skipping non-academic periods
/// This mirrors the frontend's calculateWeekDates function
fn calculate_week_dates(
    start_date: NaiveDate,
    total_weeks: i32,
    non_academic_periods: &[(NaiveDate, NaiveDate, String)],
) -> Vec<WeekDateInfo> {
    let mut weeks = Vec::with_capacity(total_weeks as usize);
    
    // Start from the first working day
    let mut current_date = get_next_working_day(start_date, non_academic_periods);
    
    for _ in 0..total_weeks {
        // Get the start of this week (ensure it's a working day)
        let week_start = get_next_working_day(current_date, non_academic_periods);
        
        // Find the end of the week (4 more working days for L, M, X, J, V = 5 days total)
        let mut working_days = 0;
        let mut week_end = week_start;
        
        while working_days < 4 {
            week_end += Duration::days(1);
            if !is_weekend(week_end) && !is_in_non_academic_period(week_end, non_academic_periods) {
                working_days += 1;
            }
        }
        
        // Format display range
        let display_range = format!(
            "{:02}/{:02} - {:02}/{:02}",
            week_start.day(), week_start.month(),
            week_end.day(), week_end.month()
        );
        
        weeks.push(WeekDateInfo {
            start_date: week_start,
            display_range,
        });
        
        // Advance to the next week
        current_date = week_end + Duration::days(1);
        current_date = get_next_working_day(current_date, non_academic_periods);
    }
    
    weeks
}

/// Calculate dates for final weeks (Consultas and Exámenes Finales)
/// These come after all regular lective weeks
fn calculate_final_weeks_dates(
    start_date: NaiveDate,
    lective_weeks: i32,
    non_academic_periods: &[(NaiveDate, NaiveDate, String)],
) -> Vec<WeekDateInfo> {
    // First calculate lective weeks to know where final weeks start
    let lective_week_dates = calculate_week_dates(start_date, lective_weeks, non_academic_periods);
    
    if lective_week_dates.is_empty() {
        return vec![];
    }
    
    // Get end of last lective week
    let last_week = &lective_week_dates[lective_week_dates.len() - 1];
    let last_week_end = last_week.start_date + Duration::days(4); // Approximate end
    
    let mut final_weeks = Vec::with_capacity(2);
    let mut current_date = last_week_end + Duration::days(1);
    current_date = get_next_working_day(current_date, non_academic_periods);
    
    // Generate 2 final weeks (Consultas and Exámenes)
    for _ in 0..2 {
        let week_start = get_next_working_day(current_date, non_academic_periods);
        
        // Find end of week (4 more working days)
        let mut working_days = 0;
        let mut week_end = week_start;
        
        while working_days < 4 {
            week_end += Duration::days(1);
            if !is_weekend(week_end) && !is_in_non_academic_period(week_end, non_academic_periods) {
                working_days += 1;
            }
        }
        
        let display_range = format!(
            "{:02}/{:02} - {:02}/{:02}",
            week_start.day(), week_start.month(),
            week_end.day(), week_end.month()
        );
        
        final_weeks.push(WeekDateInfo {
            start_date: week_start,
            display_range,
        });
        
        current_date = week_end + Duration::days(1);
        current_date = get_next_working_day(current_date, non_academic_periods);
    }
    
    final_weeks
}

/// Excel style definitions matching the template
struct ExcelStyles {
    header_bold: Format,
    header_week: Format,
    header_day: Format,
    header_day_week_end: Format,              // Day header at end of week (right border)
    cell_activity: Format,
    cell_activity_week_end: Format,           // Activity cell at end of week (right border)
    cell_activity_critical: Format,           // Critical types (T, TE, PP) - RED text
    cell_activity_critical_week_end: Format,  // Critical at end of week - RED text + right border
    cell_subject_name: Format,
}

impl ExcelStyles {
    fn new() -> Self {
        // Header style: Bold, centered, light gray background, medium borders
        let header_bold = Format::new()
            .set_bold()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_align(FormatAlign::VerticalCenter)
            .set_text_wrap()
            .set_background_color(Color::RGB(0xD9D9D9))
            .set_border(FormatBorder::Medium);

        // Week header: Bold, centered, gray background with thick black borders
        let header_week = Format::new()
            .set_bold()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_background_color(Color::RGB(0xD9D9D9))
            .set_border(FormatBorder::Medium)
            .set_border_color(Color::Black);

        // Day number header (normal)
        let header_day = Format::new()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin)
            .set_border_color(Color::Black);

        // Day number header at end of week (thick right border)
        let header_day_week_end = Format::new()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_border_top(FormatBorder::Thin)
            .set_border_bottom(FormatBorder::Thin)
            .set_border_left(FormatBorder::Thin)
            .set_border_right(FormatBorder::Medium)
            .set_border_color(Color::Black);

        // Activity cell (C, CP, S, etc.) - normal (black text)
        let cell_activity = Format::new()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin)
            .set_border_color(Color::Black);

        // Activity cell at end of week (thick right border, black text)
        let cell_activity_week_end = Format::new()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_border_top(FormatBorder::Thin)
            .set_border_bottom(FormatBorder::Thin)
            .set_border_left(FormatBorder::Thin)
            .set_border_right(FormatBorder::Medium)
            .set_border_color(Color::Black);

        // Critical activity cell (T, TE, PP) - RED background
        let cell_activity_critical = Format::new()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_background_color(Color::RGB(0xFF6B6B))  // Light red background
            .set_border(FormatBorder::Thin)
            .set_border_color(Color::Black);

        // Critical activity cell at end of week (thick right border, RED background)
        let cell_activity_critical_week_end = Format::new()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_align(FormatAlign::Center)
            .set_background_color(Color::RGB(0xFF6B6B))  // Light red background
            .set_border_top(FormatBorder::Thin)
            .set_border_bottom(FormatBorder::Thin)
            .set_border_left(FormatBorder::Thin)
            .set_border_right(FormatBorder::Medium)
            .set_border_color(Color::Black);

        // Subject name cell
        let cell_subject_name = Format::new()
            .set_font_name("Arial")
            .set_font_size(10)
            .set_border(FormatBorder::Thin)
            .set_border_color(Color::Black);

        Self {
            header_bold,
            header_week,
            header_day,
            header_day_week_end,
            cell_activity,
            cell_activity_week_end,
            cell_activity_critical,
            cell_activity_critical_week_end,
            cell_subject_name,
        }
    }
}

/// Check if an activity type is critical (should be colored red)
fn is_critical_type(activity: &str) -> bool {
    CRITICAL_TYPES.contains(&activity)
}

/// Generate an Excel workbook from balance data
pub fn generate_balance_excel(config: &BalanceExportConfig) -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    let styles = ExcelStyles::new();

    // Create data sheet for the academic year (no cover sheet needed)
    let sheet_name = format!("{}.ICS", config.academic_year);
    create_data_sheet(&mut workbook, &sheet_name, config, &styles)?;

    // Save to memory buffer
    let buffer = workbook.save_to_buffer()?;
    Ok(buffer)
}

/// Create a data sheet with the weekly distribution grid
fn create_data_sheet(
    workbook: &mut Workbook,
    sheet_name: &str,
    config: &BalanceExportConfig,
    styles: &ExcelStyles,
) -> Result<(), XlsxError> {
    let sheet = workbook.add_worksheet();
    sheet.set_name(sheet_name)?;

    let days_per_week = 4; // 4 data columns per week (matches frontend data structure)
    let consultas_cols = 4; // 4 columns for Consultas
    let examenes_cols = 5;  // 5 columns for Exámenes Finales
    let total_week_cols = (config.weeks as usize) * days_per_week;
    let total_cols = total_week_cols + consultas_cols + examenes_cols;

    // Set column widths
    sheet.set_column_width(0, 33)?; // Subject name column
    for col in 1..=total_cols {
        sheet.set_column_width(col as u16, 5)?;
    }

    // Calculate week dates (skipping non-academic periods)
    let week_dates = calculate_week_dates(
        config.start_date,
        config.weeks,
        &config.non_academic_periods,
    );

    // Calculate final weeks dates (Consultas and Exámenes)
    let final_week_dates = calculate_final_weeks_dates(
        config.start_date,
        config.weeks,
        &config.non_academic_periods,
    );

    // Spanish month abbreviations
    let month_names = ["Ene", "Feb", "Mar", "Abr", "May", "Jun", 
                       "Jul", "Ago", "Sep", "Oct", "Nov", "Dic"];

    // === ROW 0: "Asignaturas" header + date ranges ===
    sheet.write_with_format(0, 0, "Asignaturas", &styles.header_bold)?;
    
    // Regular weeks date ranges
    for (week_idx, week_info) in week_dates.iter().enumerate() {
        let col_start = 1 + (week_idx as u16) * (days_per_week as u16);
        let col_end = col_start + (days_per_week as u16) - 1;
        
        // Calculate week end date: Mon to Fri = +4 calendar days (5 day work week)
        let start_day = week_info.start_date.day();
        let end_date = week_info.start_date + chrono::Duration::days(4); // Mon + 4 = Fri
        let end_day = end_date.day();
        
        // Handle month boundary
        let start_month_idx = (week_info.start_date.month() - 1) as usize;
        let end_month_idx = (end_date.month() - 1) as usize;
        
        let date_range = if start_month_idx == end_month_idx {
            format!("{}-{} {}", start_day, end_day, month_names[start_month_idx])
        } else {
            format!("{} {}-{} {}", start_day, month_names[start_month_idx], end_day, month_names[end_month_idx])
        };
        
        sheet.merge_range(0, col_start, 0, col_end, &date_range, &styles.header_week)?;
    }

    // Consultas date range
    let consultas_col_start = 1 + (config.weeks as u16) * (days_per_week as u16);
    let consultas_col_end = consultas_col_start + (consultas_cols as u16) - 1;
    if let Some(consultas_info) = final_week_dates.get(0) {
        let start_day = consultas_info.start_date.day();
        let end_date = consultas_info.start_date + chrono::Duration::days(4);
        let end_day = end_date.day();
        let start_month_idx = (consultas_info.start_date.month() - 1) as usize;
        let end_month_idx = (end_date.month() - 1) as usize;
        
        let date_range = if start_month_idx == end_month_idx {
            format!("{}-{} {}", start_day, end_day, month_names[start_month_idx])
        } else {
            format!("{} {}-{} {}", start_day, month_names[start_month_idx], end_day, month_names[end_month_idx])
        };
        sheet.merge_range(0, consultas_col_start, 0, consultas_col_end, &date_range, &styles.header_week)?;
    }

    // Exámenes Finales date range
    let examenes_col_start = consultas_col_end + 1;
    let examenes_col_end = examenes_col_start + (examenes_cols as u16) - 1;
    if let Some(examenes_info) = final_week_dates.get(1) {
        let start_day = examenes_info.start_date.day();
        let end_date = examenes_info.start_date + chrono::Duration::days(4);
        let end_day = end_date.day();
        let start_month_idx = (examenes_info.start_date.month() - 1) as usize;
        let end_month_idx = (end_date.month() - 1) as usize;
        
        let date_range = if start_month_idx == end_month_idx {
            format!("{}-{} {}", start_day, end_day, month_names[start_month_idx])
        } else {
            format!("{} {}-{} {}", start_day, month_names[start_month_idx], end_day, month_names[end_month_idx])
        };
        sheet.merge_range(0, examenes_col_start, 0, examenes_col_end, &date_range, &styles.header_week)?;
    }

    // === ROW 1: Week labels ("Semana 1", ..., "Consultas", "Exámenes Finales") ===
    for week in 0..config.weeks {
        let col_start = 1 + (week as u16) * (days_per_week as u16);
        let col_end = col_start + (days_per_week as u16) - 1;
        let week_label = format!("Semana {}", week + 1);
        sheet.merge_range(1, col_start, 1, col_end, &week_label, &styles.header_week)?;
    }
    
    // Consultas label
    sheet.merge_range(1, consultas_col_start, 1, consultas_col_end, "Consultas", &styles.header_week)?;
    
    // Exámenes Finales label
    sheet.merge_range(1, examenes_col_start, 1, examenes_col_end, "Exámenes Finales", &styles.header_week)?;

    // === ROW 2: Day numbers (1, 2, 3, 4 repeating for weeks, 1-4 for consultas, 1-5 for exámenes) ===
    // Use thick right border on last day of each week for visual separation
    for week in 0..config.weeks {
        for day in 0..days_per_week {
            let col = 1 + (week as u16) * (days_per_week as u16) + (day as u16);
            let is_last_day = day == days_per_week - 1;
            let style = if is_last_day { &styles.header_day_week_end } else { &styles.header_day };
            sheet.write_with_format(2, col, (day + 1) as i32, style)?;
        }
    }
    
    // Consultas day numbers (1-4) - last one with thick border
    for day in 0..consultas_cols {
        let col = consultas_col_start + (day as u16);
        let is_last_day = day == consultas_cols - 1;
        let style = if is_last_day { &styles.header_day_week_end } else { &styles.header_day };
        sheet.write_with_format(2, col, (day + 1) as i32, style)?;
    }
    
    // Exámenes day numbers (1-5) - last one with thick border
    for day in 0..examenes_cols {
        let col = examenes_col_start + (day as u16);
        let is_last_day = day == examenes_cols - 1;
        let style = if is_last_day { &styles.header_day_week_end } else { &styles.header_day };
        sheet.write_with_format(2, col, (day + 1) as i32, style)?;
    }

    // === ROWS 3+: Subject data ===
    for (idx, fragment) in config.fragments.iter().enumerate() {
        let row = 3 + idx as u32;
        
        // Subject name
        sheet.write_with_format(row, 0, &fragment.name, &styles.cell_subject_name)?;
        
        // Weekly data (regular weeks) - use thick border on last day of each week
        // Critical types (T, TE, PP) are colored red
        for (week_idx, week_data) in fragment.weekly_data.iter().enumerate() {
            for (day_idx, activity) in week_data.iter().enumerate() {
                let col = 1 + (week_idx as u16) * (days_per_week as u16) + (day_idx as u16);
                let is_last_day = day_idx == days_per_week - 1;
                
                if let Some(act) = activity {
                    let is_critical = is_critical_type(act);
                    let style = match (is_last_day, is_critical) {
                        (true, true) => &styles.cell_activity_critical_week_end,
                        (true, false) => &styles.cell_activity_week_end,
                        (false, true) => &styles.cell_activity_critical,
                        (false, false) => &styles.cell_activity,
                    };
                    sheet.write_with_format(row, col, act, style)?;
                } else {
                    // Write empty cell with border
                    let style = if is_last_day { &styles.cell_activity_week_end } else { &styles.cell_activity };
                    sheet.write_with_format(row, col, "", style)?;
                }
            }
        }
        
        // Consultas data (4 cells after regular weeks)
        for day in 0..consultas_cols {
            let col = consultas_col_start + (day as u16);
            let is_last_day = day == consultas_cols - 1;
            
            if let Some(act) = &fragment.consultas_data[day] {
                let is_critical = is_critical_type(act);
                let style = match (is_last_day, is_critical) {
                    (true, true) => &styles.cell_activity_critical_week_end,
                    (true, false) => &styles.cell_activity_week_end,
                    (false, true) => &styles.cell_activity_critical,
                    (false, false) => &styles.cell_activity,
                };
                sheet.write_with_format(row, col, act, style)?;
            } else {
                let style = if is_last_day { &styles.cell_activity_week_end } else { &styles.cell_activity };
                sheet.write_with_format(row, col, "", style)?;
            }
        }
        
        // Exámenes Finales data (5 cells after consultas)
        for day in 0..examenes_cols {
            let col = examenes_col_start + (day as u16);
            let is_last_day = day == examenes_cols - 1;
            
            if let Some(act) = &fragment.examenes_data[day] {
                let is_critical = is_critical_type(act);
                let style = match (is_last_day, is_critical) {
                    (true, true) => &styles.cell_activity_critical_week_end,
                    (true, false) => &styles.cell_activity_week_end,
                    (false, true) => &styles.cell_activity_critical,
                    (false, false) => &styles.cell_activity,
                };
                sheet.write_with_format(row, col, act, style)?;
            } else {
                let style = if is_last_day { &styles.cell_activity_week_end } else { &styles.cell_activity };
                sheet.write_with_format(row, col, "", style)?;
            }
        }
    }

    Ok(())
}

/// Parse fragment JSON data into weekly structure
/// The JSON format is: { "values": ["C", "CP", "", "S", ...] } where every 4 items = 1 week
/// Index 0-3 = week 1, index 4-7 = week 2, etc.
/// After regular weeks: 4 cells for Consultas, then 5 cells for Exámenes Finales
pub fn parse_fragment_data(data: &Value, weeks: i32) -> Vec<Vec<Option<String>>> {
    let days_per_week = 4;
    let mut weekly_data = Vec::with_capacity(weeks as usize);

    // Get the values array from the data
    let values: Vec<String> = data
        .get("values")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect()
        })
        .unwrap_or_default();

    for week_num in 0..weeks as usize {
        let mut week_data = vec![None; days_per_week];
        
        for day in 0..days_per_week {
            let index = week_num * days_per_week + day;
            if let Some(activity) = values.get(index) {
                if !activity.is_empty() {
                    week_data[day] = Some(activity.clone());
                }
            }
        }

        weekly_data.push(week_data);
    }

    weekly_data
}

/// Parse Consultas data (4 cells after regular weeks)
pub fn parse_consultas_data(data: &Value, weeks: i32) -> Vec<Option<String>> {
    let days_per_week = 4;
    let consultas_start = (weeks as usize) * days_per_week;
    let consultas_cols = 4;
    
    let values: Vec<String> = data
        .get("values")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect()
        })
        .unwrap_or_default();

    let mut consultas = vec![None; consultas_cols];
    for i in 0..consultas_cols {
        let index = consultas_start + i;
        if let Some(activity) = values.get(index) {
            if !activity.is_empty() {
                consultas[i] = Some(activity.clone());
            }
        }
    }
    
    consultas
}

/// Parse Exámenes Finales data (5 cells after Consultas)
pub fn parse_examenes_data(data: &Value, weeks: i32) -> Vec<Option<String>> {
    let days_per_week = 4;
    let consultas_cols = 4;
    let examenes_start = (weeks as usize) * days_per_week + consultas_cols;
    let examenes_cols = 5;
    
    let values: Vec<String> = data
        .get("values")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect()
        })
        .unwrap_or_default();

    let mut examenes = vec![None; examenes_cols];
    for i in 0..examenes_cols {
        let index = examenes_start + i;
        if let Some(activity) = values.get(index) {
            if !activity.is_empty() {
                examenes[i] = Some(activity.clone());
            }
        }
    }
    
    examenes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fragment_data() {
        // New format: flat "values" array, 4 items per week
        let data = serde_json::json!({
            "values": ["C", "CP", "", "S", "C", "", "PL", ""]
        });

        let result = parse_fragment_data(&data, 2);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![Some("C".into()), Some("CP".into()), None, Some("S".into())]);
        assert_eq!(result[1], vec![Some("C".into()), None, Some("PL".into()), None]);
    }

    #[test]
    fn test_parse_fragment_data_empty() {
        let data = serde_json::json!({});
        let result = parse_fragment_data(&data, 2);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![None, None, None, None]);
        assert_eq!(result[1], vec![None, None, None, None]);
    }

    #[test]
    fn test_calculate_week_dates_no_gaps() {
        // Start on Monday Sep 1, 2025, no non-academic periods
        let start_date = NaiveDate::from_ymd_opt(2025, 9, 1).unwrap();
        let weeks = calculate_week_dates(start_date, 3, &[]);
        
        assert_eq!(weeks.len(), 3);
        assert_eq!(weeks[0].start_date, NaiveDate::from_ymd_opt(2025, 9, 1).unwrap());
        assert_eq!(weeks[1].start_date, NaiveDate::from_ymd_opt(2025, 9, 8).unwrap());
        assert_eq!(weeks[2].start_date, NaiveDate::from_ymd_opt(2025, 9, 15).unwrap());
    }

    #[test]
    fn test_calculate_week_dates_skips_weekend() {
        // Start on Saturday Sep 6, 2025 - should skip to Monday Sep 8
        let start_date = NaiveDate::from_ymd_opt(2025, 9, 6).unwrap();
        let weeks = calculate_week_dates(start_date, 2, &[]);
        
        assert_eq!(weeks.len(), 2);
        assert_eq!(weeks[0].start_date, NaiveDate::from_ymd_opt(2025, 9, 8).unwrap()); // Skipped to Monday
        assert_eq!(weeks[1].start_date, NaiveDate::from_ymd_opt(2025, 9, 15).unwrap());
    }

    #[test]
    fn test_calculate_week_dates_skips_non_academic() {
        // Start on Monday Sep 1, 2025
        // Non-academic period: Sep 8-14 (second week)
        let start_date = NaiveDate::from_ymd_opt(2025, 9, 1).unwrap();
        let non_academic = vec![
            (
                NaiveDate::from_ymd_opt(2025, 9, 8).unwrap(),
                NaiveDate::from_ymd_opt(2025, 9, 14).unwrap(),
                "Receso".to_string()
            )
        ];
        let weeks = calculate_week_dates(start_date, 3, &non_academic);
        
        assert_eq!(weeks.len(), 3);
        assert_eq!(weeks[0].start_date, NaiveDate::from_ymd_opt(2025, 9, 1).unwrap());
        // Week 2 should skip the non-academic period and land on Sep 15
        assert_eq!(weeks[1].start_date, NaiveDate::from_ymd_opt(2025, 9, 15).unwrap());
        assert_eq!(weeks[2].start_date, NaiveDate::from_ymd_opt(2025, 9, 22).unwrap());
    }
}
