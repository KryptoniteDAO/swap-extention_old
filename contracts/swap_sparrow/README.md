# Swap extention

Encapsulate [sparrow swap](https://sparrowswap.xyz) for easy use by Kryptonite

## InstantiateMsg {.tabset}

The instantiation message takes in the contract `owner`

### Rust

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
}
```

### JSON

```json
{
  "owner": "sei13xy3940qrar0k82k7fzhjpqaxj0h0tep7cpuxz"
}
```

| Key     | Type     | Description           |
|---------|----------|-----------------------|
| `owner` | `string` | The contract `owner`. |

## ExecuteMsg

### UpdatePairConfig {.tabset}

Updates swap pair config , pair address find
from [sparrow swap](https://github.com/SparrowSwap/sparrowswap-contracts/tree/main/artifacts)

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdatePairConfig {
        asset_infos: [AssetInfo; 2],
        pair_address: Addr,
        max_spread: Option<Decimal>,
        to: Option<Addr>,
    },
}
```

#### JSON

```json
{
  "update_pair_config": {
    "asset_infos": [
      {
        "native_token": {
          "denom": "usei"
        }
      },
      {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      }
    ],
    "pair_address": "sei1pqcgdn5vmf3g9ncs98vtxkydc6su0f9rk3uk73s5ku2xhthr6avswrwnrx"
  }
}
```

| Key            | Type     | Description                                                                 |
|----------------|----------|-----------------------------------------------------------------------------|
| `asset_infos`  | `array`  | The asset infos of the pair.                                                |
| `pair_address` | `string` | The address of the pair contract.                                           |
| `max_spread`   | `string` | The maximum spread of the pair.                                             |
| `to`           | `string` | The address of the contract to receive the swap fees. If not set, use self. |

### ChangeOwner

Change the contract `owner`.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    ChangeOwner {
        new_owner: Addr,
    },
}
```

#### JSON

```json
{
  "change_owner": {
    "new_owner": "sei...addr..."
  }
}
```

| Key         | Type     | Description               |
|-------------|----------|---------------------------|
| `new_owner` | `string` | The new contract `owner`. |

### UpdatePairStatus {.tabset}

Update whether the trading pair pool is available, the default is available, the type is bool.

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdatePairStatus {
        asset_infos: [AssetInfo; 2],
        is_disabled: bool,
    },
}
```

#### JSON

```json
{
  "update_pair_status": {
    "asset_infos": [
      {
        "native_token": {
          "denom": "usei"
        }
      },
      {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      }
    ],
    "is_disabled": false
  }
}
```

| Key           | Type    | Description                  |
|---------------|---------|------------------------------|
| `asset_infos` | `array` | The asset infos of the pair. |
| `is_disabled` | `bool`  | The status of the pair.      |

### UpdatePairMaxSpread {.tabset}

Set the maximum spread for a single pool, and use the default value of sparrow swap if it is not set by default

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    UpdatePairMaxSpread {
        asset_infos: [AssetInfo; 2],
        max_spread: Decimal,
    },
}
```

#### JSON

```json
{
  "update_pair_max_spread": {
    "asset_infos": [
      {
        "native_token": {
          "denom": "usei"
        }
      },
      {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      }
    ],
    "max_spread": 123456
  }
}

```

| Key           | Type     | Description                  |
|---------------|----------|------------------------------|
| `asset_infos` | `array`  | The asset infos of the pair. |
| `max_spread`  | `string` | The max spread of the pair.  |

### SetWhitelist {.tabset}

To set those addresses you can call the SwapDenom method

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    SetWhitelist {
        caller: Addr,
        is_whitelist: bool,
    },
}
```

#### JSON

```json
{
  "set_whitelist": {
    "caller": "sei...addr...",
    "is_whitelist": true
  }
}
```

| Key            | Type    | Description                |
|----------------|---------|----------------------------|
| `caller`       | `array` | The address of the caller. |
| `is_whitelist` | `bool`  | The status of the pair.    |

### SwapDenom

Swap for assets of denom type

#### Rust

```rust
#[cw_serde]
pub enum ExecuteMsg {
    SwapDenom {
        from_coin: Coin,
        target_denom: String,
        to_address: Option<String>,
    },
}
```

#### JSON

```json
{
  "swap_denom": {
    "from_coin": {
      "denom": "usei",
      "amount": "12300"
    },
    "target_denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
  }
}
```

