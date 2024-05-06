import hypersync_fuel
import asyncio
from hypersync_fuel import BlockField, TransactionField, ReceiptField, InputField, OutputField


QUERY = hypersync_fuel.Query(
    from_block= 8076516,
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


# async def test_send_req():
#     client = hypersync.HypersyncClient()
#     total_time = 0
#     for _ in range(NUM_BENCHMARK_RUNS):
#         start_time = time.time()
#         res = await client.send_req(QUERY)
#         execution_time = (time.time() - start_time) * 1000
#         total_time += execution_time
#     avg_time = total_time / NUM_BENCHMARK_RUNS
#     print(f"send_req time: {format(execution_time, '.9f')}ms")




async def main():
    await test_create_parquet_folder()
    # print("hypersync-client-python")
    # print(f"number of runs for each test: {NUM_BENCHMARK_RUNS}")
    # await test_send_req()
    # await test_send_req_arrow()
    # await test_send_events_req()
    # await test_get_height()
    # await test_decode_logs()
    # await test_decode_events()
    # await test_create_parquet_folder()
    # await test_preset_query_blocks_and_transactions()
    # await test_preset_query_blocks_and_transaction_hashes()
    # await test_preset_query_logs()
    # await test_preset_query_logs_of_event()


asyncio.run(main())
