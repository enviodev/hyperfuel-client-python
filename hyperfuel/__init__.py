from .hyperfuel import HyperfuelClient as _HyperfuelClient
from typing import Optional
from dataclasses import dataclass, asdict
from strenum import StrEnum


class DataType(StrEnum):
    UINT64 = 'uint64'
    UINT32 = 'uint32'
    INT64 = 'int64'
    INT32 = 'int32'
    FLOAT32 = 'float32'
    FLOAT64 = 'float64'

class BlockField(StrEnum):
    ID = 'id'
    DA_HEIGHT = 'da_height'
    TRANSACTIONS_COUNT = 'transactions_count'
    MESSAGE_RECEIPT_COUNT = 'message_receipt_count'
    TRANSACTIONS_ROOT = 'transactions_root'
    MESSAGE_RECEIPT_ROOT = 'message_receipt_root'
    HEIGHT = 'height'
    PREV_ROOT = 'prev_root'
    TIME = 'time'
    APPLICATION_HASH = 'application_hash'

class TransactionField(StrEnum):
    BLOCK_HEIGHT = 'block_height'
    ID = 'id'
    INPUT_ASSET_IDS = 'input_asset_ids'
    INPUT_CONTRACTS = 'input_contracts'
    INPUT_CONTRACT_UTXO_ID = 'input_contract_utxo_id'
    INPUT_CONTRACT_BALANCE_ROOT = 'input_contract_balance_root'
    INPUT_CONTRACT_STATE_ROOT = 'input_contract_state_root'
    INPUT_CONTRACT_TX_POINTER_BLOCK_HEIGHT = 'input_contract_tx_pointer_block_height'
    INPUT_CONTRACT_TX_POINTER_TX_INDEX = 'input_contract_tx_pointer_tx_index'
    INPUT_CONTRACT = 'input_contract'
    GAS_PRICE = 'gas_price'
    GAS_LIMIT = 'gas_limit'
    MATURITY = 'maturity'
    MINT_AMOUNT = 'mint_amount'
    MINT_ASSET_ID = 'mint_asset_id'
    TX_POINTER_BLOCK_HEIGHT = 'tx_pointer_block_height'
    TX_POINTER_TX_INDEX = 'tx_pointer_tx_index'
    TX_TYPE = 'tx_type'
    OUTPUT_CONTRACT_INPUT_INDEX = 'output_contract_input_index'
    OUTPUT_CONTRACT_BALANCE_ROOT = 'output_contract_balance_root'
    OUTPUT_CONTRACT_STATE_ROOT = 'output_contract_state_root'
    WITNESSES = 'witnesses'
    RECEIPTS_ROOT = 'receipts_root'
    STATUS = 'status'
    TIME = 'time'
    REASON = 'reason'
    SCRIPT = 'script'
    SCRIPT_DATA = 'script_data'
    BYTECODE_WITNESS_INDEX = 'bytecode_witness_index'
    BYTECODE_LENGTH = 'bytecode_length'
    SALT = 'salt'

class ReceiptField(StrEnum):
    RECEIPT_INDEX = 'receipt_index'
    ROOT_CONTRACT_ID = 'root_contract_id'
    TX_ID = 'tx_id'
    BLOCK_HEIGHT = 'block_height'
    PC = 'pc'
    IS = 'is'
    TO = 'to'
    TO_ADDRESS = 'to_address'
    AMOUNT = 'amount'
    ASSET_ID = 'asset_id'
    GAS = 'gas'
    PARAM1 = 'param1'
    PARAM2 = 'param2'
    VAL = 'val'
    PTR = 'ptr'
    DIGEST = 'digest'
    REASON = 'reason'
    RA = 'ra'
    RB = 'rb'
    RC = 'rc'
    RD = 'rd'
    LEN = 'len'
    RECEIPT_TYPE = 'receipt_type'
    RESULT = 'result'
    GAS_USED = 'gas_used'
    DATA = 'data'
    SENDER = 'sender'
    RECIPIENT = 'recipient'
    NONCE = 'nonce'
    CONTRACT_ID = 'contract_id'
    SUB_ID = 'sub_id'

