# hypersync-fuel-client-python
Python package for [Envio's](https://envio.dev/) Hypersync-Fuel client written in Rust.

More information about Envio's Hypersync support of Fuel at [https://github.com/enviodev/hypersync-fuel-docs](https://github.com/enviodev/hypersync-fuel-docs)

## quickstart example
```python
# This example will simply fetch the logs from the given contracts over a block range
# returns all log data necessary for fuel's decoder
import hypersync_fuel
import asyncio

async def main():
    client = hypersync_fuel.HypersyncClient()

    contracts = ["0xff63ad3cdb5fde197dfa2d248330d458bffe631bda65938aa7ab7e37efa561d0"]
    from_block = 8076516
    to_block = 8076517

    logs = await client.preset_query_get_logs(contracts, from_block, to_block)

    print("number of logs: " + str(len(logs.data)))
    print("logs: " + str(logs.data))

asyncio.run(main())

```

### More examples in `examples/`
