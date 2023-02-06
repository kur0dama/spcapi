use polars::{prelude::*, lazy::dsl::StrpTimeOptions};
use std::fs::File;

pub fn load_csv(csvpath: &str, date_col: &str, n_col: &str, sample_col: &str) -> DataFrame {
    let file = File::open(csvpath).expect("Could not open file");
    let load_result = CsvReader::new(file)
        .infer_schema(None)
        .has_header(true)
        .finish();
    let df = match load_result {
        Ok(df) => df,
        Err(error) => panic!("Encountered an error loading csv: {}", error),
    };

    let df2 = df
        .clone()
        .lazy()
        .with_column(
            col(date_col)
                .str()
                .strptime(StrpTimeOptions {
                    date_dtype: DataType::Datetime(TimeUnit::Milliseconds, None),
                    fmt: Some("%Y-%m-%dT%H:%M:%S".into()),
                    strict: false,
                    exact: true,
                    cache: false,
                    tz_aware: false
                })
                .alias("dt"),
        )
        .select([
            col("dt"),
            col(n_col).alias("n"),
            col(sample_col).alias("w")
        ])
        .collect();

    df2.unwrap()
}