# Get all addresses that received a specific asset_id
# This returns fields of all receipts that are type "TransferOut" and 
# have a specific asset_id

import hyperfuel
from hyperfuel import ReceiptField
import asyncio

async def main():
    client = hyperfuel.HyperfuelClient()

    # asset_id we want to get all transfersOut receipts for
    asset_id="0x0000000000000000000000000000000000000000000000000000000000000000"

    query = hyperfuel.Query(
        from_block=0,
        # which receipts to return data from
        receipts=[
            hyperfuel.ReceiptSelection(
                # only return receipts that have `asset_id` field = this asset_id...
                asset_id=[asset_id],
                # ...AND that are type 8 (TransferOut)
                receipt_type=[8] 
            ),
        ],
        # what data we want returned from the receipts we queried for
        field_selection=hyperfuel.FieldSelection(
            receipt=[
                ReceiptField.TX_ID,
                ReceiptField.BLOCK_HEIGHT,
                ReceiptField.RECEIPT_INDEX,
                ReceiptField.TO,
            ]
        )
    )

    res = await client.get_selected_data(query)

    # fields that aren't selected will be None by default
    print("receipts: " + str(res.data.receipts))

asyncio.run(main())
