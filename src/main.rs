mod adapters;
mod domain;

use crate::adapters::memory::InMemoryEngineDeps;
use crate::domain::engine::TransactionEngine;
use crate::domain::model::{InputRecord, Transaction};
use crate::domain::ports::{Engine, EngineConfig};
use clap::{App, Arg};
use csv::{ReaderBuilder, Trim};
use std::convert::TryInto;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let matches = App::new("Simple Payment Engine")
        .version("1.0")
        .author("Brandon K. <brandonkite92@gmail.com>")
        .arg(
            Arg::with_name("TRANSACTIONS_FILE")
                .help("A file containing the transactions")
                .required(true),
        )
        .get_matches();

    let file = matches
        .value_of("TRANSACTIONS_FILE")
        // We shouldn't reach this due to usage of `.required(true)` above
        .expect("No transactions file input provided.");

    let engine = TransactionEngine::<InMemoryEngineDeps>::default();
    process_file(file, engine).await
}

async fn process_file<C: EngineConfig>(file_path: &str, mut engine: TransactionEngine<C>) {
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(PathBuf::from(file_path))
        .unwrap();
    for result in rdr.deserialize() {
        let record: InputRecord = result.unwrap();
        let transaction: Transaction = record.try_into().unwrap();
        engine
            .process_transaction(transaction.clone())
            .await
            .unwrap();
        println!("processed {:?}", transaction);
    }
}
