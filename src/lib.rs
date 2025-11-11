use backtrace::Backtrace;
use chrono::{Local, NaiveDate};
use notify_rust::Notification;
use std::env;
use std::fs;
use std::io::Write;
use std::panic;
use std::sync::mpsc::{Sender, channel};
use std::thread;

pub struct VersaLog {
    enum_mode: String,
    tag: String,
    showFile: bool,
    showTag: bool,
    notice: bool,
    enableall: bool,
    allsave: bool,
    savelevels: Vec<String>,
    silent: bool,
    tx: Option<Sender<(String, String)>>,
    last_cleanup_date: Option<NaiveDate>,
    catch_exceptions: bool,
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

pub fn NewVersaLog(
    enum_mode: &str,
    show_file: bool,
    show_tag: bool,
    tag: &str,
    enable_all: bool,
    notice: bool,
    all_save: bool,
    save_levels: Vec<String>,
    catch_exceptions: bool,
) -> VersaLog {
    let mode = enum_mode.to_lowercase();
    let tag = tag.to_string();

    if !VALID_MODES.contains(&enum_mode) {
        panic!(
            "Invalid mode '{}' specified. Valid modes are: simple, simple2, detailed, file",
            enum_mode
        );
    }

    let mut showFile = show_file;
    let mut showTag = show_tag;
    let mut notice_enabled = notice;
    let mut allsave = all_save;
    let mut savelevels = save_levels;

    if enable_all {
        showFile = true;
        showTag = true;
        notice_enabled = true;
        allsave = true;
    }

    if enum_mode == "file" {
        showFile = true;
    }

    if allsave {
        if savelevels.is_empty() {
            savelevels = VALID_SAVE_LEVELS.iter().map(|s| s.to_string()).collect();
        } else {
            for level in &savelevels {
                if !VALID_SAVE_LEVELS.contains(&level.as_str()) {
                    panic!(
                        "Invalid saveLevels specified. Valid levels are: {:?}",
                        VALID_SAVE_LEVELS
                    );
                }
            }
        }
    }

    let tx = if allsave {
        let (tx, rx) = channel::<(String, String)>();
        thread::spawn(move || {
            while let Ok((log_text, _level)) = rx.recv() {
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
        });
        Some(tx)
    } else {
        None
    };

    VersaLog {
        enum_mode: enum_mode.to_string(),
        tag,
        showFile,
        showTag,
        notice: notice_enabled,
        enableall: enable_all,
        allsave,
        savelevels,
        silent: false,
        tx,
        last_cleanup_date: None,
        catch_exceptions,
    }
}

pub fn NewVersaLogSimple(enum_mode: &str, tag: &str) -> VersaLog {
    NewVersaLog(
        enum_mode,
        false,
        false,
        tag,
        false,
        false,
        false,
        Vec::new(),
        false,
    )
}

pub fn NewVersaLogSimple2(enum_mode: &str, tag: &str, enable_all: bool) -> VersaLog {
    NewVersaLog(
        enum_mode,
        false,
        false,
        tag,
        enable_all,
        false,
        false,
        Vec::new(),
        false,
    )
}

impl VersaLog {
    pub fn log(&self, msg: String, level: String, tags: &[&str]) {
        let level = level.to_uppercase();

        let color = COLORS
            .iter()
            .find(|(l, _)| *l == level)
            .map(|(_, c)| *c)
            .unwrap_or("");
        let symbol = SYMBOLS
            .iter()
            .find(|(l, _)| *l == level)
            .map(|(_, s)| *s)
            .unwrap_or("");

        let caller = if self.showFile || self.enum_mode == "file" {
            self.get_caller()
        } else {
            String::new()
        };

        let final_tag = if !tags.is_empty() && !tags[0].is_empty() {
            tags[0].to_string()
        } else if self.showTag && !self.tag.is_empty() {
            self.tag.clone()
        } else {
            String::new()
        };

        let (output, plain) = match self.enum_mode.as_str() {
            "simple" => {
                if self.showFile {
                    if !final_tag.is_empty() {
                        let output = format!(
                            "[{}][{}]{}{}{} {}",
                            caller, final_tag, color, symbol, RESET, msg
                        );
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
            }
            "simple2" => {
                let timestamp = self.get_time();
                if self.showFile {
                    if !final_tag.is_empty() {
                        let output = format!(
                            "[{}] [{}][{}]{}{}{} {}",
                            timestamp, caller, final_tag, color, symbol, RESET, msg
                        );
                        let plain = format!(
                            "[{}] [{}][{}]{} {}",
                            timestamp, caller, final_tag, symbol, msg
                        );
                        (output, plain)
                    } else {
                        let output = format!(
                            "[{}] [{}]{}{}{} {}",
                            timestamp, caller, color, symbol, RESET, msg
                        );
                        let plain = format!("[{}] [{}]{} {}", timestamp, caller, symbol, msg);
                        (output, plain)
                    }
                } else {
                    let output = format!("[{}] {}{}{} {}", timestamp, color, symbol, RESET, msg);
                    let plain = format!("[{}] {} {}", timestamp, symbol, msg);
                    (output, plain)
                }
            }
            "file" => {
                let output = format!("[{}]{}{}[{}]{}", caller, color, level, RESET, msg);
                let plain = format!("[{}][{}] {}", caller, level, msg);
                (output, plain)
            }
            _ => {
                let timestamp = self.get_time();
                let mut output = format!("[{}]{}{}[{}]{}", timestamp, color, level, RESET);
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

        if !self.silent {
            println!("{}", output);
        }
        self.save_log(plain, level.clone());

        if self.notice && (level == "ERROR" || level == "CRITICAL") {
            let _ = Notification::new()
                .summary(&format!("{} Log notice", level))
                .body(&msg)
                .show();
        }
    }

    pub fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
    }

    pub fn install_panic_hook(self: std::sync::Arc<Self>) {
        let logger = self.clone();
        panic::set_hook(Box::new(move |info| {
            let payload = info.payload();
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                (*s).to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic".to_string()
            };

            let mut details = String::new();
            if let Some(loc) = info.location() {
                details.push_str(&format!(
                    "at {}:{}:{}\n",
                    loc.file(),
                    loc.line(),
                    loc.column()
                ));
            }
            let bt = Backtrace::new();
            details.push_str(&format!("{:?}", bt));

            logger.Critical_no_tag(&format!("Unhandled panic: {}\n{}", msg, details));
        }));
    }

    pub fn handle_exception(&self, exc_type: &str, exc_value: &str, exc_traceback: &str) {
        let tb_str = format!(
            "Exception Type: {}\nException Value: {}\nTraceback:\n{}",
            exc_type, exc_value, exc_traceback
        );
        self.Critical_no_tag(&format!("Unhandled exception:\n{}", tb_str));
    }

    pub fn Info(&self, msg: &str, tags: &[&str]) {
        self.log(msg.to_string(), "INFO".to_string(), tags);
    }

    pub fn Error(&self, msg: &str, tags: &[&str]) {
        self.log(msg.to_string(), "ERROR".to_string(), tags);
    }

    pub fn Warning(&self, msg: &str, tags: &[&str]) {
        self.log(msg.to_string(), "WARNING".to_string(), tags);
    }

    pub fn Debug(&self, msg: &str, tags: &[&str]) {
        self.log(msg.to_string(), "DEBUG".to_string(), tags);
    }

    pub fn Critical(&self, msg: &str, tags: &[&str]) {
        self.log(msg.to_string(), "CRITICAL".to_string(), tags);
    }

    pub fn info(&self, msg: &str, tags: &[&str]) {
        self.Info(msg, tags);
    }

    pub fn error(&self, msg: &str, tags: &[&str]) {
        self.Error(msg, tags);
    }

    pub fn warning(&self, msg: &str, tags: &[&str]) {
        self.Warning(msg, tags);
    }

    pub fn debug(&self, msg: &str, tags: &[&str]) {
        self.Debug(msg, tags);
    }

    pub fn critical(&self, msg: &str, tags: &[&str]) {
        self.Critical(msg, tags);
    }

    pub fn Info_no_tag(&self, msg: &str) {
        self.log(msg.to_string(), "INFO".to_string(), &[]);
    }

    pub fn Error_no_tag(&self, msg: &str) {
        self.log(msg.to_string(), "ERROR".to_string(), &[]);
    }

    pub fn Warning_no_tag(&self, msg: &str) {
        self.log(msg.to_string(), "WARNING".to_string(), &[]);
    }

    pub fn Debug_no_tag(&self, msg: &str) {
        self.log(msg.to_string(), "DEBUG".to_string(), &[]);
    }

    pub fn Critical_no_tag(&self, msg: &str) {
        self.log(msg.to_string(), "CRITICAL".to_string(), &[]);
    }

    fn get_time(&self) -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn get_caller(&self) -> String {
        let bt = Backtrace::new();
        if let Some(frame) = bt.frames().get(3) {
            if let Some(symbol) = frame.symbols().first() {
                if let Some(file) = symbol.filename() {
                    if let Some(file_name) = file.file_name() {
                        if let Some(line) = symbol.lineno() {
                            return format!("{}:{}", file_name.to_string_lossy(), line);
                        }
                    }
                }
            }
        }
        "unknown:0".to_string()
    }

    fn cleanup_old_logs(&self, days: i64) {
        let cwd = env::current_dir().unwrap_or_else(|_| env::current_dir().unwrap());
        let log_dir = cwd.join("log");

        if !log_dir.exists() {
            return;
        }

        let now = Local::now().naive_local().date();

        if let Ok(entries) = fs::read_dir(&log_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
                        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                            if let Ok(file_date) = NaiveDate::parse_from_str(
                                &file_name.replace(".log", ""),
                                "%Y-%m-%d",
                            ) {
                                if (now - file_date).num_days() >= days {
                                    if let Err(e) = fs::remove_file(&path) {
                                        if !self.silent {
                                            println!(
                                                "[LOG CLEANUP WARNING] {} cannot be removed: {}",
                                                path.display(),
                                                e
                                            );
                                        }
                                    } else if !self.silent {
                                        println!("[LOG CLEANUP] removed: {}", path.display());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn save_log(&self, log_text: String, level: String) {
        if !self.allsave || !self.savelevels.contains(&level) {
            return;
        }

        if let Some(tx) = &self.tx {
            let _ = tx.send((log_text, level));
            return;
        }

        self.save_log_sync(log_text, level);
    }

    fn save_log_sync(&self, log_text: String, level: String) {
        if !self.allsave || !self.savelevels.contains(&level) {
            return;
        }

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

        let today_date = Local::now().naive_local().date();
        if self.last_cleanup_date != Some(today_date) {
            self.cleanup_old_logs(7);
        }
    }
}
