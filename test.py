import hypersync_fuel
import asyncio
from hypersync_fuel import BlockField, TransactionField, ReceiptField, InputField, OutputField


QUERY = hypersync_fuel.Query(
    from_block=8076516,
    to_block=8076517,
    receipts=[
        hypersync_fuel.ReceiptSelection(
            root_contract_id=["0xff63ad3cdb5fde197dfa2d248330d458bffe631bda65938aa7ab7e37efa561d0"],
            receipt_type=[5, 6]
        )
    ],
    field_selection=hypersync_fuel.FieldSelection(
        block=[
            BlockField.HEIGHT,
            BlockField.TRANSACTIONS_ROOT
        ],
        transaction=[
            TransactionField.ID,
            TransactionField.STATUS
        ],
        receipt=[
            ReceiptField.TX_ID,
            ReceiptField.RECEIPT_INDEX,
            ReceiptField.BLOCK_HEIGHT,
            ReceiptField.ROOT_CONTRACT_ID,
            ReceiptField.RA,
            ReceiptField.RB,
            ReceiptField.RC,
            ReceiptField.RD,
            ReceiptField.DATA,
            ReceiptField.RECEIPT_TYPE,
        ],
        input=[
            InputField.TX_ID,
            InputField.OWNER,
        ],
        output=[
            OutputField.ASSET_ID
        ]
    )
)

async def test_create_parquet_folder():
    client = hypersync_fuel.HypersyncClient()
    await client.create_parquet_folder(QUERY, "data")

async def test_get_height():
    client = hypersync_fuel.HypersyncClient()
    height = await client.get_height()
    print("current height: " + str(height))

async def test_get_arrow_data():
    import pyarrow
    client = hypersync_fuel.HypersyncClient()
    res = await client.get_arrow_data(QUERY)
    assert(type(res.data.blocks) == pyarrow.lib.Table)
    assert(res.data.blocks._is_initialized())
    assert(type(res.data.transactions) == pyarrow.lib.Table)
    assert(res.data.transactions._is_initialized())
    assert(type(res.data.receipts) == pyarrow.lib.Table)
    assert(res.data.receipts._is_initialized())
    assert(type(res.data.inputs) == pyarrow.lib.Table)
    assert(res.data.inputs._is_initialized())
    assert(type(res.data.outputs) == pyarrow.lib.Table)
    assert(res.data.outputs._is_initialized())

async def test_get_data():
    client = hypersync_fuel.HypersyncClient()
    res = await client.get_data(QUERY)

async def test_get_selected_data():
    client = hypersync_fuel.HypersyncClient()
    res = await client.get_selected_data(QUERY)

async def test_preset_query_get_logs():
    client = hypersync_fuel.HypersyncClient()
    contracts = ["0xff63ad3cdb5fde197dfa2d248330d458bffe631bda65938aa7ab7e37efa561d0"]
    res = await client.preset_query_get_logs(emitting_contracts=contracts,from_block=8076516,to_block=8076517,)

async def main():
    print("smoke test hypersync-fuel-client-python")
    await test_create_parquet_folder()
    await test_get_height()
    await test_get_arrow_data()
    await test_get_data()
    await test_get_selected_data()
    await test_preset_query_get_logs()

asyncio.run(main())
