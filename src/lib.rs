use anyhow::{Context, Result};
use arrow2::datatypes::Field;
use arrow2::ffi;
use arrow2::{array::StructArray, datatypes::DataType};
use pyo3::ffi::Py_uintptr_t;
use pyo3_asyncio::tokio::future_into_py;
use response::{LogResponse, QueryResponseArrow, QueryResponseArrowData, QueryResponseTyped};
use skar_client_fuel::ArrowBatch;
use std::sync::Arc;

mod config;
mod query;
mod response;
mod types;

use pyo3::{
    exceptions::{PyIOError, PyValueError},
    prelude::*,
};

pub use config::Config;

#[pymodule]
fn hyperfuel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperfuelClient>()
}
#[pyclass]
pub struct HyperfuelClient {
    inner: Arc<skar_client_fuel::Client>,
}

impl HyperfuelClient {
    fn new_impl(config: Config) -> Result<HyperfuelClient> {
        env_logger::try_init().ok();

        let config = config.try_convert().context("parse config")?;

        Ok(HyperfuelClient {
            inner: Arc::new(skar_client_fuel::Client::new(config).context("create client")?),
        })
    }
}

#[pymethods]
impl HyperfuelClient {
    /// Create a new client with given config
    #[new]
    fn new(config: Config) -> PyResult<HyperfuelClient> {
        Self::new_impl(config).map_err(|e| PyIOError::new_err(format!("{:?}", e)))
    }

