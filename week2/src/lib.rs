mod dp;
mod bb;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::conversion::FromPyObject;
use pyo3::types::PyTuple;


#[derive(Debug)]
struct Item {
    index: usize,
    value: usize,
    weight: usize,
    src_idx: usize,         // item index in a source array
}

impl FromPyObject<'_> for Item {
    fn extract(obj: &'_ PyAny) -> PyResult<Self> {
        let tmp = obj.cast_as::<PyTuple>()?;
        Ok(Item{
            index: tmp.get_item(0).extract::<usize>()?,
            value: tmp.get_item(1).extract::<usize>()?,
            weight: tmp.get_item(2).extract::<usize>()?,
            src_idx: 0,
        })
    }
}

#[pyfunction]
fn solve_dp(items: Vec<Item>, cap: usize) -> PyResult<(usize, usize, Vec<usize>)> {
    Ok(dp::solve_dp(items, cap))
}

#[pyfunction]
fn solve_bb(items: Vec<Item>, cap: usize) -> PyResult<(usize, usize, Vec<usize>)> {
    Ok(dp::solve_dp(items, cap))
}


#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(solve_dp))?;

    Ok(())
}
