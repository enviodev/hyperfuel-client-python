use pyo3::pyclass;

/// The block header contains metadata about a certain block.
#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Block {
    /// String of the header
    pub id: String,
    /// The block height for the data availability layer up to which (inclusive) input messages are processed.
    pub da_height: u64,
    /// The number of transactions in the block.
    pub transactions_count: u64,
    /// The number of receipt messages in the block.
    pub message_receipt_count: u64,
    /// The merkle root of the transactions in the block.
    pub transactions_root: String,
    /// The merkle root of the receipt messages in the block.
    pub message_receipt_root: String,
    /// The block height.
    pub height: u64,
    /// The merkle root of all previous consensus header Stringes (not including this block).
    pub prev_root: String,
    /// The timestamp for the block.
    pub time: u64,
    /// The String of the serialized application header for this block.
    pub application_String: String,
}

/// An object containing information about a transaction.
#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Transaction {
    /// block the transaction is in.
    pub block_height: u64,
    /// A unique transaction id.
    pub id: String,
    /// An array of asset ids used for the transaction inputs.
    pub input_asset_ids: Option<Vec<String>>,
    // Contract object -> bincode into schema
    /// An array of contracts used for the transaction inputs.
    pub input_contracts: Option<Vec<String>>,
    /// A contract used for the transaction input.
    /// A unique 32 byte identifier for the UTXO for a contract used for the transaction input.
    pub input_contract_utxo_id: Option<String>,
    /// The root of amount of coins owned by contract before transaction execution for a contract used for the transaction input.
    pub input_contract_balance_root: Option<String>,
    /// The state root of contract before transaction execution for a contract used for the transaction input.
    pub input_contract_state_root: Option<String>,
    /// A pointer to the TX whose output is being spent for a contract used for the transaction input.
    pub input_contract_tx_pointer_block_height: Option<u64>,
    /// A pointer to the TX whose output is being spent for a contract used for the transaction input.
    pub input_contract_tx_pointer_tx_index: Option<u64>,
    /// The contract id for a contract used for the transaction input.
    pub input_contract: Option<String>,
    /// The gas price for the transaction.
    pub gas_price: Option<u64>,
    /// The gas limit for the transaction.
    pub gas_limit: Option<u64>,
    /// The minimum block height that the transaction can be included at.
    pub maturity: Option<u64>,
    /// The amount minted in the transaction.
    pub mint_amount: Option<u64>,
    /// The asset ID for coins minted in the transaction.
    pub mint_asset_id: Option<String>,
    /// The location of the transaction in the block.
    pub tx_pointer_block_height: Option<u64>,
    pub tx_pointer_tx_index: Option<u64>,
    /// Script, creating a new contract, or minting new coins
    pub tx_type: u8,
    /// The index of the input from a transaction that changed the state of a contract.
    pub output_contract_input_index: Option<u64>,
    /// The root of amount of coins owned by contract after transaction execution from a transaction that changed the state of a contract.
    pub output_contract_balance_root: Option<String>,
    /// The state root of contract after transaction execution from a transaction that changed the state of a contract.
    pub output_contract_state_root: Option<String>,
    /// An array of witnesses.
    pub witnesses: Option<String>,
    /// The root of the receipts.
    pub receipts_root: Option<String>,
    /// The status type of the transaction.
    pub status: u8,
    /// for SubmittedStatus, SuccessStatus, and FailureStatus, the time a transaction was submitted, successful, or failed
    pub time: u64,
    /// for SuccessStatus, the state of the program execution
    // pub program_state: Option<ProgramState>
    /// for SqueezedOutStatus & FailureStatus, the reason the transaction was squeezed out or failed
    pub reason: Option<String>,
    /// The script to execute.
    pub script: Option<String>,
    /// The script input parameters.
    pub script_data: Option<String>,
    /// The witness index of contract bytecode.
    pub bytecode_witness_index: Option<u64>,
    /// The length of the transaction bytecode.
    pub bytecode_length: Option<u64>,
    /// The salt value for the transaction.
    pub salt: Option<String>,
}

