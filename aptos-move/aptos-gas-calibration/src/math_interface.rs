// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

const DEFAULT_VALUE: f64 = 0.0;

/// Getter function for number of rows
///
/// ### Arguments
///
/// * `input` - All the equations
pub fn total_num_rows(input: &[BTreeMap<String, u64>]) -> usize {
    input.len()
}

/// Taking the union finds the total distinct gas parameters
///
/// ### Arguments
///
/// * `input` - All the equations in the simplified mapping version
pub fn total_num_of_cols(input: &[BTreeMap<String, u64>]) -> usize {
    input.iter().flat_map(|map| map.keys()).collect::<BTreeMap<_, _>>().len()
}

/// Creates a generic template for BTreeMaps so we can easily
/// translate the entries into indices of our vector representation
///
/// ### Arguments
///
/// * `input` - All the equations in the simplified mapping version
pub fn generic_map(input: &[BTreeMap<String, u64>]) -> BTreeMap<String, f64> {
    input
        .iter()
        .flat_map(|map| map.keys().cloned())
        .collect::<BTreeMap<String, f64>>()
        .into_iter()
        .map(|(key, _)| (key, DEFAULT_VALUE))
        .collect()
}

/// Standardize all maps into a generic map
///
/// ### Arguments
///
/// * `input` - All the equations in the simplified mapping version
pub fn convert_to_generic_map(input: &[BTreeMap<String, u64>]) -> Vec<BTreeMap<String, f64>> {
    let template = generic_map(input);
    input
        .iter()
        .map(|map| {
            let mut generic = template.clone();
            for (key, value) in map {
                generic.entry(key.clone()).and_modify(|v| *v = *value as f64);
            }
            generic
        })
        .collect()
}

/// Transform into standardized mapping before converting to a vector
///
/// ### Arguments
///
/// * `input` - All the equations in the simplified mapping version
pub fn convert_to_matrix_format(input: &[BTreeMap<String, u64>]) -> Vec<Vec<f64>> {
    let ncols = total_num_of_cols(input);
    let generic_maps = convert_to_generic_map(input);

    generic_maps
        .into_iter()
        .map(|eq| {
            let vec_format: Vec<f64> = eq.into_values().collect();
            assert_eq!(vec_format.len(), ncols);
            vec_format
        })
        .collect()
}
  
