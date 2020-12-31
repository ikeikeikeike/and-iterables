// #![allow(unused_variables)]
// #![allow(unused_imports)]

use pyo3::prelude::*;
// use pyo3::types::PyDict;
use pyo3::exceptions::PyTypeError;
use pyo3::types::{PyAny, PyDict, PyFunction, PyList};
use pyo3::wrap_pyfunction;

#[pymodule]
fn and_itertools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(all))?;

    Ok(())
}

#[pyfunction]
pub fn all(_py: Python, iterable: &PyList, f: &PyAny) -> PyResult<bool> {
    if !f.is_callable() {
        return Err(PyTypeError::new_err("predicate is not callable"));
    }

    Ok(itertools::all(iterable.iter(), |elt| {
        match f.call1((elt,)).map(|r| r.is_true().unwrap_or(false)) {
            Ok(r) => r,
            _ => false,
        }
    }))
}
