use pyo3::{Py, PyAny, pyclass, pymethods};

#[pyclass]
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    #[pyo3(get, set)]
    pub handler: Py<PyAny>,
    #[pyo3(get, set)]
    pub is_async: bool,
    #[pyo3(get, set)]
    pub number_of_params: u8,
}

#[pymethods]
impl FunctionInfo {
    #[new]
    pub fn new(handler: Py<PyAny>, is_async: bool, number_of_params: u8) -> Self {
        Self {
            handler,
            is_async,
            number_of_params,
        }
    }
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct MessageProcessor {
    pub function: FunctionInfo,
    pub filter: String,
}

#[pymethods]
impl MessageProcessor {
    #[new]
    pub fn new(function: FunctionInfo, filter: String) -> Self {
        Self {
            function,
            filter,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct DataMessage {
    pub message: String,
    pub status: String,
    pub sender: String,
    pub dispatch_time: String,
}

#[pymethods]
impl DataMessage {
    #[new]
    pub fn new(message: String, status: String, sender: String) -> Self {
        let dispatch_time = chrono::Utc::now().to_rfc3339();
        Self {
            message,
            status,
            sender,
            dispatch_time,
        }
    }
}