| Key            | Type     | Description                       |
|----------------|----------|-----------------------------------|
| `from_coin`    | `object` | The asset info of the pair.       |
| `target_denom` | `string` | The address of the pair contract. |

## QueryMsg

All query messages are described below. A custom struct is defined for each query response.

### QueryConfig

Returns information about global config.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    QueryConfig {},
}
```

#### JSON

```json
{
  "query_config": {}
}
```

### ConfigResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
}
```

#### JSON

```json
{
  "owner": "sei...addr..."
}
```

| Key     | Type     | Description           |
|---------|----------|-----------------------|
| `owner` | `string` | The contract `owner`. |

### QueryIsSwapWhitelist {.tabset}

Returns whether the address is in the whitelist.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    QueryIsSwapWhitelist {
        caller: Addr
    },
}
```

#### JSON

```json
{
  "query_is_swap_whitelist": {
    "caller": "sei...addr..."
  }
}
```

| Key      | Type     | Description                |
|----------|----------|----------------------------|
| `caller` | `string` | The address of the caller. |

### QueryPairConfig {.tabset}

Returns information about a specific swap pair config.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PairConfigResponse)]
    QueryPairConfig {
        asset_infos: [AssetInfo; 2]
    },
}
```

#### JSON

```json
{
  "query_pair_config": {
    "asset_infos": [
      {
        "native_token": {
          "denom": "usei"
        }
      },
      {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      }
    ]
  }
}
```

| Key           | Type    | Description                  |
|---------------|---------|------------------------------|
| `asset_infos` | `array` | The asset infos of the pair. |

### PairConfigResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct PairConfigResponse {
    pub pair_address: Addr,
    pub is_disabled: bool,
    pub max_spread: Option<Decimal>,
    pub to: Option<Addr>,
}
```

#### JSON

```json
{
  "pair_address": "sei...addr...",
  "is_disabled": false,
  "max_spread": "123456",
  "to": "sei...addr..."
}
```

| Key            | Type     | Description                       |
|----------------|----------|-----------------------------------|
| `pair_address` | `string` | The address of the pair contract. |
| `is_disabled`  | `bool`   | The status of the pair.           |
| `max_spread`   | `Uint128` | The max spread of the pair.       |
| `to`           | `string` | The address of the receiver.      |

### QuerySwapInfo {.tabset}

Returns information about a specific swap pair info.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(SwapInfoResponse)]
    QuerySwapInfo {
        asset_infos: [AssetInfo; 2]
    },
}

```

#### JSON

```json
{
  "query_swap_info": {
    "asset_infos": [
      {
        "native_token": {
          "denom": "usei"
        }
      },
      {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      }
    ]
  }
}
```

| Key           | Type    | Description                  |
|---------------|---------|------------------------------|
| `asset_infos` | `array` | The asset infos of the pair. |

### SwapInfoResponse {.tabset}

#### Rust

```rust
#[cw_serde]
pub struct SwapInfoResponse {
    pub total_amount_in: Uint128,
    pub total_amount_out: Uint128,
}
```

#### JSON

```json
{
  "total_amount_in": "123456",
  "total_amount_out": "123456"
}
```

| Key                | Type     | Description                       |
|--------------------|----------|-----------------------------------|
| `total_amount_in`  | `Uint128` | The total amount in of the pair.  |
| `total_amount_out` | `Uint128` | The total amount out of the pair. |

### QuerySimulation {.tabset}

