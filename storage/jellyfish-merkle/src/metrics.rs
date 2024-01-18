// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use mango_metrics::{register_int_counter, IntCounter};
use once_cell::sync::Lazy;

pub static DIEM_JELLYFISH_LEAF_ENCODED_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "diem_jellyfish_leaf_encoded_bytes",
        "mango jellyfish leaf encoded bytes in total"
    )
    .unwrap()
});

pub static DIEM_JELLYFISH_INTERNAL_ENCODED_BYTES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "diem_jellyfish_internal_encoded_bytes",
        "mango jellyfish total internal nodes encoded in bytes"
    )
    .unwrap()
});

pub static DIEM_JELLYFISH_STORAGE_READS: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "diem_jellyfish_storage_reads",
        "mango jellyfish reads from storage"
    )
    .unwrap()
});
