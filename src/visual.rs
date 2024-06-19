use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{fmt::Write, u64};

pub fn progress_bar(to_process: u64) -> ProgressBar {
    let pb = ProgressBar::new(to_process);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:50.cyan/blue}] ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    pb
}
