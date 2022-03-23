FROM rust:latest AS build-image

WORKDIR /auto-requeue
ADD . /auto-requeue

RUN apt update

RUN cargo build --release

#runtime image
FROM mcr.microsoft.com/azure-functions/dotnet:3.0-appservice

ENV AzureWebJobsScriptRoot=/home/site/wwwroot \
  FUNCTIONS_WORKER_RUNTIME=Rust \
  languageWorkers__workersDirectory=/home/site/wwwroot/workers \
  AzureFunctionsJobHost__Logging__Console__IsEnabled=true

COPY --from=build-image ["/auto-requeue/", "/home/site/wwwroot"]