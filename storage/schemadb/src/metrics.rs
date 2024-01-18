// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use mango_metrics::{
    register_histogram_vec, register_int_counter_vec, HistogramVec, IntCounterVec,
};
use once_cell::sync::Lazy;

pub static DIEM_SCHEMADB_ITER_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_iter_latency_seconds",
        // metric description
        "mango schemadb iter latency in seconds",
        // metric labels (dimensions)
        &["cf_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_ITER_BYTES: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_iter_bytes",
        // metric description
        "mango schemadb iter size in bytess",
        // metric labels (dimensions)
        &["cf_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_GET_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_get_latency_seconds",
        // metric description
        "mango schemadb get latency in seconds",
        // metric labels (dimensions)
        &["cf_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_GET_BYTES: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_get_bytes",
        // metric description
        "mango schemadb get call returned data size in bytes",
        // metric labels (dimensions)
        &["cf_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_BATCH_COMMIT_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_batch_commit_latency_seconds",
        // metric description
        "mango schemadb schema batch commit latency in seconds",
        // metric labels (dimensions)
        &["db_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_BATCH_COMMIT_BYTES: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_batch_commit_bytes",
        // metric description
        "mango schemadb schema batch commit size in bytes",
        // metric labels (dimensions)
        &["db_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_PUT_BYTES: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "diem_schemadb_put_bytes",
        // metric description
        "mango schemadb put call puts data size in bytes",
        // metric labels (dimensions)
        &["cf_name"]
    )
    .unwrap()
});

pub static DIEM_SCHEMADB_DELETES: Lazy<IntCounterVec> = Lazy::new(|| {
    register_int_counter_vec!(
        "diem_storage_deletes",
        "mango storage delete calls",
        &["cf_name"]
    )
    .unwrap()
});
