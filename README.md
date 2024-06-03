# hyperfuel-client-python
Python package for [Envio's](https://envio.dev/) Hyperfuel client written in Rust.

More information about Envio's support of Fuel at [docs.envio.dev/docs/hyperfuel](https://docs.envio.dev/docs/hyperfuel).

## quickstart example
```bash
$ pip install hyperfuel
```

```python
# This example will simply fetch the logs from a vector of contracts over a specified block range
import hyperfuel
import asyncio

async def main():
    client = hyperfuel.HyperfuelClient()

    contracts = ["0x4a2ce054e3e94155f7092f7365b212f7f45105b74819c623744ebcc5d065c6ac"]

    logs = await client.preset_query_get_logs(contracts, 0, to_block=None)

    print("number of logs: " + str(len(logs.data)))
    print("logs: " + str(logs.data))

asyncio.run(main())

```

### More examples in `examples/`
