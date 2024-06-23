use chrono::{DateTime, Utc};
use clap::Parser;
use serde_json::Value;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(version, about = "Insert or inject time calculations into JSON logs from file or STDIN", long_about = None)]
struct Cli {
    /// Optional filename
    filename: Option<PathBuf>,

    /// Insert new JSON lines with milliseconds since the previous line
    #[arg(short('i'), long("insert-millis-since-previous"))]
    insert_millis_since_previous: bool,

    /// Insert new JSON lines with total milliseconds elapsed since the first line
    #[arg(short('s'), long("insert-millis-since-start"))]
    insert_millis_since_start: bool,

    /// Inject a new JSON field with milliseconds since the previous line
    #[arg(short('I'), long("inject-millis-since-previous"))]
    inject_millis_since_previous: bool,

    /// Inject a new JSON field with total milliseconds elapsed since the first line
    #[arg(short('S'), long("inject-millis-since-start"))]
    inject_millis_since_start: bool,

    /// The JSON field to use for time values: defaults to "time"
    #[arg(short, long, value_name = "TIME_FIELD", default_value = "time")]
    time_field: String,
}

fn main() {
    let cli = Cli::parse();

    let input: Box<dyn BufRead> = if let Some(file) = &cli.filename {
        Box::new(BufReader::new(
            File::open(file).expect("Unable to open file"),
        ))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    process_lines(
        input,
        cli.insert_millis_since_previous,
        cli.insert_millis_since_start,
        cli.inject_millis_since_previous,
        cli.inject_millis_since_start,
        &cli.time_field,
    );
}

fn process_lines<R: BufRead>(
    reader: R,
    insert_millis_since_previous: bool,
    insert_millis_since_start: bool,
    inject_millis_since_previous: bool,
    inject_millis_since_start: bool,
    time_field: &str,
) {
    let mut previous_time: Option<DateTime<Utc>> = None;
    let mut start_time: Option<DateTime<Utc>> = None;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut json: Value = serde_json::from_str(&line).expect("Invalid JSON");

        if let Some(time_str) = json.get(time_field).and_then(Value::as_str) {
            let current_time: DateTime<Utc> = time_str.parse().expect("Invalid time format");
            if start_time.is_none() {
                start_time = Some(current_time);
            }

            if let Some(prev_time) = previous_time {
                let millis_since_previous = (current_time - prev_time).num_milliseconds();
                let millis_since_start = (current_time - start_time.unwrap()).num_milliseconds();

                if inject_millis_since_previous {
                    json["millis_since_previous_line"] = serde_json::json!(millis_since_previous);
                }

                if inject_millis_since_start {
                    json["millis_since_start"] = serde_json::json!(millis_since_start);
                }

                if insert_millis_since_previous || insert_millis_since_start {
                    let mut inserted_line = serde_json::Map::new();
                    if insert_millis_since_previous {
                        inserted_line.insert(
                            "millis_since_previous_line".to_string(),
                            serde_json::json!(millis_since_previous),
                        );
                    }
                    if insert_millis_since_start {
                        inserted_line.insert(
                            "millis_since_start".to_string(),
                            serde_json::json!(millis_since_start),
                        );
                    }
                    println!(
                        "{}",
                        serde_json::to_string(&inserted_line).expect("Failed to serialize JSON")
                    );
                }
            }

            previous_time = Some(current_time);
        }

        println!(
            "{}",
            serde_json::to_string(&json).expect("Failed to serialize JSON")
        );
    }
}
