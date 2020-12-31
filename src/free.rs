use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};

#[pyfunction]
pub fn all(_py: Python, iterable: &PyAny, f: &PyAny) -> PyResult<bool> {
    if !f.is_callable() {
        return Err(PyTypeError::new_err("predicate is not callable"));
    }
    let iter = iterable.iter();
    if iter.is_err() {
        return Err(iter.unwrap_err());
    }

    let r = itertools::all(iter.unwrap(), |elt| {
        if elt.is_err() {
            return false; // XXX: raise error
        }

        let ok = f.call1((elt.unwrap(),));
        match ok.map(|r| r.is_true().unwrap_or(false)) {
            Ok(r) => r,
            _ => false,
        }
    });

    Ok(r)
}
