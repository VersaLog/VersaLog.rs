use notify_rust::Notification;
use chrono::Local;
use std::env;
use std::fs;
use std::io::Write;

pub struct VersaLog {
    mode: String,
    tag: String,
    showFile: bool,
    showTag: bool,
    notice: bool,
    enableall: bool,
    allsave: bool,
    savelevels: Vec<String>,
}

static COLORS: &[(&str, &str)] = &[
    ("INFO", "\x1b[32m"),
    ("ERROR", "\x1b[31m"),
    ("WARNING", "\x1b[33m"),
    ("DEBUG", "\x1b[36m"),
    ("CRITICAL", "\x1b[35m"),
];

static SYMBOLS: &[(&str, &str)] = &[
    ("INFO", "[+]"),
    ("ERROR", "[-]"),
    ("WARNING", "[!]"),
    ("DEBUG", "[D]"),
    ("CRITICAL", "[C]"),
];

static RESET: &str = "\x1b[0m";

static VALID_MODES: &[&str] = &["simple", "simple2", "detailed", "file"];
static VALID_SAVE_LEVELS: &[&str] = &["INFO", "ERROR", "WARNING", "DEBUG", "CRITICAL"];

// Goのバージョンと同様の簡潔なAPI
pub fn NewVersaLog(mode: &str, show_file: bool, show_tag: bool, tag: &str, enable_all: bool, notice: bool) -> VersaLog {
    let mode = mode.to_string();
    let tag = tag.to_string();
    
    if !VALID_MODES.contains(&mode.as_str()) {
        panic!("Invalid mode: {}", mode);
    }

    let mut showFile = show_file;
    let mut showTag = show_tag;
    let mut notice_enabled = notice;
    let mut allsave = false;
    let mut savelevels = Vec::new();

    if enable_all {
        showFile = true;
        showTag = true;
        notice_enabled = true;
        allsave = true;
    }

    if mode == "file" {
        showFile = true;
    }

    if allsave {
        savelevels = VALID_SAVE_LEVELS.iter().map(|s| s.to_string()).collect();
    }

    VersaLog {
        mode,
        tag,
        showFile,
        showTag,
        notice: notice_enabled,
        enableall: enable_all,
        allsave,
        savelevels,
    }
}

// より簡潔なコンストラクタ（デフォルト値付き）
pub fn NewVersaLogSimple(mode: &str, tag: &str) -> VersaLog {
    NewVersaLog(mode, false, false, tag, false, false)
}

