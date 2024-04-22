use std::sync::Arc;

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
    fn new_impl(config: Config) -> Result<HypersyncClient> {
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

    /// Create a parquet file by executing a query.
    ///
    /// Path should point to a folder that will contain the parquet files in the end.
    pub fn create_parquet_folder<'py>(
        &'py self,
        query: Query,
        config: ParquetConfig,
        py: Python<'py>,
    ) -> PyResult<&'py PyAny> {
        let inner = Arc::clone(&self.inner);

        future_into_py(py, async move {
            let query = query
                .try_convert()
                .map_err(|_e| PyValueError::new_err("parsing query"))?;

            let config = config
                .try_convert()
                .map_err(|e| PyValueError::new_err(format!("parsing config: {:?}", e)))?;

            inner
                .create_parquet_folder(query, config)
                .await
                .map_err(|e| PyIOError::new_err(format!("{:?}", e)))?;

            Ok(())
        })
    }

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

    /// Returns a query for all Blocks and Transactions within the block range (from_block, to_block]
    /// If to_block is None then query runs to the head of the chain.
    pub fn preset_query_blocks_and_transactions<'py>(
        &'py self,
        py: Python<'py>,
        from_block: u64,
        to_block: Option<u64>,
    ) -> PyResult<PyObject> {
        let query: Query =
            skar_client::Client::preset_query_blocks_and_transactions(from_block, to_block)
                .try_into()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        Ok(query.into_py(py))
    }

    /// Returns a query object for all Blocks and hashes of the Transactions within the block range
    /// (from_block, to_block].  Also returns the block_hash and block_number fields on each Transaction
    /// so it can be mapped to a block.  If to_block is None then query runs to the head of the chain.
    pub fn preset_query_blocks_and_transaction_hashes<'py>(
        &'py self,
        py: Python<'py>,
        from_block: u64,
        to_block: Option<u64>,
    ) -> PyResult<PyObject> {
        let query: Query =
            skar_client::Client::preset_query_blocks_and_transaction_hashes(from_block, to_block)
                .try_into()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        Ok(query.into_py(py))
    }

    /// Returns a query object for all Logs within the block range from the given address.
    /// If to_block is None then query runs to the head of the chain.
    pub fn preset_query_logs<'py>(
        &'py self,
        py: Python<'py>,
        contract_address: &str,
        from_block: u64,
        to_block: Option<u64>,
    ) -> PyResult<PyObject> {
        // cut the "0x" off the address
        let address: &str = if &contract_address[..2] == "0x" {
            &contract_address[2..]
        } else {
            contract_address
        };
        let address = hex_str_address_to_byte_array(address)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        let query: Query = skar_client::Client::preset_query_logs(from_block, to_block, address)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?
            .try_into()
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        Ok(query.into_py(py))
    }

    /// Returns a query for all Logs within the block range from the given address with a
    /// matching topic0 event signature.  Topic0 is the keccak256 hash of the event signature.
    /// If to_block is None then query runs to the head of the chain.
    pub fn preset_query_logs_of_event<'py>(
        &'py self,
        py: Python<'py>,
        contract_address: &str,
        topic0: &str,
        from_block: u64,
        to_block: Option<u64>,
    ) -> PyResult<PyObject> {
        // cut the "0x" off the address
        let address: &str = if &contract_address[..2] == "0x" {
            &contract_address[2..]
        } else {
            contract_address
        };
        let address = hex_str_address_to_byte_array(address)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        // cut the "0x" off the topic0
        let topic0: &str = if &topic0[..2] == "0x" {
            &topic0[2..]
        } else {
            topic0
        };
        let topic0 = hex_str_topic0_to_byte_array(topic0)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        let query: Query =
            skar_client::Client::preset_query_logs_of_event(from_block, to_block, topic0, address)
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?
                .try_into()
                .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;
        Ok(query.into_py(py))
    }
}
