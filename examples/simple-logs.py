# This example will simply fetch the logs from a vector of contracts over a specified block range.
# returns all log data necessary for fuel's decoder.
import hyperfuel
import asyncio

async def main():
    client = hyperfuel.HyperfuelClient()

    # contract(s) we want logs from
    contracts = ["0x4a2ce054e3e94155f7092f7365b212f7f45105b74819c623744ebcc5d065c6ac"]

    # get logs from blocks 0 (inclusive) to 1627509 (exclusive)
    logs = await client.preset_query_get_logs(contracts, 0, 1627509)

    print("number of logs: " + str(len(logs.data)))
    print("logs: " + str(logs.data))

asyncio.run(main())
