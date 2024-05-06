use pyo3::{pyclass, pymethods, PyObject, PyResult};

use crate::types::{Block, Input, Output, Receipt, Transaction};

#[pyclass]
#[pyo3(get_all)]
#[derive(Clone, Debug)]
pub struct QueryArrowResponse {
    pub blocks: PyObject,
    pub transactions: PyObject,
    pub receipts: PyObject,
    pub inputs: PyObject,
    pub outputs: PyObject,
}

#[pymethods]
impl QueryArrowResponse {
    /// TODO: if we want to implement this method we could call _is_initialized method,
    /// but we will need py reference
    fn __bool__(&self) -> bool {
        true
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[pyo3(get_all)]
#[derive(Clone, Debug)]
pub struct QueryResponseArrow {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data in pyarrow format
    pub data: QueryArrowResponse,
}

#[pymethods]
impl QueryResponseArrow {
    fn __bool__(&self) -> bool {
        self.archive_height.is_some()
            || self.next_block != i64::default()
            || self.total_execution_time != i64::default()
            || self.data.__bool__()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Clone)]
pub struct QueryResponseTyped {
    /// Current height of the source hypersync instance
    pub archive_height: Option<u64>,
    /// Next block to query for, the responses are paginated so
    /// the caller should continue the query from this block if they
    /// didn't get responses up to the to_block they specified in the Query.
    pub next_block: u64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: u64,
    /// Response data
    pub data: QueryResponseDataTyped,
}

#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Clone)]
pub struct QueryResponseDataTyped {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl From<skar_client_fuel::QueryResponseTyped> for QueryResponseTyped {
    fn from(resp: skar_client_fuel::QueryResponseTyped) -> Self {
        todo!()
    }
}

#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Clone)]
pub struct LogResponse {
    /// Current height of the source hypersync instance
    pub archive_height: Option<u64>,
    /// Next block to query for, the responses are paginated so
    /// the caller should continue the query from this block if they
    /// didn't get responses up to the to_block they specified in the Query.
    pub next_block: u64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: u64,
    /// Response data
    pub data: Vec<LogContext>,
}

/// Contains all the fields needed for decoding plus some additional fields
/// for context.
#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Clone)]
pub struct LogContext {
    pub block_height: u64,
    pub tx_id: String,
    pub receipt_index: u64,
    pub receipt_type: u8,
    pub contract_id: Option<String>,
    pub root_contract_id: Option<String>,
    pub ra: Option<u64>,
    pub rb: Option<u64>,
    pub rc: Option<u64>,
    pub rd: Option<u64>,
    pub pc: Option<u64>,
    pub is: Option<u64>,
    pub ptr: Option<u64>,
    pub len: Option<u64>,
    pub digest: Option<String>,
    pub data: Option<String>,
}

impl From<skar_client_fuel::LogResponse> for LogResponse {
    fn from(resp: skar_client_fuel::LogResponse) -> Self {
        todo!()
    }
}
