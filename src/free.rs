use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyInt, PyString};

#[pyfunction]
pub fn all(_py: Python, iterable: &PyAny, f: &PyAny) -> PyResult<bool> {
    if !f.is_callable() {
        return Err(PyTypeError::new_err("predicate is not callable"));
    }

    let r = iterable.iter()?.all(|elt| {
        if elt.is_err() {
            return false; // TODO: must be returned Err
        }

        match f.call1((elt.unwrap(),)).and_then(|r| r.is_true()) {
            Ok(ok) => ok,
            _ => false,
        }
    });

    Ok(r)
}

#[pyfunction]
pub fn any(_py: Python, iterable: &PyAny, f: &PyAny) -> PyResult<bool> {
    if !f.is_callable() {
        return Err(PyTypeError::new_err("predicate is not callable"));
    }

    let r = iterable.iter()?.any(|elt| {
        if elt.is_err() {
            return false; // TODO: must be returned Err
        }

        match f.call1((elt.unwrap(),)).and_then(|r| r.is_true()) {
            Ok(ok) => ok,
            _ => false,
        }
    });

    Ok(r)
}

#[pyfunction]
pub fn max(_py: Python, iterable: &PyAny) -> PyResult<i32> {
    let r = iterable
        .iter()?
        .map(|elt| {
            let pyint = elt
                .and_then(|r| r.cast_as::<PyInt>().map_err(|r| r.into()))
                .and_then(|r| r.extract().map_err(|r| r.into()));

            if pyint.is_err() {
                return 0; // TODO: must be returned Err
            }

            pyint.unwrap()
        })
        .max();

    Ok(r.unwrap())
}

#[pyfunction]
pub fn join(_py: Python, iterable: &PyAny, sep: &str) -> PyResult<String> {
    let r = iterable.iter()?.map(|elt| {
        let pystr = elt
            .and_then(|r| r.cast_as::<PyString>().map_err(|r| r.into()))
            .and_then(|r| r.extract().map_err(|r| r.into()));

        if pystr.is_err() {
            return ""; // TODO: must be returned Err
        }

        pystr.unwrap()
    });

    Ok(itertools::join(r, sep))
}
