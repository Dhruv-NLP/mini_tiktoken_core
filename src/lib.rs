use pyo3::prelude::*;

/// A simple tokenizer class
#[pyclass]
struct SimpleTokenizer;

#[pymethods]
impl SimpleTokenizer {
    #[new]
    fn new() -> Self {
        SimpleTokenizer
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace().map(|s| s.to_string()).collect()
    }
}

/// PyO3 module entry point
/// Python module definition
#[pymodule]
fn mini_tiktoken_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SimpleTokenizer>()?;
    Ok(())
}