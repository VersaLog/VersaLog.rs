use versalogrs::{NewVersaLog, VersaLog};

// showFile false
fn main() {
    let logger = NewVersaLog("simple", false, false, "VersaLog", false, false, false, vec![]);
    
    logger.Info("info", None);
    logger.Error("error", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}

// showFile true
fn main() {
    let logger = NewVersaLog("simple", true, false, "VersaLog", false, false, false, vec![]);
    
    logger.Info("info", None);
    logger.Error("error", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}

// show_tag false
fn main() {
    let logger = NewVersaLog("simple", false, false, "VersaLog", false, false, false, vec![]);
    
    logger.Info("info", None);
    logger.Error("error", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}

// show_tag true
fn main() {
    let logger = NewVersaLog("simple", false, true, "VersaLog", false, false, false, vec![]);
    
    logger.Info("info", None);
    logger.Error("error", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}

// notice false
fn main() {
    let logger = NewVersaLog("simple", false, false, "VersaLog", false, false, false, vec![]);
    
    logger.Info("info", None);
    logger.Error("error", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}

// notice true
fn main() {
    let logger = NewVersaLog("simple", false, false, "VersaLog", false, true, false, vec![]);
    
    logger.Info("info", None);
    logger.Error("error", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}

// enable_all true
fn main() {
    let logger = NewVersaLog("simple", false, false, "VersaLog", true, false, false, vec![]);
    
    logger.Info("info", None);
    logger.Error("info", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}
