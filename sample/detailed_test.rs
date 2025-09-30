use versalogrs::NewVersaLog;

// showFile false
fn main() {
    let logger = NewVersaLog(
        "detailed",
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
        "detailed",
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
        "detailed",
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
        "detailed",
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
        "detailed",
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
        "detailed",
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

// silent false
fn main() {
    let logger = NewVersaLog(
        "detailed",
        false,
        false,
        "VersaLog",
        false,
        false,
        false,
        vec![],
        false,
    );

    logger.Info("info", &[]);
    logger.Error("info", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}

// silent true
fn main() {
    let logger = NewVersaLog(
        "detailed",
        false,
        false,
        "VersaLog",
        false,
        false,
        true,
        vec![],
        true,
    );

    logger.Info("info", &[]);
    logger.Error("info", &[]);
    logger.Warning("warning.", &[]);
    logger.Debug("debug", &[]);
    logger.Critical("critical", &[]);
}

// enable_all true
fn main() {
    let logger = NewVersaLog(
        "detailed",
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
