# Rust command line for Solana gRPC interface

This is a Rust client for Solana using geyser gRPC.

## Unary Commands
   commands that execute then exit after printing response. like `get-version` and `ping`

  - ### GetVersion
    ```shell
    cargo run  get-version
    ```  
    the response would like
    ```json
    {
      "extra": {
        "hostname": "bb4a479daa69"
      },
      "version": {
        "buildts": "2025-04-09T13:11:41.212695369Z",
        "git": "5518728",
        "package": "yellowstone-grpc-geyser",
        "proto": "5.0.0+solana.2.1.16",
        "rustc": "1.81.0",
        "solana": "2.1.16",
        "version": "5.0.1+solana.2.1.16"
      }
    }
    ```
  - ### Health Check of gRPC server
    ```shell
      cargo run health-check 
    ```
    ```text
    response: PongResponse { count: 18 }
    ```  
   - ### Monitor the price of the Token in Raydium
   ```shell
   cargo run subscribe-token-price --account 8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj 
   ```
   ```text
   [2025-05-08T08:52:11Z INFO  yellowstone_grpc_command::args::subscribe_token_price] WSOL Price: 152.86149380610254
   [2025-05-08T08:52:12Z INFO  yellowstone_grpc_command::args::subscribe_token_price] WSOL Price: 152.86240002125396
   [2025-05-08T08:52:13Z INFO  yellowstone_grpc_command::args::subscribe_token_price] WSOL Price: 152.86553629657067
   ```
  - ### Mointor Transactions of given accounts
  ```shell
  cargo run subscribe-tx --account 38PgzpJYu2HkiYvV8qePFakB8tuobPdGm2FFEn7Dpump
  ```
  

