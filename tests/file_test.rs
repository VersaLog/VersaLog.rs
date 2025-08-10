use versalogrs::{NewVersaLog, VersaLog};

fn main() {
    let logger = NewVersaLog("file", false, false, "VersaLog", false, false, false, vec![]);
    
    logger.Info("info", None);
    logger.Error("error", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}