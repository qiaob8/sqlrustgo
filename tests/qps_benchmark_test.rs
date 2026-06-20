//! QPS Benchmark Tests for SQLRustGo
//! Measures queries per second for DELETE, UPDATE, INSERT, SELECT operations

use sqlrustgo::{parse, ExecutionEngine};
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;

const BENCHMARK_ITERATIONS: usize = 1000;

fn log_result(msg: &str) {
    eprintln!("{}", msg);
    println!("{}", msg);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("qps_results.log")
        .unwrap();
    writeln!(file, "{}", msg).unwrap();
}

fn create_engine() -> ExecutionEngine {
    ExecutionEngine::new()
}

fn setup_tables(engine: &mut ExecutionEngine) {
    let _ = engine.execute(parse("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)").unwrap());
}

#[test]
#[ignore]
fn test_qps_delete() {
    let mut engine = create_engine();
    setup_tables(&mut engine);

    // Insert benchmark rows
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("INSERT INTO users VALUES ({}, 'bench_{}', {})", i, i, 30)).unwrap()
        );
    }

    // Benchmark DELETE
    let start = Instant::now();
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("DELETE FROM users WHERE id = {}", i)).unwrap()
        );
    }
    let duration = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / duration.as_secs_f64();

    log_result(&format!("DELETE QPS: {} queries in {:.2}s ({:.2} qps)",
             BENCHMARK_ITERATIONS, duration.as_secs_f64(), qps));
}

#[test]
#[ignore]
fn test_qps_update() {
    let mut engine = create_engine();
    setup_tables(&mut engine);

    // Insert benchmark rows
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("INSERT INTO users VALUES ({}, 'bench_{}', {})", i, i, 30)).unwrap()
        );
    }

    // Benchmark UPDATE
    let start = Instant::now();
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("UPDATE users SET age = {} WHERE id = {}", i + 1, i)).unwrap()
        );
    }
    let duration = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / duration.as_secs_f64();

    log_result(&format!("UPDATE QPS: {} queries in {:.2}s ({:.2} qps)",
             BENCHMARK_ITERATIONS, duration.as_secs_f64(), qps));
}

#[test]
#[ignore]
fn test_qps_insert() {
    let mut engine = create_engine();
    setup_tables(&mut engine);

    // Benchmark INSERT
    let start = Instant::now();
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("INSERT INTO users VALUES ({}, 'bench_{}', {})", i, i, 30)).unwrap()
        );
    }
    let duration = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / duration.as_secs_f64();

    log_result(&format!("INSERT QPS: {} queries in {:.2}s ({:.2} qps)",
             BENCHMARK_ITERATIONS, duration.as_secs_f64(), qps));
}

#[test]
#[ignore]
fn test_qps_simple_select() {
    let mut engine = create_engine();
    setup_tables(&mut engine);

    // Insert benchmark rows
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("INSERT INTO users VALUES ({}, 'bench_{}', {})", i, i, 30)).unwrap()
        );
    }

    // Benchmark SELECT
    let start = Instant::now();
    for i in 0..BENCHMARK_ITERATIONS {
        let _ = engine.execute(
            parse(&format!("SELECT * FROM users WHERE id = {}", i)).unwrap()
        );
    }
    let duration = start.elapsed();
    let qps = BENCHMARK_ITERATIONS as f64 / duration.as_secs_f64();

    log_result(&format!("SELECT QPS: {} queries in {:.2}s ({:.2} qps)",
             BENCHMARK_ITERATIONS, duration.as_secs_f64(), qps));
}
