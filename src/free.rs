use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};

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
