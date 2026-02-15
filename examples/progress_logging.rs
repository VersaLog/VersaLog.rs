use versalogrs::NewVersaLog;

fn process_file(log: &VersaLog, index: usize, total: usize) {
    log.Step(
        &format!("Processing file_{}.txt", index),
        index,
        total,
        &[],
    );

    let _timer = log.Timer(
        &format!("file_{}.txt", index),
        &[],
    );

    let total_lines = 10;

    for i in 1..=total_lines {
        std::thread::sleep(std::time::Duration::from_millis(100));

        log.Progress(
            &format!("file_{}.txt", index),
            i,
            total_lines,
            &[],
        );
    }
}

fn main() {
    let log = VersaLog::new(
        "detailed",
        true,
        true,
        "BATCH",
    );

    log.Info("Batch Start", &[]);

    let _batch_timer = log.Timer("Total Batch", &[]);

    let total_files = 3;

    for i in 1..=total_files {
        process_file(&log, i, total_files);

        log.Progress(
            "Overall Progress",
            i,
            total_files,
            &[],
        );
    }

    log.Info("Batch Finished", &[]);
}