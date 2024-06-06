#  We query from blocks 0 (inclusive) to 1299067 (exclusive) for all
#  `Inputs` where the address `0x2a0d0ed9d2217ec7f32dcd9a1902ce2a66d68437aeff84e3a3cc8bebee0d2eea` 
# matches on the `asset_id` field.

import hyperfuel
from hyperfuel import InputField
import asyncio

async def main():
    client = hyperfuel.HyperfuelClient()

    query = hyperfuel.Query(
        # start query from block 0
        from_block=0,
        # if to_block is not set, query runs to the end of the chain
        to_block = 1300000, 
        # load inputs that have `asset_id` = 0x2a0d0ed9d2217ec7f32dcd9a1902ce2a66d68437aeff84e3a3cc8bebee0d2eea
        inputs=[
            hyperfuel.InputSelection(
                asset_id=["0x2a0d0ed9d2217ec7f32dcd9a1902ce2a66d68437aeff84e3a3cc8bebee0d2eea"]
            )
        ],
        # what data we want returned from the inputs we loaded
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