class InputField(StrEnum):
    TX_ID = 'tx_id'
    BLOCK_HEIGHT = 'block_height'
    INPUT_TYPE = 'input_type'
    UTXO_ID = 'utxo_id'
    OWNER = 'owner'
    AMOUNT = 'amount'
    ASSET_ID = 'asset_id'
    TX_POINTER_BLOCK_HEIGHT = 'tx_pointer_block_height'
    TX_POINTER_TX_INDEX = 'tx_pointer_tx_index'
    WITNESS_INDEX = 'witness_index'
    PREDICATE_GAS_USED = 'predicate_gas_used'
    PREDICATE = 'predicate'
    PREDICATE_DATA = 'predicate_data'
    BALANCE_ROOT = 'balance_root'
    STATE_ROOT = 'state_root'
    CONTRACT = 'contract'
    SENDER = 'sender'
    RECIPIENT = 'recipient'
    NONCE = 'nonce'
    DATA = 'data'

class OutputField(StrEnum):
    TX_ID = 'tx_id'
    BLOCK_HEIGHT = 'block_height'
    OUTPUT_TYPE = 'output_type'
    TO = 'to'
    AMOUNT = 'amount'
    ASSET_ID = 'asset_id'
    INPUT_INDEX = 'input_index'
    BALANCE_ROOT = 'balance_root'
    STATE_ROOT = 'state_root'
    CONTRACT = 'contract'


@dataclass
class ReceiptSelection:
    root_contract_id: Optional[list[str]] = None
    to_address: Optional[list[str]] = None
    asset_id: Optional[list[str]] = None
    receipt_type: Optional[list[str]] = None
    sender: Optional[list[str]] = None
    recipient: Optional[list[str]] = None
    contract_id: Optional[list[str]] = None
    ra: Optional[list[str]] = None
    rb: Optional[list[str]] = None
    rc: Optional[list[str]] = None
    rd: Optional[list[str]] = None

@dataclass
class InputSelection:
    owner: Optional[list[str]] = None
    asset_id: Optional[list[str]] = None
    contract: Optional[list[str]] = None
    sender: Optional[list[str]] = None
    recipient: Optional[list[str]] = None
    input_type: Optional[list[str]] = None

@dataclass
class OutputSelection:
    to: Optional[list[str]] = None
    asset_id: Optional[list[str]] = None
    contract: Optional[list[str]] = None
    output_type: Optional[list[str]] = None

@dataclass
class FieldSelection:
    block: Optional[list[BlockField]] = None
    transaction: Optional[list[TransactionField]] = None
    receipt: Optional[list[ReceiptField]] = None
    input: Optional[list[InputField]] = None
    output: Optional[list[OutputField]] = None

@dataclass
class Query:
    from_block: int
    field_selection: FieldSelection
    to_block: Optional[int] = None
    include_all_blocks: Optional[bool] = None
    receipts: Optional[list[ReceiptSelection]] = None
    inputs: Optional[list[InputSelection]] = None
    outputs: Optional[list[OutputSelection]] = None
    max_num_blocks: Optional[int] = None
    max_num_transactions: Optional[int] = None

class HyperfuelClient:
    # Create a new client with given config
    def __init__(self, url="https://fuel-15.hypersync.xyz", bearer_token=None, http_req_timeout_millis=None):
        self.inner = _HyperfuelClient({
            "url": url,
            "bearer_token": bearer_token,
            "http_req_timeout_millis": http_req_timeout_millis
        })

