# Rust command line for Solana gRPC interface

This is a Rust client for Solana using geyser gRPC.

## Unary Commands
   commands that execute then exit after printing response. like `get-version` and `ping`

  - ### GetVersion
    ```shell
    cargo run --bin client  get-version
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
  - ### Ping
    ```shell
      cargo run --bin client  ping -c 18
    ```
    ```text
    response: PongResponse { count: 18 }
    ```  
