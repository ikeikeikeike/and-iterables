use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod free;
use crate::free::*;

#[pymodule]
fn and_itertools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(all))?;

    Ok(())
}
