pub fn init_debug_logger() {
    #[cfg(debug_assertions)]
    {
        use std::fs::{self, OpenOptions};
        use tracing_subscriber::fmt::writer::BoxMakeWriter;

        let _ = fs::create_dir_all("logs");

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("logs/debug.log")
            .expect("failed to open debug log file");

        tracing_subscriber::fmt()
            .with_writer(BoxMakeWriter::new(file))
            .with_ansi(false)
            .with_target(true)
            .with_level(true)
            .init();
    }

    #[cfg(not(debug_assertions))]
    {
        // releaseビルドではログファイルを作らない
    }
}
