# Auto-ReQ
Azure Function app to evaluate failed pipeline runs for flaky failures and automatically requeue runs.

## Requirements
- [Azure Functions Core Tools](https://docs.microsoft.com/en-us/azure/azure-functions/functions-run-local#v2)
- [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli)
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/products/docker-desktop/)
- [openssl & libssl](https://www.openssl.org/source/) (`sudo apt-get install openssl libssl`)

## Building
`cargo build --release`

## Running Locally
`func start`
Then visit http://localhost:7071/api/Auto-ReQ?name=Foo

## Deploying

1. Build to target linux/x86
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/handler .
```
2. Publish to Azure
```
func azure functionapp publish <FunctionAppName> # TODO find out FunctionAppName!
```
3. TODO