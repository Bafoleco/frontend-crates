// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

//! Conformance harness for the authored Unified parser corpus.

mod runner;

pub use runner::{CaseFailure, CaseUnsupported, RunOptions, RunReport, run_unified_family};