# Create a parquet file by executing a query.
    #
    # If the query can't be finished in a single request, this function will
    # keep on making requests using the pagination mechanism (next_block) until
    # it reaches the end. It will stream data into the parquet file as it comes from
    # the server.
    #
    # Path should point to a folder that will contain the parquet files in the end.
    async def create_parquet_folder(self, query: Query, path: str) -> None:
        return await self.inner.create_parquet_folder(asdict(query), path)

    # Get the height of the source hypersync instance
    async def get_height(self) -> int:    
        return await self.inner.get_height()
        
    # Get the height of the source hypersync instance
    # Internally calls get_height.
    # On an error from the source hypersync instance, sleeps for
    # 1 second (increasing by 1 each failure up to max of 5 seconds)
    # and retries query until success.
    async def get_height_with_retry(self) -> int:
        return await self.inner.get_height_with_retry()
    
    # Send a query request to the source hypersync instance.
    #
    # Returns a query response which contains pyarrow data.
    #
    # NOTE: this query returns loads all transactions that your match your receipt, input, or output selections
    # and applies the field selection to all these loaded transactions.  So your query will return the data you
    # want plus additional data from the loaded transactions.  This functionality is in case you want to associate
    # receipts, inputs, or outputs with eachother.
    async def get_arrow_data(self, query: Query) -> any:
        return await self.inner.get_arrow_data(asdict(query))
    
    # Send a query request to the source hypersync instance.
    # On an error from the source hypersync instance, sleeps for
    # 1 second (increasing by 1 each failure up to max of 5 seconds)
    # and retries query until success.
    #
    # Returns a query response which contains pyarrow data.
    #
    # NOTE: this query returns loads all transactions that your match your receipt, input, or output selections
    # and applies the field selection to all these loaded transactions.  So your query will return the data you
    # want plus additional data from the loaded transactions.  This functionality is in case you want to associate
    # receipts, inputs, or outputs with eachother.
    # Format can be ArrowIpc.
    async def get_arrow_data_with_retry(self, query: Query) -> any:
        return await self.inner.get_arrow_data_with_retry(asdict(query))

    # Send a query request to the source hypersync instance.
    #
    # Returns a query response which contains typed data.
    #
    # NOTE: this query returns loads all transactions that your match your receipt, input, or output selections
    # and applies the field selection to all these loaded transactions.  So your query will return the data you
    # want plus additional data from the loaded transactions.  This functionality is in case you want to associate
    # receipts, inputs, or outputs with eachother.
    async def get_data(self, query: Query) -> any:
        return await self.inner.get_data(asdict(query))

    # Send a query request to the source hypersync instance.
    #
    # Returns a query response that which contains structured data that doesn't include any inputs, outputs,
    # and receipts that don't exactly match the query's input, outout, or receipt selection.
    async def get_selected_data(self, query: Query) -> any:
        return await self.inner.get_selected_data(asdict(query))
    
    # Send a query request to the source hypersync instance.
    #
    # Returns all log and logdata receipts of logs emitted by any of the specified contracts
    # within the block range.
    # If no 'to_block' is specified, query will run to the head of the chain.
    # Returned data contains all the data needed to decode Fuel Log or LogData
    # receipts as well as some extra data for context.  This query doesn't return any logs that
    # were a part of a failed transaction.
    #
    # NOTE: this function is experimental and might be removed in future versions.
    async def preset_query_get_logs(self, emitting_contracts: list[str], from_block: int, to_block: Optional[int]) -> any:
        return await self.inner.preset_query_get_logs(emitting_contracts, from_block, to_block)


    

# helper function for converting a Query object from the rust side interpreted as a dict into a 
# dataclass Query
def dict_to_query(data: dict) -> Query:
    receipts = [ReceiptSelection(**receipt) for receipt in data.get('receipts', [])] if 'receipts' in data else None
    inputs = [InputSelection(**inpt) for inpt in data.get('inputs', [])] if 'inputs' in data else None
    outputs = [OutputSelection(**output) for output in data.get('outputs', [])] if 'outputs' in data else None
    
    field_selection = FieldSelection(
        block=[BlockField(block) for block in data['field_selection'].get('block', [])] if 'block' in data['field_selection'] else None,
        transaction=[TransactionField(txn) for txn in data['field_selection'].get('transaction', [])] if 'transaction' in data['field_selection'] else None,
        receipt=[ReceiptField(receipt) for receipt in data['field_selection'].get('receipt', [])] if 'receipt' in data['field_selection'] else None,
        input=[InputField(inpt) for inpt in data['field_selection'].get('input', [])] if 'input' in data['field_selection'] else None,
        output=[OutputField(output) for output in data['field_selection'].get('output', [])] if 'output' in data['field_selection'] else None,
    )
    
    return Query(
        from_block=data['from_block'],
        to_block=data.get('to_block'),
        receipts=receipts,
        inputs=inputs,
        outputs=outputs,
        include_all_blocks=data.get('include_all_blocks'),
        max_num_blocks=data.get('max_num_blocks'),
        max_num_transactions=data.get('max_num_transactions'),
        field_selection=field_selection,
    )