use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(
    Default,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    dict_derive::FromPyObject,
    dict_derive::IntoPyObject,
)]
pub struct ReceiptSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_contract_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_type: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ra: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rb: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rc: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rd: Option<Vec<u64>>,
}

#[derive(
    Default,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    dict_derive::FromPyObject,
    dict_derive::IntoPyObject,
)]
pub struct InputSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<Vec<u8>>,
}

#[derive(
    Default,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    dict_derive::FromPyObject,
    dict_derive::IntoPyObject,
)]
pub struct OutputSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<Vec<u8>>,
}

#[derive(
    Default,
    Debug,
    Clone,
    Serialize,
    Deserialize,
    dict_derive::FromPyObject,
    dict_derive::IntoPyObject,
)]
pub struct FieldSelection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<String>>,
}

#[derive(
    Default,
    Serialize,
    Deserialize,
    Clone,
    Debug,
    dict_derive::FromPyObject,
    dict_derive::IntoPyObject,
)]
pub struct Query {
    /// The block to start the query from
    pub from_block: u64,
    /// The block to end the query at. If not specified, the query will go until the
    ///  end of data. Exclusive, the returned range will be [from_block..to_block).
    ///
    /// The query will return before it reaches this target block if it hits the time limit
    ///  configured on the server. The user should continue their query by putting the
    ///  next_block field in the response into from_block field of their next query. This implements
    ///  pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_block: Option<u64>,
    /// List of receipt selections, the query will return receipts that match any of these selections and
    ///  it will return receipts that are related to the returned objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<ReceiptSelection>>,
    /// List of input selections, the query will return inputs that match any of these selections and
    ///  it will return inputs that are related to the returned objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<InputSelection>>,
    /// List of output selections, the query will return outputs that match any of these selections and
    ///  it will return outputs that are related to the returned objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<OutputSelection>>,
    /// Whether to include all blocks regardless of if they are related to a returned transaction or log. Normally
    ///  the server will return only the blocks that are related to the transaction or logs in the response. But if this
    ///  is set to true, the server will return data for all blocks in the requested range [from_block, to_block).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_blocks: Option<bool>,
    /// Field selection. The user can select which fields they are interested in, requesting less fields will improve
    ///  query execution time and reduce the payload size so the user should always use a minimal number of fields.
    pub field_selection: FieldSelection,
    /// Maximum number of blocks that should be returned, the server might return more blocks than this number but
    ///  it won't overshoot by too much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_blocks: Option<usize>,
    /// Maximum number of transactions that should be returned, the server might return more transactions than this number but
    ///  it won't overshoot by too much.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_transactions: Option<usize>,
}

impl Query {
    pub fn try_convert(&self) -> Result<skar_net_types_fuel::Query> {
        let json = serde_json::to_vec(self).context("serialize to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}

impl TryFrom<skar_net_types_fuel::Query> for Query {
    type Error = anyhow::Error;

    fn try_from(skar_query: skar_net_types_fuel::Query) -> Result<Self> {
        let json = serde_json::to_vec(&skar_query).context("serialize query to json")?;
        serde_json::from_slice(&json).context("parse json")
    }
}
