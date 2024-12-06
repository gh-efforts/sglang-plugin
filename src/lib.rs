use crate::knapsack_impl::problem::{Item, Problem};
use pyo3::prelude::*;

#[allow(unused)]
mod knapsack_impl;

#[pyfunction]
fn knapsack(input: Vec<usize>, max_tokens: usize) -> Vec<usize> {
    let items = input
        .iter()
        .enumerate()
        .map(|(i, &weight)| Item::new(i, 1, weight))
        .collect::<Vec<_>>();

    let problem = Problem {
        items,
        capacity: max_tokens,
    };

    let solution = knapsack_impl::greedy::solve(&problem);

    solution.decision.iter().enumerate()
        .filter(|(_, f)| **f)
        .map(|(i, _)| i)
        .collect()
}

// #[test]
// fn test() {
//     let arr: [u8; 32] = rand::random();
//     let arr = arr.map(|v| v as usize);
//     knapsack(&arr, 100);
// }

/// A Python module implemented in Rust.
#[pymodule]
fn sglang_plugin(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(knapsack, m)?)?;
    Ok(())
}
