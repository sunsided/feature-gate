#![allow(dead_code)]

use feature_gate::{feature_gate, feature_gate_ex};

#[feature_gate("test")]
struct Test;

#[feature_gate_ex(any(test, feature = "test"))]
struct Test;
