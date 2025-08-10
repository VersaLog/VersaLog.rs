use versalogrs::{NewVersaLog, VersaLog};

fn main() {
    let logger = NewVersaLog("detailed", false, false, "VersaLog", false, false, true, vec![]);
    
    logger.Info("info", None);
    logger.Error("error", None);
    logger.Warning("warning.", None);
    logger.Debug("debug", None);
    logger.Critical("critical", None);
}