/// An object representing all possible types of receipts.
#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Receipt {
    /// Index of the receipt in the block
    pub receipt_index: u64,
    /// Contract that produced the receipt
    pub root_contract_id: Option<String>,
    /// transaction that this receipt originated from
    pub tx_id: String,
    /// block that the receipt originated in
    pub block_height: u64,
    /// The value of the program counter register $pc, which is the memory address of the current instruction.
    pub pc: Option<u64>,
    /// The value of register $is, which is the pointer to the start of the currently-executing code.
    pub is: Option<u64>,
    /// The recipient contract
    pub to: Option<String>,
    /// The recipient address
    pub to_address: Option<String>,
    /// The amount of coins transferred.
    pub amount: Option<u64>,
    /// The asset id of the coins transferred.
    pub asset_id: Option<String>,
    /// The gas used for the transaction.
    pub gas: Option<u64>,
    /// The first parameter for a CALL receipt type, holds the function selector.
    pub param1: Option<u64>,
    /// The second parameter for a CALL receipt type, typically used for the user-specified input to the ABI function being selected.
    pub param2: Option<u64>,
    /// The value of registers at the end of execution, used for debugging.
    pub val: Option<u64>,
    /// The value of the pointer register, used for debugging.
    pub ptr: Option<u64>,
    /// A 32-byte String of MEM[$rC, $rD]. The syntax MEM[x, y] means the memory range starting at byte x, of length y bytes.
    pub digest: Option<String>,
    /// The decimal string representation of an 8-bit unsigned integer for the panic reason. Only returned if the receipt type is PANIC.
    pub reason: Option<u64>,
    /// The value of register $rA.
    pub ra: Option<u64>,
    /// The value of register $rB.
    pub rb: Option<u64>,
    /// The value of register $rC.
    pub rc: Option<u64>,
    /// The value of register $rD.
    pub rd: Option<u64>,
    /// The length of the receipt.
    pub len: Option<u64>,
    /// The type of receipt.
    pub receipt_type: u8,
    /// 0 if script exited successfully, any otherwise.
    pub result: Option<u64>,
    /// The amount of gas consumed by the script.
    pub gas_used: Option<u64>,
    /// The receipt data.
    pub data: Option<String>,
    /// The address of the message sender.
    pub sender: Option<String>,
    /// The address of the message recipient.
    pub recipient: Option<String>,
    /// The nonce value for a message.
    pub nonce: Option<u64>,
    /// Current context if in an internal context. null otherwise
    pub contract_id: Option<String>,
    /// The sub id.
    pub sub_id: Option<String>,
}

/// An object representing all possible types of inputs.  InputCoin, InputContract, InputMessage
#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Input {
    /// transaction that this input originated from
    pub tx_id: String,
    /// block that the input originated in
    pub block_height: u64,
    /// InputCoin, InputContract, or InputMessage
    pub input_type: u8,
    /// A unique 32 byte identifier for the UTXO.
    pub utxo_id: Option<String>,
    /// The owning address or predicate root.
    pub owner: Option<String>,
    /// for InputCoin type: The amount of coins.
    /// for InputMessage type: The amount sent in the message.
    pub amount: Option<u64>,
    /// The asset ID of the coins.
    pub asset_id: Option<String>,
    /// A pointer to the transaction whose output is being spent.
    pub tx_pointer_block_height: Option<u64>,
    pub tx_pointer_tx_index: Option<u64>,
    /// The index of the witness that authorizes spending the coin.
    pub witness_index: Option<u64>,
    /// The amount of gas used in the predicate transaction.
    pub predicate_gas_used: Option<u64>,
    /// The predicate bytecode.
    pub predicate: Option<String>,
    /// The predicate input parameters.
    pub predicate_data: Option<String>,
    /// The root of amount of coins owned by contract before transaction execution.
    pub balance_root: Option<String>,
    /// The state root of contract before transaction execution.
    pub state_root: Option<String>,
    /// The input contract.
    pub contract: Option<String>,
    /// The sender address of the message.
    pub sender: Option<String>,
    /// The recipient address of the message.
    pub recipient: Option<String>,
    /// A nonce value for the message input, which is determined by the sending system and is published at the time the message is sent.
    pub nonce: Option<String>,
    /// The message data.
    pub data: Option<String>,
}

/// An object representing all possible types of Outputs. CoinOutput, ContractOutput, ChangeOutput, VariableOutput, ContractCreated
#[pyclass]
#[pyo3(get_all)]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Output {
    /// transaction that this out originated from
    pub tx_id: String,
    /// block that the output originated in
    pub block_height: u64,
    /// CoinOutput, ContractOutput, ChangeOutput, VariableOutput, or ContractCreated
    pub output_type: u8,
    /// The address the coins were sent to.
    pub to: Option<String>,
    /// The amount of coins in the output.
    pub amount: Option<u64>,
    /// The asset id for the coins sent.
    pub asset_id: Option<String>,
    /// The index of the input.
    pub input_index: Option<u64>,
    /// The root of amount of coins owned by contract after transaction execution.
    pub balance_root: Option<String>,
    /// for ContractedCreated type: The initial state root of contract.
    /// for ContractOutput type: The state root of contract after transaction execution.
    pub state_root: Option<String>,
    /// for ContractCreated type: The contract that was created.
    pub contract: Option<String>,
}
// i64, bool, String, u32
/*
/// Evm log object
///
/// See ethereum rpc spec for the meaning of fields
#[pyclass]
#[pyo3(get_all)]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Log {

}

#[pymethods]
impl Log {
    fn __bool__(&self) -> bool {
        *self != Log::default()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

/// Evm transaction object
///
/// See ethereum rpc spec for the meaning of fields
#[pyclass]
#[pyo3(get_all)]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Transaction {

}

#[pymethods]
impl Transaction {
    fn __bool__(&self) -> bool {
        *self != Transaction::default()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

/// Evm block header object
///

#[pymethods]
impl Block {
    fn __bool__(&self) -> bool {
        *self != Block::default()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}
*/
