use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyfunction]
pub fn all(_py: Python, iterable: &PyAny, f: &PyAny) -> PyResult<bool> {
    if !f.is_callable() {
        return Err(PyTypeError::new_err("predicate is not callable"));
    }

    let r = itertools::all(iterable.iter()?, |elt| {
        if elt.is_err() {
            return false; // XXX: Return Err?
        }

        match f.call1((elt.unwrap(),)).and_then(|r| r.is_true()) {
            Ok(ok) => ok,
            _ => false,
        }
    });

    Ok(r)
}