Returns information about a specific swap simulation.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(SimulationResponse)]
    QuerySimulation {
        asset_infos: [AssetInfo; 2],
        offer_asset: Asset,
    },
}
```

#### JSON

```json
{
  "query_simulation": {
    "asset_infos": [
      {
        "native_token": {
          "denom": "usei"
        }
      },
      {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      }
    ],
    "offer_asset": {
      "info": {
        "native_token": {
          "denom": "usei"
        }
      },
      "amount": "123456"
    }
  }
}
```

| Key           | Type     | Description                  |
|---------------|----------|------------------------------|
| `asset_infos` | `array`  | The asset infos of the pair. |
| `offer_asset` | `object` | The offer asset.             |

### SimulationResponse {.tabset}

#### Rust

```rust
/// This structure holds the parameters that are returned from a swap simulation response
#[cw_serde]
pub struct SimulationResponse {
    /// The amount of ask assets returned by the swap
    pub return_amount: Uint128,
    /// The spread used in the swap operation
    pub spread_amount: Uint128,
    /// The amount of fees charged by the transaction
    pub commission_amount: Uint128,
}
```

#### JSON

```json
{
  "return_amount": "123456",
  "spread_amount": "123456",
  "commission_amount": "123456"
}
```

| Key                 | Type     | Description                        |
|---------------------|----------|------------------------------------|
| `return_amount`     | `Uint128` | The return amount of the swap.     |
| `spread_amount`     | `Uint128` | The spread amount of the swap.     |
| `commission_amount` | `Uint128` | The commission amount of the swap. |

### QueryReverseSimulation {.tabset}

Returns information about a specific reverse swap simulation.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ReverseSimulationResponse)]
    QueryReverseSimulation {
        asset_infos: [AssetInfo; 2],
        ask_asset: Asset,
    },
}
```

#### JSON

```json
{
  "query_reverse_simulation": {
    "asset_infos": [
      {
        "native_token": {
          "denom": "usei"
        }
      },
      {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      }
    ],
    "ask_asset": {
      "info": {
        "native_token": {
          "denom": "usei"
        }
      },
      "amount": "123456"
    }
  }
}
```

| Key           | Type     | Description                  |
|---------------|----------|------------------------------|
| `asset_infos` | `array`  | The asset infos of the pair. |
| `ask_asset`   | `object` | The ask asset.               |

### ReverseSimulationResponse {.tabset}

#### Rust

```rust
/// This structure holds the parameters that are returned from a reverse swap simulation response.
#[cw_serde]
pub struct ReverseSimulationResponse {
    /// The amount of offer assets returned by the reverse swap
    pub offer_amount: Uint128,
    /// The spread used in the swap operation
    pub spread_amount: Uint128,
    /// The amount of fees charged by the transaction
    pub commission_amount: Uint128,
}
```

#### JSON

```json
{
  "offer_amount": "123456",
  "spread_amount": "123456",
  "commission_amount": "123456"
}
```

| Key                 | Type     | Description                        |
|---------------------|----------|------------------------------------|
| `offer_amount`      | `Uint128` | The offer amount of the swap.      |
| `spread_amount`     | `Uint128` | The spread amount of the swap.     |
| `commission_amount` | `Uint128` | The commission amount of the swap. |


### QueryCumulativePrices {.tabset}

Returns information about a specific cumulative prices.

#### Rust

```rust
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CumulativePricesResponse)]
    QueryCumulativePrices {
        asset_infos: [AssetInfo; 2],
    },
}
```

#### JSON

```json
{
  "query_cumulative_prices": {
    "asset_infos": [
      {
        "native_token": {
          "denom": "usei"
        }
      },
      {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      }
    ]
  }
}
```

| Key           | Type    | Description                  |
|---------------|---------|------------------------------|
| `asset_infos` | `array` | The asset infos of the pair. |


### CumulativePricesResponse {.tabset}

#### Rust

```rust
/// This structure is used to return a cumulative prices query response.
#[cw_serde]
pub struct CumulativePricesResponse {
    /// The two assets in the pool to query
    pub assets: [Asset; 2],
    /// The total amount of LP tokens currently issued
    pub total_share: Uint128,
    /// The last value for the token0 cumulative price
    pub price0_cumulative_last: Uint128,
    /// The last value for the token1 cumulative price
    pub price1_cumulative_last: Uint128,
}
```

#### JSON

```json
{
  "assets": [
    {
      "info": {
        "native_token": {
          "denom": "usei"
        }
      },
      "amount": "123456"
    },
    {
      "info": {
        "native_token": {
          "denom": "factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"
        }
      },
      "amount": "123456"
    }
  ],
  "total_share": "123456",
  "price0_cumulative_last": "123456",
  "price1_cumulative_last": "123456"
}
```

| Key                       | Type     | Description                        |
|---------------------------|----------|------------------------------------|
| `assets`                  | `array`  | The assets of the pair.            |
| `total_share`             | `Uint128` | The total share of the pair.       |
| `price0_cumulative_last`  | `Uint128` | The price0 cumulative last of the pair. |
| `price1_cumulative_last`  | `Uint128` | The price1 cumulative last of the pair. |

