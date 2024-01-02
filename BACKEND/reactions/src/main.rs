use tracing::{info, error, Level};

fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(Level::TRACE)
        .with_current_span(true)
        .init();

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = 34;

    let a: Result<(), &str> = Err("something really bad happened");

    error!(
        error = a.unwrap_err(),
        "something really bad happened"
    );

    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        filename = file!(),
        "yak shaving completed",
    );
}
