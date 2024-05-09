# This example will simply fetch the logs from a vector of contracts over a specified block range
import hyperfuel
import asyncio

async def main():
    client = hyperfuel.HyperfuelClient()

    contracts = ["0xff63ad3cdb5fde197dfa2d248330d458bffe631bda65938aa7ab7e37efa561d0"]
    from_block = 8076516
    to_block = 8076517

    logs = await client.preset_query_get_logs(contracts, from_block, to_block)

    print("number of logs: " + str(len(logs.data)))
    print("logs: " + str(logs.data))

asyncio.run(main())
