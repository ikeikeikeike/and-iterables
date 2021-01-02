use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

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
            let i = elt.and_then(|r| r.extract().map_err(|r| r.into()));

            if i.is_err() {
                return 0; // TODO: must be returned Err
            }

            i.unwrap()
        })
        .max();

    Ok(r.unwrap())
}

#[pyfunction]
pub fn join(_py: Python, iterable: &PyAny, sep: &str) -> PyResult<String> {
    let r = iterable.iter()?.map(|elt| {
        let s = elt.and_then(|r| r.extract().map_err(|r| r.into()));

        if s.is_err() {
            return ""; // TODO: must be returned Err
        }

        s.unwrap()
    });

    Ok(itertools::join(r, sep))
}

#[pyfunction]
pub fn sorted<'a>(_py: Python, iterable: &'a PyAny) -> PyResult<&'a PyAny> {
    // let r = iterable.iter()?.map(itertools::sorted);
    //
    // println!("{:?}", r);

    // let r = iterable.iter()?.map(|elt| {
    //     let r = elt
    //         .and_then(|r| r.cast_as::<PyString>().map_err(|r| r.into()))
    //         .and_then(|r| r.extract().map_err(|r| r.into()));
    //     if !r.is_err() {
    //         return r;
    //     }
    //     let b = elt
    //         .and_then(|r| r.cast_as::<PyInt>().map_err(|r| r.into()))
    //         .and_then(|r| r.extract().map_err(|r| r.into()));
    //     if !b.is_err() {
    //         return b;
    //     }
    //
    //     Err(PyTypeError::new_err("a value cannot cast"))
    // });
    //
    // println!("{:?}", r);

    // let b = r.map(itertools::sorted);

    Ok(iterable)
    // Ok(iterable)
}
