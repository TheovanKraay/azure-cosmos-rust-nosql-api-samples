---
page_type: sample
languages:
- rust
products:
- rust sdk
description: "Sample code repo for Azure Cosmos DB Rust SDK for NoSQL API"
urlFragment: ""
---

# Azure Cosmos DB Rust NoSQL API Samples

Sample code repository for Azure Cosmos DB Rust SDK for NoSQL API. By cloning and running these samples, and then studying their implementations, you will have examples for sending various requests to Azure Cosmos DB from the Rust SDK via the NoSQL API.

## Contents

| File/folder       | Description                                 |
|-------------------|---------------------------------------------|
| `basic-samples`    | Basic CRUD operations and container management samples. |
| `advanced-samples` | More advanced features and usage examples.  |
| `.gitignore`      | Defines what to ignore at commit time.      |
| `Cargo.toml`      | Cargo configuration file for Rust.          |
| `README.md`       | This README file.                           |
| `LICENSE`         | The license for the sample.                 |

## Prerequisites

* Rust (1.56 or newer)
* Setting up an Azure Cosmos DB account through the Azure Portal. This [guide](https://learn.microsoft.com/azure/cosmos-db/nosql/quickstart-portal) walks you through account creation.
* The hostname and master key for your Azure Cosmos DB account

## Setup

Clone the sample to your PC. You can run the samples using Cargo.

```bash
git clone https://github.com/Azure-Samples/azure-cosmos-rust-nosql-api-samples.git
cd azure-cosmos-rust-nosql-api-samples/basic-samples/container-crud
```

## Running the samples

These environment variables must be set in order to give the samples read/write access to your account:

```bash
export COSMOSDB_ENDPOINT=your_account_hostname
```

To run a sample, use Cargo:

```bash
cargo run
```

This will build and run the selected sample.

## Key concepts

These samples cover a range of Azure Cosmos DB usage topics from more to less basic:
* Basic management of databases and containers
* CRUD operations
* Querying items

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.
