mod dp;
mod bb;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::conversion::FromPyObject;
use pyo3::types::PyTuple;


#[derive(Debug, Copy, Clone)]
struct Item {
    index: usize,
    value: usize,
    weight: usize,
}

impl FromPyObject<'_> for Item {
    fn extract(obj: &'_ PyAny) -> PyResult<Self> {
        let tmp = obj.cast_as::<PyTuple>()?;
        Ok(Item{
            index: tmp.get_item(0).extract::<usize>()?,
            value: tmp.get_item(1).extract::<usize>()?,
            weight: tmp.get_item(2).extract::<usize>()?,
        })
    }
}

#[pyfunction]
fn solve_dp(mut items: Vec<Item>, cap: usize) -> PyResult<(usize, usize, Vec<usize>)> {
    Ok(dp::solve(&mut items, cap))
}

#[pyfunction]
fn solve_bb(mut items: Vec<Item>, error: f64, cap: usize) -> PyResult<(usize, usize, Vec<usize>)> {
    Ok(bb::solve_bb(&mut items, error, cap))
}

#[pymodule]
fn knapsack(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(solve_dp))?;
    m.add_wrapped(wrap_pyfunction!(solve_bb))?;
    Ok(())
}
