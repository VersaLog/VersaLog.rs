use versalogrs::NewVersaLog;

fn main() {
    let logger = NewVersaLog(
        "simple",
        false,
        false,
        "VersaLog",
        false,
        false,
        true,
        vec![],
    );

    logger.Info("info", &[]);
    logger.Error("error", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}
