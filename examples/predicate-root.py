# This example returns `input` field data from an input
# where the `owner` field matches the predicate root 0x94a8e322ff02baeb1d625e83dadf5ec88870ac801da370d4b15bbd5f0af01169
# `owner` is ["The owning address or predicate root."](https://docs.fuel.network/docs/graphql/reference/objects/#inputcoin) of an InputCoin Input type

import hyperfuel
from hyperfuel import InputField
import asyncio

async def main():
    client = hyperfuel.HyperfuelClient()

    query = hyperfuel.Query(
        # start query from block 0
        from_block = 0,
        # if to_block is not set, query runs to the end of the chain
        to_block=1427625,
        # load inputs that have `owner` = 0x94a8e322ff02baeb1d625e83dadf5ec88870ac801da370d4b15bbd5f0af01169
        inputs=[
            hyperfuel.InputSelection(
                owner=["0x94a8e322ff02baeb1d625e83dadf5ec88870ac801da370d4b15bbd5f0af01169"]
            )
        ],
        # fields we want returned from loaded inputs
        field_selection=hyperfuel.FieldSelection(
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