    /// Get the height of the source hypersync instance
    pub fn get_height<'py>(&'py self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);
        future_into_py::<_, u64>(py, async move {
            let height: u64 = inner
                .get_height()
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            Ok(height)
        })
    }

    /// Get the height of the source hypersync instance
    /// Internally calls get_height.
    /// On an error from the source hypersync instance, sleeps for
    /// 1 second (increasing by 1 each failure up to max of 5 seconds)
    /// and retries query until success.
    pub fn get_height_with_retry<'py>(&'py self, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);
        future_into_py::<_, u64>(py, async move {
            let height: u64 = inner
                .get_height_with_retry()
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            Ok(height)
        })
    }

    /// Create a parquet file by executing a query.
    ///
    /// If the query can't be finished in a single request, this function will
    /// keep on making requests using the pagination mechanism (next_block) until
    /// it reaches the end. It will stream data into the parquet file as it comes from
    /// the server.
    ///
    /// Path should point to a folder that will contain the parquet files in the end.
    pub fn create_parquet_folder<'py>(
        &'py self,
        query: query::Query,
        path: String,
        py: Python<'py>,
    ) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let query = query
                .try_convert()
                .map_err(|_e| PyValueError::new_err("parsing query"))?;

            inner
                .create_parquet_folder(query, path)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            Ok(())
        })
    }

    /// Send a query request to the source hypersync instance.
    ///
    /// Returns a query response which contains typed data.
    ///
    /// NOTE: this query returns loads all transactions that your match your receipt, input, or output selections
    /// and applies the field selection to all these loaded transactions.  So your query will return the data you
    /// want plus additional data from the loaded transactions.  This functionality is in case you want to associate
    /// receipts, inputs, or outputs with eachother.
    pub fn get_data<'py>(&'py self, query: query::Query, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);

        future_into_py::<_, QueryResponseTyped>(py, async move {
            let query = query
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            let res = inner
                .get_data(&query)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            Ok(res.into())
        })
    }

    /// Send a query request to the source hypersync instance.
    ///
    /// Returns a query response that which contains structured data that doesn't include any inputs, outputs,
    /// and receipts that don't exactly match the query's input, outout, or receipt selection.
    pub fn get_selected_data<'py>(
        &'py self,
        query: query::Query,
        py: Python<'py>,
    ) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);

        future_into_py::<_, QueryResponseTyped>(py, async move {
            let query = query
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            let res = inner
                .get_selected_data(&query)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            Ok(res.into())
        })
    }

    /// Send a query request to the source hypersync instance.
    ///
    /// Returns all log and logdata receipts of logs emitted by any of the specified contracts
    /// within the block range.
    /// If no 'to_block' is specified, query will run to the head of the chain.
    /// Returned data contains all the data needed to decode Fuel Log or LogData
    /// receipts as well as some extra data for context.  This query doesn't return any logs that
    /// were a part of a failed transaction.
    ///
    /// NOTE: this function is experimental and might be removed in future versions.
    pub fn preset_query_get_logs<'py>(
        &'py self,
        emitting_contracts: Vec<String>,
        from_block: u64,
        to_block: Option<u64>,
        py: Python<'py>,
    ) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);

        // cut the "0x" off the address
        let mut emitting_contracts_args = vec![];
        for contract_address in emitting_contracts {
            let address: &str = if &contract_address[..2] == "0x" {
                &contract_address[2..]
            } else {
                &contract_address
            };
            let address = hex_str_address_to_byte_array(address)
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
            emitting_contracts_args.push(address)
        }

        future_into_py::<_, LogResponse>(py, async move {
            let res = inner
                .preset_query_get_logs(emitting_contracts_args, from_block, to_block)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            Ok(res.into())
        })
    }

    /// Send a query request to the source hypersync instance.
    ///
    /// Returns a query response which contains pyarrow data.
    ///
    /// NOTE: this query returns loads all transactions that your match your receipt, input, or output selections
    /// and applies the field selection to all these loaded transactions.  So your query will return the data you
    /// want plus additional data from the loaded transactions.  This functionality is in case you want to associate
    /// receipts, inputs, or outputs with eachother.
    pub fn get_arrow_data<'py>(
        &'py self,
        query: query::Query,
        py: Python<'py>,
    ) -> PyResult<&'py PyAny> {
        // initialize an array
        let inner = Arc::clone(&self.inner);

        future_into_py::<_, QueryResponseArrow>(py, async move {
            let query = query
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            let res = inner
                .get_arrow_data(&query)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            let blocks = res.data.blocks;
            let transactions = res.data.transactions;
            let receipts = res.data.receipts;
            let inputs = res.data.inputs;
            let outputs = res.data.outputs;

            let (blocks, transactions, receipts, inputs, outputs) = Python::with_gil(|py| {
                let pyarrow = py.import("pyarrow")?;
                let blocks = convert_batch_to_pyarrow_table(py, pyarrow, blocks)?;
                let transactions = convert_batch_to_pyarrow_table(py, pyarrow, transactions)?;
                let receipts = convert_batch_to_pyarrow_table(py, pyarrow, receipts)?;
                let inputs = convert_batch_to_pyarrow_table(py, pyarrow, inputs)?;
                let outputs = convert_batch_to_pyarrow_table(py, pyarrow, outputs)?;

                Ok::<(PyObject, PyObject, PyObject, PyObject, PyObject), PyErr>((
                    blocks,
                    transactions,
                    receipts,
                    inputs,
                    outputs,
                ))
            })?;

            let query_response = compose_pyarrow_response(
                res.archive_height,
                res.next_block,
                res.total_execution_time,
                blocks,
                transactions,
                receipts,
                inputs,
                outputs,
            )
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            Ok(query_response)
        })
    }

    /// Send a query request to the source hypersync instance.
    /// On an error from the source hypersync instance, sleeps for
    /// 1 second (increasing by 1 each failure up to max of 5 seconds)
    /// and retries query until success.
    ///
    /// Returns a query response which contains pyarrow data.
    ///
    /// NOTE: this query returns loads all transactions that your match your receipt, input, or output selections
    /// and applies the field selection to all these loaded transactions.  So your query will return the data you
    /// want plus additional data from the loaded transactions.  This functionality is in case you want to associate
    /// receipts, inputs, or outputs with eachother.
    /// Format can be ArrowIpc.
    pub fn get_arrow_data_with_retry<'py>(
        &'py self,
        query: query::Query,
        py: Python<'py>,
    ) -> PyResult<&'py PyAny> {
        // initialize an array
        let inner = Arc::clone(&self.inner);

        future_into_py::<_, QueryResponseArrow>(py, async move {
            let query = query
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            let res = inner
                .get_arrow_data_with_retry(&query)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            let blocks = res.data.blocks;
            let transactions = res.data.transactions;
            let receipts = res.data.receipts;
            let inputs = res.data.inputs;
            let outputs = res.data.outputs;

            let (blocks, transactions, receipts, inputs, outputs) = Python::with_gil(|py| {
                let pyarrow = py.import("pyarrow")?;
                let blocks = convert_batch_to_pyarrow_table(py, pyarrow, blocks)?;
                let transactions = convert_batch_to_pyarrow_table(py, pyarrow, transactions)?;
                let receipts = convert_batch_to_pyarrow_table(py, pyarrow, receipts)?;
                let inputs = convert_batch_to_pyarrow_table(py, pyarrow, inputs)?;
                let outputs = convert_batch_to_pyarrow_table(py, pyarrow, outputs)?;

                Ok::<(PyObject, PyObject, PyObject, PyObject, PyObject), PyErr>((
                    blocks,
                    transactions,
                    receipts,
                    inputs,
                    outputs,
                ))
            })?;

            let query_response = compose_pyarrow_response(
                res.archive_height,
                res.next_block,
                res.total_execution_time,
                blocks,
                transactions,
                receipts,
                inputs,
                outputs,
            )
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            Ok(query_response)
        })
    }
}