impl VersaLog {
    pub fn log(&self, msg: String, level: String, tag: Option<String>) {
        let level = level.to_uppercase();
        
        let color = COLORS.iter()
            .find(|(l, _)| *l == level)
            .map(|(_, c)| *c)
            .unwrap_or("");
        let symbol = SYMBOLS.iter()
            .find(|(l, _)| *l == level)
            .map(|(_, s)| *s)
            .unwrap_or("");
        
        let caller = if self.showFile || self.mode == "file" {
            self.get_caller()
        } else {
            String::new()
        };
        
        let final_tag = tag.unwrap_or_else(|| {
            if self.showTag && !self.tag.is_empty() {
                self.tag.clone()
            } else {
                String::new()
            }
        });
        
        let (output, plain) = match self.mode.as_str() {
            "simple" => {
                if self.showFile {
                    if !final_tag.is_empty() {
                        let output = format!("[{}][{}]{}{}{} {}", caller, final_tag, color, symbol, RESET, msg);
                        let plain = format!("[{}][{}]{} {}", caller, final_tag, symbol, msg);
                        (output, plain)
                    } else {
                        let output = format!("[{}]{}{}{} {}", caller, color, symbol, RESET, msg);
                        let plain = format!("[{}]{} {}", caller, symbol, msg);
                        (output, plain)
                    }
                } else {
                    if !final_tag.is_empty() {
                        let output = format!("[{}]{}{}{} {}", final_tag, color, symbol, RESET, msg);
                        let plain = format!("[{}]{} {}", final_tag, symbol, msg);
                        (output, plain)
                    } else {
                        let output = format!("{}{}{} {}", color, symbol, RESET, msg);
                        let plain = format!("{} {}", symbol, msg);
                        (output, plain)
                    }
                }
            },
            "simple2" => {
                let timestamp = self.get_time();
                if self.showFile {
                    if !final_tag.is_empty() {
                        let output = format!("[{}] [{}][{}]{}{}{} {}", timestamp, caller, final_tag, color, symbol, RESET, msg);
                        let plain = format!("[{}] [{}][{}]{} {}", timestamp, caller, final_tag, symbol, msg);
                        (output, plain)
                    } else {
                        let output = format!("[{}] [{}]{}{}{} {}", timestamp, caller, color, symbol, RESET, msg);
                        let plain = format!("[{}] [{}]{} {}", timestamp, caller, symbol, msg);
                        (output, plain)
                    }
                } else {
                    let output = format!("[{}] {}{}{} {}", timestamp, color, symbol, RESET, msg);
                    let plain = format!("[{}] {} {}", timestamp, symbol, msg);
                    (output, plain)
                }
            },
            "file" => {
                let output = format!("[{}]{}{}[{}]{}", caller, color, level, RESET, msg);
                let plain = format!("[{}][{}] {}", caller, level, msg);
                (output, plain)
            },
            _ => {
                let timestamp = self.get_time();
                let mut output = format!("[{}]{}{}[{}]", timestamp, color, level, RESET);
                let mut plain = format!("[{}][{}]", timestamp, level);
                
                if !final_tag.is_empty() {
                    output.push_str(&format!("[{}]", final_tag));
                    plain.push_str(&format!("[{}]", final_tag));
                }
                
                if self.showFile {
                    output.push_str(&format!("[{}]", caller));
                    plain.push_str(&format!("[{}]", caller));
                }
                
                output.push_str(&format!(" : {}", msg));
                plain.push_str(&format!(" : {}", msg));
                
                (output, plain)
            }
        };
        
        println!("{}", output);
        self.save_log(plain, level.clone());
        
        if self.notice && (level == "ERROR" || level == "CRITICAL") {
            let _ = Notification::new()
                .summary(&format!("{} Log notice", level))
                .body(&msg)
                .show();
        }
    }
    
    pub fn Info(&self, msg: &str, tag: Option<&str>) {
        self.log(msg.to_string(), "INFO".to_string(), tag.map(|s| s.to_string()));
    }
    
    pub fn Error(&self, msg: &str, tag: Option<&str>) {
        self.log(msg.to_string(), "ERROR".to_string(), tag.map(|s| s.to_string()));
    }
    
    pub fn Warning(&self, msg: &str, tag: Option<&str>) {
        self.log(msg.to_string(), "WARNING".to_string(), tag.map(|s| s.to_string()));
    }
    
    pub fn Debug(&self, msg: &str, tag: Option<&str>) {
        self.log(msg.to_string(), "DEBUG".to_string(), tag.map(|s| s.to_string()));
    }
    
    pub fn Critical(&self, msg: &str, tag: Option<&str>) {
        self.log(msg.to_string(), "CRITICAL".to_string(), tag.map(|s| s.to_string()));
    }

    // 下位互換性のための小文字メソッド
    pub fn info(&self, msg: &str, tag: Option<&str>) {
        self.Info(msg, tag);
    }
    
    pub fn error(&self, msg: &str, tag: Option<&str>) {
        self.Error(msg, tag);
    }
    
    pub fn warning(&self, msg: &str, tag: Option<&str>) {
        self.Warning(msg, tag);
    }
    
    pub fn debug(&self, msg: &str, tag: Option<&str>) {
        self.Debug(msg, tag);
    }
    
    pub fn critical(&self, msg: &str, tag: Option<&str>) {
        self.Critical(msg, tag);
    }
    
    fn get_time(&self) -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
    
    fn get_caller(&self) -> String {
        "main:0".to_string()
    }
    
    fn save_log(&self, log_text: String, _level: String) {
        let cwd = env::current_dir().unwrap_or_else(|_| env::current_dir().unwrap());
        let log_dir = cwd.join("log");
        
        if !log_dir.exists() {
            let _ = fs::create_dir_all(&log_dir);
        }
        
        let today = Local::now().format("%Y-%m-%d").to_string();
        let log_file = log_dir.join(format!("{}.log", today));
        
        let log_entry = format!("{}\n", log_text);
        let _ = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(&log_file)
            .and_then(|mut file| file.write_all(log_entry.as_bytes()));
    }
}