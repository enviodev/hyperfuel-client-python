#  We query from blocks 7980000 (inclusive) to 7980100 (exclusive) for all
#  `Inputs` where the address `0x0000000000000000000000000000000000000000000000000000000000000000` 
# matches on the `asset_id` field.

import hyperfuel
from hyperfuel import InputField
import asyncio

async def main():
    client = hyperfuel.HyperfuelClient()
    query = hyperfuel.HyperfuelClient()

    query = hyperfuel.Query(
        from_block=7980000,
        to_block=7980100,
        inputs=[
            hyperfuel.InputSelection(
                asset_id=["0x0000000000000000000000000000000000000000000000000000000000000000"]
            )
        ],
        field_selection=hyperfuel.FieldSelection(
            input=[
                InputField.TX_ID,
                InputField.BLOCK_HEIGHT,
                InputField.INPUT_TYPE,
                InputField.UTXO_ID,
                InputField.OWNER,
                InputField.AMOUNT,
                InputField.ASSET_ID,
            ]
        )
    )

    res = await client.get_selected_data(query)

    print("inputs: " + str(res.data.inputs))

asyncio.run(main())
