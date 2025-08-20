use versalogrs::NewVersaLog;

// showFile false
fn main() {
    let logger = NewVersaLog(
        "simple",
        false,
        false,
        "VersaLog",
        false,
        false,
        false,
        vec![],
    );

    logger.Info("info", &[]);
    logger.Error("error", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}

// showFile true
fn main() {
    let logger = NewVersaLog(
        "simple",
        true,
        false,
        "VersaLog",
        false,
        false,
        false,
        vec![],
    );

    logger.Info("info", &[]);
    logger.Error("error", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}

// show_tag false
fn main() {
    let logger = NewVersaLog(
        "simple",
        false,
        false,
        "VersaLog",
        false,
        false,
        false,
        vec![],
    );

    logger.Info("info", &[]);
    logger.Error("error", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}

// show_tag true
fn main() {
    let logger = NewVersaLog(
        "simple",
        false,
        true,
        "VersaLog",
        false,
        false,
        false,
        vec![],
    );

    logger.Info("info", &[]);
    logger.Error("error", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}

// notice false
fn main() {
    let logger = NewVersaLog(
        "simple",
        false,
        false,
        "VersaLog",
        false,
        false,
        false,
        vec![],
    );

    logger.Info("info", &[]);
    logger.Error("error", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}

// notice true
fn main() {
    let logger = NewVersaLog(
        "simple",
        false,
        false,
        "VersaLog",
        false,
        true,
        false,
        vec![],
    );

    logger.Info("info", &[]);
    logger.Error("error", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}

// enable_all true
fn main() {
    let logger = NewVersaLog(
        "simple",
        false,
        false,
        "VersaLog",
        true,
        false,
        false,
        vec![],
    );

    logger.Info("info", &[]);
    logger.Error("info", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}
