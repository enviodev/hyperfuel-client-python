use pyo3::{pyclass, pymethods, PyObject, PyResult};
use skar_format_fuel::Hex;

use crate::types::{Block, Input, Output, Receipt, Transaction};

#[pyclass]
#[pyo3(get_all)]
#[derive(Clone, Debug)]
pub struct QueryResponseArrowData {
    pub blocks: PyObject,
    pub transactions: PyObject,
    pub receipts: PyObject,
    pub inputs: PyObject,
    pub outputs: PyObject,
}

#[pymethods]
impl QueryResponseArrowData {
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
    pub archive_height: Option<u64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: u64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: u64,
    /// Response data in pyarrow format
    pub data: QueryResponseArrowData,
}

#[pymethods]
impl QueryResponseArrow {
    fn __bool__(&self) -> bool {
        self.archive_height.is_some()
            || self.next_block != u64::default()
            || self.total_execution_time != u64::default()
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

#[pymethods]
impl QueryResponseTyped {
    fn __bool__(&self) -> bool {
        self.archive_height.is_some()
            || self.next_block != u64::default()
            || self.total_execution_time != u64::default()
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
pub struct QueryResponseDataTyped {
    pub blocks: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[pymethods]
impl QueryResponseDataTyped {
    fn __bool__(&self) -> bool {
        !self.blocks.is_empty()
            || !self.transactions.is_empty()
            || !self.receipts.is_empty()
            || !self.inputs.is_empty()
            || !self.outputs.is_empty()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

impl From<skar_client_fuel::QueryResponseTyped> for QueryResponseTyped {
    fn from(r: skar_client_fuel::QueryResponseTyped) -> Self {
        let archive_height = r.archive_height;
        let next_block = r.next_block;
        let total_execution_time = r.total_execution_time;
        let data = QueryResponseDataTyped {
            blocks: r.data.blocks.into_iter().map(|b| b.into()).collect(),
            transactions: r.data.transactions.into_iter().map(|b| b.into()).collect(),
            receipts: r.data.receipts.into_iter().map(|b| b.into()).collect(),
            inputs: r.data.inputs.into_iter().map(|b| b.into()).collect(),
            outputs: r.data.outputs.into_iter().map(|b| b.into()).collect(),
        };

        Self {
            archive_height,
            next_block,
            total_execution_time,
            data,
        }
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

#[pymethods]
impl LogResponse {
    fn __bool__(&self) -> bool {
        self.archive_height.is_some()
            || self.next_block != u64::default()
            || self.total_execution_time != u64::default()
            || !self.data.is_empty()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
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

#[pymethods]
impl LogContext {
    fn __bool__(&self) -> bool {
        self.block_height == u64::default()
            || self.receipt_index == u64::default()
            || self.receipt_type == u8::default()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

impl From<skar_client_fuel::LogResponse> for LogResponse {
    fn from(r: skar_client_fuel::LogResponse) -> Self {
        let archive_height = r.archive_height;
        let next_block = r.next_block;
        let total_execution_time = r.total_execution_time;
        let data = r
            .data
            .into_iter()
            .map(|c| LogContext {
                block_height: c.block_height.into(),
                tx_id: c.tx_id.encode_hex(),
                receipt_index: c.receipt_index.into(),
                receipt_type: c.receipt_type.to_u8(),
                contract_id: c.contract_id.map(|i| i.encode_hex()),
                root_contract_id: c.root_contract_id.map(|i| i.encode_hex()),
                ra: c.ra.map(|i| i.into()),
                rb: c.rb.map(|i| i.into()),
                rc: c.rc.map(|i| i.into()),
                rd: c.rd.map(|i| i.into()),
                pc: c.pc.map(|i| i.into()),
                is: c.is.map(|i| i.into()),
                ptr: c.ptr.map(|i| i.into()),
                len: c.len.map(|i| i.into()),
                digest: c.digest.map(|i| i.encode_hex()),
                data: c.data.map(|i| i.encode_hex()),
            })
            .collect();
        Self {
            archive_height,
            next_block,
            total_execution_time,
            data,
        }
    }
}
