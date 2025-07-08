use serde::{Deserialize, Serialize};
use serde_json::Value;
use colored::Colorize;
use prettytable::{Table, row, cell};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Table,
    Pretty,
    Csv,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "table" => Ok(OutputFormat::Table),
            "pretty" => Ok(OutputFormat::Pretty),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(format!("Unknown output format: {s}")),
        }
    }
}

pub trait Formatter {
    fn format(&self, data: &Value) -> Result<String, Box<dyn std::error::Error>>;
}

pub fn create_formatter(format: OutputFormat) -> Box<dyn Formatter> {
    match format {
        OutputFormat::Json => Box::new(JsonFormatter),
        OutputFormat::Table => Box::new(TableFormatter),
        OutputFormat::Pretty => Box::new(PrettyFormatter),
        OutputFormat::Csv => Box::new(CsvFormatter),
    }
}

pub struct JsonFormatter;
pub struct TableFormatter;
pub struct PrettyFormatter;
pub struct CsvFormatter;

impl Formatter for JsonFormatter {
    fn format(&self, data: &Value) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(data)?)
    }
}

impl Formatter for PrettyFormatter {
    fn format(&self, data: &Value) -> Result<String, Box<dyn std::error::Error>> {
        match data {
            Value::Object(map) => {
                let mut output = String::new();
                
                // Special handling for APOD
                if let Some(title) = map.get("title").and_then(|v| v.as_str()) {
                    output.push_str(&format!("{}\n", title.bold().cyan()));
                    output.push_str(&"─".repeat(title.len()));
                    output.push('\n');
                }
                
                if let Some(date) = map.get("date").and_then(|v| v.as_str()) {
                    output.push_str(&format!("{}: {date}\n", "Date".green()));
                }
                
                if let Some(explanation) = map.get("explanation").and_then(|v| v.as_str()) {
                    output.push_str(&format!("\n{explanation}\n"));
                }
                
                if let Some(url) = map.get("url").and_then(|v| v.as_str()) {
                    output.push_str(&format!("\n{}: {}\n", "URL".yellow(), url.underline().blue()));
                }
                
                if let Some(hdurl) = map.get("hdurl").and_then(|v| v.as_str()) {
                    output.push_str(&format!("{}: {}\n", "HD URL".yellow(), hdurl.underline().blue()));
                }
                
                // Generic pretty printing for other fields
                for (key, value) in map {
                    if !["title", "date", "explanation", "url", "hdurl"].contains(&key.as_str()) {
                        let formatted_value = format_value(value);
                        output.push_str(&format!("{}: {formatted_value}\n", key.green()));
                    }
                }
                
                Ok(output)
            }
            Value::Array(arr) => {
                let mut output = String::new();
                let len = arr.len();
                output.push_str(&format!("{}\n", format!("Found {len} items").bold()));
                output.push_str(&"─".repeat(40));
                output.push('\n');
                
                for (i, item) in arr.iter().enumerate() {
                    let item_num = i + 1;
                    output.push_str(&format!("\n{} {}\n", "►".cyan(), format!("Item {item_num}").bold()));
                    output.push_str(&PrettyFormatter.format(item)?);
                    if i < arr.len() - 1 {
                        output.push('\n');
                    }
                }
                
                Ok(output)
            }
            _ => Ok(format_value(data))
        }
    }
}

impl Formatter for TableFormatter {
    fn format(&self, data: &Value) -> Result<String, Box<dyn std::error::Error>> {
        match data {
            Value::Array(arr) if !arr.is_empty() => {
                // Assume array of objects with same structure
                if let Some(Value::Object(first)) = arr.first() {
                    let mut table = Table::new();
                    
                    // Add headers
                    let headers: Vec<_> = first.keys().map(|k| cell!(k.bold())).collect();
                    table.add_row(headers.into());
                    
                    // Add data rows
                    for item in arr {
                        if let Value::Object(obj) = item {
                            let row: Vec<_> = first.keys()
                                .map(|k| cell!(format_table_value(obj.get(k).unwrap_or(&Value::Null))))
                                .collect();
                            table.add_row(row.into());
                        }
                    }
                    
                    Ok(table.to_string())
                } else {
                    Ok(format!("{arr:?}"))
                }
            }
            Value::Object(obj) => {
                let mut table = Table::new();
                table.add_row(row!["Field".bold(), "Value".bold()]);
                
                for (key, value) in obj {
                    table.add_row(row![key.green(), format_table_value(value)]);
                }
                
                Ok(table.to_string())
            }
            _ => Ok(format_value(data))
        }
    }
}

impl Formatter for CsvFormatter {
    fn format(&self, data: &Value) -> Result<String, Box<dyn std::error::Error>> {
        match data {
            Value::Array(arr) if !arr.is_empty() => {
                let mut output = String::new();
                
                if let Some(Value::Object(first)) = arr.first() {
                    // Headers
                    let headers: Vec<_> = first.keys().cloned().collect();
                    output.push_str(&headers.join(","));
                    output.push('\n');
                    
                    // Data
                    for item in arr {
                        if let Value::Object(obj) = item {
                            let values: Vec<String> = headers.iter()
                                .map(|k| csv_escape(format_value(obj.get(k).unwrap_or(&Value::Null))))
                                .collect();
                            output.push_str(&values.join(","));
                            output.push('\n');
                        }
                    }
                }
                
                Ok(output)
            }
            _ => Err("CSV format only supports arrays of objects".into())
        }
    }
}

fn format_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(arr) => format!("[{} items]", arr.len()),
        Value::Object(obj) => format!("{{{} fields}}", obj.len()),
    }
}

fn format_table_value(value: &Value) -> String {
    match value {
        Value::String(s) => {
            if s.len() > 50 {
                let truncated = &s[..47];
                format!("{truncated}...")
            } else {
                s.clone()
            }
        }
        _ => format_value(value)
    }
}

fn csv_escape(s: String) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        let escaped = s.replace('"', "\"\"");
        format!("\"{escaped}\"")
    } else {
        s
    }
}