# Auto-ReQ
Azure Function app to evaluate failed pipeline runs for flaky failures and automatically requeue runs.

## Requirements
- [Azure Functions Core Tools](https://docs.microsoft.com/en-us/azure/azure-functions/functions-run-local#v2)
- [Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli)
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/products/docker-desktop/)

## Building
`cargo build --release`

## Running Locally
`func start`
Then visit http://localhost:7071/api/Auto-ReQ?name=Foo

## Deploying
TODO