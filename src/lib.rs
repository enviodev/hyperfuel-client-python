use std::sync::Arc;
use anyhow::{Context, Result};

mod config;
mod query;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn hypersync(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HypersyncClient>()
}
#[pyclass]
pub struct HypersyncClient {
    inner: Arc<skar_client::Client>,
}

impl HypersyncClient {
    fn new_impl(config: config::Config) -> Result<HypersyncClient> {
        env_logger::try_init().ok();

        let config = config.try_convert().context("parse config")?;

        Ok(HypersyncClient {
            inner: Arc::new(skar_client::Client::new(config).context("create client")?),
        })
    }
}

#[pymethods]
impl HypersyncClient {
    /// Create a new client with given config
    #[new]
    fn new(config: Config) -> PyResult<HypersyncClient> {
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
    /// Path should point to a folder that will contain the parquet files in the end.
    pub fn create_parquet_folder<'py>(
        &'py self,
        query: Query,
        path: string,
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
    /// Returns a query response which contains structure query response data joined on transaction.
    /// Format can be ArrowIpc.
    pub fn get_data<'py>(&'py, query: Query, py: Python<'py>) -> PyResult<&'py PyAny> {
        
        future_into_py::<_, QueryResponse>(py, async move {
        
            let query = query
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            let res = inner
                .get_data::<skar_client::ArrowIpc>(&query)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            let res = convert_response_to_query_response(res)
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            Ok(res)
        })
    }

    /* Old stuff, for reference
/// Send a query request to the source hypersync instance.
    ///
    /// Returns a query response which contains structure query response data joined on transaction.
    /// Format can be ArrowIpc.
    /// Send a query request to the source hypersync instance.
    ///
    /// Returns a query response which contains block, tx and log data.
    pub fn send_req<'py>(&'py self, query: Query, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);

        future_into_py::<_, QueryResponse>(py, async move {
            let query = query
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            let res = inner
                .send::<skar_client::ArrowIpc>(&query)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            let res = convert_response_to_query_response(res)
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            Ok(res)
        })
    }

    /// Send a query request to the source hypersync instance.
    ///
    /// Returns a query response which contains block, tx and log data in pyarrow table.
    pub fn send_req_arrow<'py>(&'py self, query: Query, py: Python<'py>) -> PyResult<&'py PyAny> {
        // initialize an array
        let inner = Arc::clone(&self.inner);

        future_into_py::<_, QueryResponseArrow>(py, async move {
            let query = query
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            let res = inner
                .send::<skar_client::ArrowIpc>(&query)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            let blocks = res.data.blocks;
            let transactions = res.data.transactions;
            let logs = res.data.logs;

            let (blocks, transactions, logs) = Python::with_gil(|py| {
                let pyarrow = py.import("pyarrow")?;
                let blocks = convert_batch_to_pyarrow_table(py, pyarrow, blocks)?;
                let transactions = convert_batch_to_pyarrow_table(py, pyarrow, transactions)?;
                let logs = convert_batch_to_pyarrow_table(py, pyarrow, logs)?;
                Ok::<(PyObject, PyObject, PyObject), PyErr>((blocks, transactions, logs))
            })?;

            let query_response = compose_pyarrow_response(
                res.archive_height,
                res.next_block,
                res.total_execution_time,
                blocks,
                transactions,
                logs,
            )
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            Ok(query_response)
        })
    }

    /// Send a event query request to the source hypersync instance.
    ///
    /// This executes the same query as send_events_req function on the source side but
    /// it groups data for each event(log) so it is easier to process it.
    pub fn send_events_req<'py>(&'py self, query: Query, py: Python<'py>) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);

        future_into_py::<_, Events>(py, async move {
            let mut query = query
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            if !query.field_selection.block.is_empty() {
                for field in BLOCK_JOIN_FIELDS.iter() {
                    query.field_selection.block.insert(field.to_string());
                }
            }

            if !query.field_selection.transaction.is_empty() {
                for field in TX_JOIN_FIELDS.iter() {
                    query.field_selection.transaction.insert(field.to_string());
                }
            }

            if !query.field_selection.log.is_empty() {
                for field in LOG_JOIN_FIELDS.iter() {
                    query.field_selection.log.insert(field.to_string());
                }
            }

            let res = inner
                .send::<skar_client::ArrowIpc>(&query)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            let res = convert_response_to_events(res)
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

            Ok(res)
        })
    }
    */
}


fn convert_response_to_query_response(res: skar_client::QueryResponse) -> Result<QueryResponse> {
    
    let converted_response = skar_client::transactions_from_arrow_data(res).context("convert arrow data to internal structured type")?;
    // turn converted response (skar-client type) into this crates type
    let query_response = todo!();
    // let blocks = res
    //     .data
    //     .blocks
    //     .iter()
    //     .map(Block::from_arrow)
    //     .collect::<Result<Vec<_>>>()
    //     .context("map blocks from arrow")?
    //     .concat();

    // let transactions = res
    //     .data
    //     .transactions
    //     .iter()
    //     .map(Transaction::from_arrow)
    //     .collect::<Result<Vec<_>>>()
    //     .context("map transactions from arrow")?
    //     .concat();

    // let logs = res
    //     .data
    //     .logs
    //     .iter()
    //     .map(Log::from_arrow)
    //     .collect::<Result<Vec<_>>>()
    //     .context("map logs from arrow")?
    //     .concat();

    Ok(query_response
        /*QueryResponse {
        archive_height: res
            .archive_height
            .map(|h| h.try_into())
            .transpose()
            .context("convert height")?,
        next_block: res.next_block.try_into().context("convert next_block")?,
        total_execution_time: res
            .total_execution_time
            .try_into()
            .context("convert total_execution_time")?,
        data: QueryResponseData {
            blocks,
            transactions,
            logs,
        },
    }*/)
}


#[pyclass]
#[pyo3(get_all)]
#[derive(Clone, Debug)]
pub struct QueryResponse {
    /// Current height of the source hypersync instance
    pub archive_height: Option<i64>,
    /// Next block to query for, the responses are paginated so,
    ///  the caller should continue the query from this block if they
    ///  didn't get responses up to the to_block they specified in the Query.
    pub next_block: i64,
    /// Total time it took the hypersync instance to execute the query.
    pub total_execution_time: i64,
    /// Response data joined on transaction
    pub data:Vec<TransactionContext>,
}

#[pymethods]
impl QueryResponse {
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
#[derive(Clone, Debug)]
pub struct TransactionContext {
    pub block: BlockHeader,
    pub transaction: Transaction,
    pub receipts: Vec<Receipt>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}


// #[pymethods]
// impl QueryResponseData {
//     fn __bool__(&self) -> bool {
//         !self.block.is_empty() || !self.transactions.is_empty() || !self.receipts.is_empty() || !self.inputs.is_empty()
//     }

//     fn __repr__(&self) -> PyResult<String> {
//         Ok(format!("{:?}", self))
//     }

//     fn __str__(&self) -> PyResult<String> {
//         Ok(format!("{:?}", self))
//     }
// }