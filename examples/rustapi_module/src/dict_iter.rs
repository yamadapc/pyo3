use pyo3::prelude::*;

use pyo3::exceptions::RuntimeError;
use pyo3::types::PyDict;
use pyo3::pythonrun::{POOL, ReleasePool};

#[pymodinit(test_dict)]
fn test_dict(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DictSize>()?;
    Ok(())
}

#[pyclass]
pub struct DictSize {
    expected: u32,
}

#[pymethods]
impl DictSize {
    #[new]
    fn __new__(obj: &PyRawObject, expected: u32) -> PyResult<()> {
        unsafe {
            let pool: &'static mut ReleasePool = &mut *POOL;

            pool.owned.reserve(10000000);
            pool.borrowed.reserve(10000000);
            // *(pool.pointers).reserve(10000000);
            pool.obj.reserve(10000000);
        }

        obj.init(|| DictSize { expected })
    }

    fn iter_dict(&mut self, _py: Python, dict: &PyDict) -> PyResult<u32> {
        let mut seen = 0u32;
        for (sym, values) in dict.iter() {
            seen += 1;
            println!(
                "{:4}/{:4} iterations:{}=>{}",
                seen, self.expected, sym, values
            );
        }

        if seen == self.expected {
            Ok(seen)
        } else {
            Err(PyErr::new::<RuntimeError, _>(format!(
                "Expected {} iterations - performed {}",
                self.expected, seen
            )))
        }
    }
}