// helper function to decode hex string as address
fn hex_str_address_to_byte_array(hex_str: &str) -> Result<[u8; 32], String> {
    if hex_str.len() != 64 {
        return Err("address must be 64 hex characters".to_owned());
    }

    let mut dst = [0u8; 32];
    match faster_hex::hex_decode(hex_str.as_bytes(), &mut dst) {
        Ok(()) => Ok(dst),
        Err(e) => Err(format!("Failed to decode hex string: {}", e)),
    }
}

/// Construct response and centralize error mapping for calling function.
fn compose_pyarrow_response(
    archive_height: Option<u64>,
    next_block: u64,
    total_execution_time: u64,
    blocks: PyObject,
    transactions: PyObject,
    receipts: PyObject,
    inputs: PyObject,
    outputs: PyObject,
) -> Result<QueryResponseArrow> {
    Ok(QueryResponseArrow {
        archive_height: archive_height
            .map(|h| h.try_into())
            .transpose()
            .context("convert height")?,
        next_block: next_block.try_into().context("convert next_block")?,
        total_execution_time: total_execution_time
            .try_into()
            .context("convert total_execution_time")?,
        data: QueryResponseArrowData {
            blocks,
            transactions,
            receipts,
            inputs,
            outputs,
        },
    })
}

/// Uses RecordBatchReader to convert vector of ArrayBatch to reader by c-interface
/// and then crates table from this reader with method from_batches
fn convert_batch_to_pyarrow_table<'py>(
    py: Python<'py>,
    pyarrow: &'py PyModule,
    batches: Vec<ArrowBatch>,
) -> PyResult<PyObject> {
    if batches.is_empty() {
        return Ok(py.None());
    }

    let schema = batches.first().unwrap().schema.fields.clone();
    let field = Field::new("a", DataType::Struct(schema), true);

    let mut data = vec![];
    for batch in batches {
        data.push(
            StructArray::new(field.data_type.clone(), batch.chunk.arrays().to_vec(), None).boxed(),
        );
    }

    let iter = Box::new(data.into_iter().map(Ok)) as _;
    let stream = Box::new(ffi::export_iterator(iter, field));
    let py_stream = pyarrow.getattr("RecordBatchReader")?.call_method1(
        "_import_from_c",
        ((&*stream as *const ffi::ArrowArrayStream) as Py_uintptr_t,),
    )?;
    let table = pyarrow
        .getattr("Table")?
        .call_method1("from_batches", (py_stream,))?;

    Ok(table.to_object(py))
}
