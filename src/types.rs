use pyo3::{pyclass, pymethods, PyResult};
use skar_format_fuel::Hex;

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
    pub transactions_count: String,
    /// The number of receipt messages in the block.
    pub message_receipt_count: String,
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
    pub application_hash: String,
}

#[pymethods]
impl Block {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
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

#[pymethods]
impl Transaction {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
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
    pub nonce: Option<String>,
    /// Current context if in an internal context. null otherwise
    pub contract_id: Option<String>,
    /// The sub id.
    pub sub_id: Option<String>,
}

#[pymethods]
impl Receipt {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
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

#[pymethods]
impl Input {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
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

#[pymethods]
impl Output {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

impl From<skar_format_fuel::BlockHeader> for Block {
    fn from(b: skar_format_fuel::BlockHeader) -> Self {
        Self {
            id: b.id.encode_hex(),
            da_height: b.da_height.into(),
            transactions_count: b.transactions_count.encode_hex(),
            message_receipt_count: b.message_receipt_count.encode_hex(),
            transactions_root: b.transactions_root.encode_hex(),
            message_receipt_root: b.message_receipt_root.encode_hex(),
            height: b.height.into(),
            prev_root: b.prev_root.encode_hex(),
            time: b.time.into(),
            application_hash: b.application_hash.encode_hex(),
        }
    }
}

impl From<skar_format_fuel::Transaction> for Transaction {
    fn from(t: skar_format_fuel::Transaction) -> Self {
        Self {
            block_height: t.block_height.into(),
            id: t.id.encode_hex(),
            input_asset_ids: t
                .input_asset_ids
                .map(|d| d.into_iter().map(|i| i.encode_hex()).collect()),
            input_contracts: t
                .input_contracts
                .map(|d| d.into_iter().map(|i| i.encode_hex()).collect()),
            input_contract_utxo_id: t.input_contract_utxo_id.map(|d| d.encode_hex()),
            input_contract_balance_root: t.input_contract_balance_root.map(|d| d.encode_hex()),
            input_contract_state_root: t.input_contract_state_root.map(|d| d.encode_hex()),
            input_contract_tx_pointer_block_height: t
                .input_contract_tx_pointer_block_height
                .map(|d| d.into()),
            input_contract_tx_pointer_tx_index: t
                .input_contract_tx_pointer_tx_index
                .map(|d| d.into()),
            input_contract: t.input_contract.map(|d| d.encode_hex()),
            gas_price: t.gas_price.map(|d| d.into()),
            gas_limit: t.gas_limit.map(|d| d.into()),
            maturity: t.maturity.map(|d| d.into()),
            mint_amount: t.mint_amount.map(|d| d.into()),
            mint_asset_id: t.mint_asset_id.map(|d| d.encode_hex()),
            tx_pointer_block_height: t.tx_pointer_block_height.map(|d| d.into()),
            tx_pointer_tx_index: t.tx_pointer_tx_index.map(|d| d.into()),
            tx_type: t.tx_type.to_u8(),
            output_contract_input_index: t.output_contract_input_index.map(|d| d.into()),
            output_contract_balance_root: t.output_contract_balance_root.map(|d| d.encode_hex()),
            output_contract_state_root: t.output_contract_state_root.map(|d| d.encode_hex()),
            witnesses: t.witnesses.map(|d| d.encode_hex()),
            receipts_root: t.receipts_root.map(|d| d.encode_hex()),
            status: t.status.as_u8(),
            time: t.time.into(),
            reason: t.reason.map(|d| d.into()),
            script: t.script.map(|d| d.encode_hex()),
            script_data: t.script_data.map(|d| d.encode_hex()),
            bytecode_witness_index: t.bytecode_witness_index.map(|d| d.into()),
            bytecode_length: t.bytecode_length.map(|d| d.into()),
            salt: t.salt.map(|d| d.encode_hex()),
        }
    }
}

impl From<skar_format_fuel::Receipt> for Receipt {
    fn from(r: skar_format_fuel::Receipt) -> Self {
        Self {
            receipt_index: r.receipt_index.into(),
            root_contract_id: r.root_contract_id.map(|d| d.encode_hex()),
            tx_id: r.tx_id.encode_hex(),
            block_height: r.block_height.into(),
            pc: r.pc.map(|d| d.into()),
            is: r.is.map(|d| d.into()),
            to: r.to.map(|d| d.encode_hex()),
            to_address: r.to_address.map(|d| d.encode_hex()),
            amount: r.amount.map(|d| d.into()),
            asset_id: r.asset_id.map(|d| d.encode_hex()),
            gas: r.gas.map(|d| d.into()),
            param1: r.param1.map(|d| d.into()),
            param2: r.param2.map(|d| d.into()),
            val: r.val.map(|d| d.into()),
            ptr: r.ptr.map(|d| d.into()),
            digest: r.digest.map(|d| d.encode_hex()),
            reason: r.reason.map(|d| d.into()),
            ra: r.ra.map(|d| d.into()),
            rb: r.rb.map(|d| d.into()),
            rc: r.rc.map(|d| d.into()),
            rd: r.rd.map(|d| d.into()),
            len: r.len.map(|d| d.into()),
            receipt_type: r.receipt_type.to_u8(),
            result: r.result.map(|d| d.into()),
            gas_used: r.gas_used.map(|d| d.into()),
            data: r.data.map(|d| d.encode_hex()),
            sender: r.sender.map(|d| d.encode_hex()),
            recipient: r.recipient.map(|d| d.encode_hex()),
            nonce: r.nonce.map(|d| d.encode_hex()),
            contract_id: r.contract_id.map(|d| d.encode_hex()),
            sub_id: r.sub_id.map(|d| d.encode_hex()),
        }
    }
}

impl From<skar_format_fuel::Input> for Input {
    fn from(i: skar_format_fuel::Input) -> Self {
        Self {
            tx_id: i.tx_id.encode_hex(),
            block_height: i.block_height.into(),
            input_type: i.input_type.as_u8(),
            utxo_id: i.utxo_id.map(|d| d.encode_hex()),
            owner: i.owner.map(|d| d.encode_hex()),
            amount: i.amount.map(|d| d.into()),
            asset_id: i.asset_id.map(|d| d.encode_hex()),
            tx_pointer_block_height: i.tx_pointer_block_height.map(|d| d.into()),
            tx_pointer_tx_index: i.tx_pointer_tx_index.map(|d| d.into()),
            witness_index: i.witness_index.map(|d| d.into()),
            predicate_gas_used: i.predicate_gas_used.map(|d| d.into()),
            predicate: i.predicate.map(|d| d.encode_hex()),
            predicate_data: i.predicate_data.map(|d| d.encode_hex()),
            balance_root: i.balance_root.map(|d| d.encode_hex()),
            state_root: i.state_root.map(|d| d.encode_hex()),
            contract: i.contract.map(|d| d.encode_hex()),
            sender: i.sender.map(|d| d.encode_hex()),
            recipient: i.recipient.map(|d| d.encode_hex()),
            nonce: i.nonce.map(|d| d.encode_hex()),
            data: i.data.map(|d| d.encode_hex()),
        }
    }
}

impl From<skar_format_fuel::Output> for Output {
    fn from(r: skar_format_fuel::Output) -> Self {
        Self {
            tx_id: r.tx_id.encode_hex(),
            block_height: r.block_height.into(),
            output_type: r.output_type.as_u8(),
            to: r.to.map(|d| d.encode_hex()),
            amount: r.amount.map(|d| d.into()),
            asset_id: r.asset_id.map(|d| d.encode_hex()),
            input_index: r.input_index.map(|d| d.into()),
            balance_root: r.balance_root.map(|d| d.encode_hex()),
            state_root: r.state_root.map(|d| d.encode_hex()),
            contract: r.contract.map(|d| d.encode_hex()),
        }
    }
}
