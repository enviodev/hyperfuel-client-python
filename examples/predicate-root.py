# This example returns `input` field data from an input
# where the `owner` field matches the predicate root 0x48a0f31c78e1c837ff6a885785ceb7c2090f86ed93db3ed2d8821d13739fe981
# `owner` is ["The owning address or predicate root."](https://docs.fuel.network/docs/beta-4/graphql/reference/objects/#inputcoin) of an InputCoin Input type

import hypersync_fuel
from hypersync_fuel import InputField
import asyncio

async def main():
    client = hypersync_fuel.HypersyncClient()

    query = hypersync_fuel.Query(
        from_block=4105960,
        to_block=4106000,
        inputs=[
            hypersync_fuel.InputSelection(
                owner=["0x48a0f31c78e1c837ff6a885785ceb7c2090f86ed93db3ed2d8821d13739fe981"]
            )
        ],
        field_selection=hypersync_fuel.FieldSelection(
            input=[
                InputField.TX_ID,
                InputField.BLOCK_HEIGHT,
                InputField.INPUT_TYPE,
                InputField.UTXO_ID,
                InputField.OWNER,
                InputField.AMOUNT,
                InputField.ASSET_ID,
                InputField.PREDICATE_GAS_USED,
                InputField.PREDICATE,
                InputField.PREDICATE_DATA
            ]
        )
    )

    res = await client.get_selected_data(query)

    print("inputs: " + str(res.data.inputs))

asyncio.run(main())
