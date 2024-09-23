# reth-exex-template
A Reth Execution Extension (ExEx) based template for developing your own ExEx.

Resources:
- [ExEx examples](https://github.com/paradigmxyz/reth-exex-examples)
- [about ExEx](https://www.paradigm.xyz/2024/05/reth-exex)

This template includes:

- [Reth node](https://github.com/paradigmxyz/reth) binaries to launch node with exex(-es) on mainnet or op-stack chains (optimism, base, etc.). They are served in the same way as in Reth crate
for mainnet and optimism (under the eponymous feature).
- `/.env.example` - Environment, regarded to run on **base mainnet** . For more details see recomendations [here](https://github.com/base-org/node/blob/main/.env.mainnet)
- `/docker-compose.yml` - Example docker configuration for **base mainnet** with pg image

### Configuration
*may not be well covered, it's just a sample*

#### Deploy
1. place your mainnet rpc and beacon nodes in the environment at regarded var keys:
```
OP_NODE_L1_ETH_RPC=
OP_NODE_L1_BEACON=
```
2. configure ports (on .env and docker's YAML),then configure their mapping (on `ports` section in the `/docker-compose.yml`).
3. carefully observe pg containers and other .env vars.
4. create a jwt file and place path (`/path/to/jwt`) to `--authrpc.jwtsecret` arg.

#### Add your own ExEx
See [minimal ExEx](reth-src/exex/bin/reth/src/exex_implementation/minimal.rs) for example implementation. You can also (re)place your own ExEx into [exex_implementation](reth-src/exex/bin/reth/src/exex_implementation) module.
See this readme file "Resources" section on the top for more examples and key principes of execution extension.

### Test
*See Makefile at* `reth-src/exex/Makefile` *for more specific test commands*
```sh
cd reth-src/exex/
make test
```

### Build & Install
```sh
# mainnet
cd reth-src/exex/
make build
make install

# optimism
cd reth-src/exex/
make build-op
make install-op
```
