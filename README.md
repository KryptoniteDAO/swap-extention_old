# Swap extention

Encapsulate sparrow swap for easy use by Kryptonite

## InstantiateMsg

The instantiation message takes in the contract `owner`

```json
{
  "owner": "sei13xy3940qrar0k82k7fzhjpqaxj0h0tep7cpuxz"
}
```

## ExecuteMsg

### `update_pair_config`

Updates swap pair config , pair address find
from [sparrow swap](https://github.com/SparrowSwap/sparrowswap-contracts/tree/main/artifacts)

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

### `change_owner`

Change the contract `owner`.

```json
{
  "change_owner": {
    "new_owner": "sei...addr..."
  }
}
```

### `update_pair_status`

Update whether the trading pair pool is available, the default is available, the type is bool.

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

### `update_pair_max_spread`

Set the maximum spread for a single pool, and use the default value of sparrow swap if it is not set by default

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

### 'set_whitelist'

To set those addresses you can call the SwapDenom method

```json
{
  "set_whitelist": {
    "caller": "sei...addr...",
    "is_whitelist": true
  }
}
```

### 'swap_denom'

Swap for assets of denom type

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

## QueryMsg

All query messages are described below. A custom struct is defined for each query response.

### 'query_config'

Returns information about global config.

```json
{
  "query_config": {}
}
```

### 'query_swap_info'

Returns information about a specific swap pair info.

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

### 'query_is_swap_whitelist'

Returns whether the address is in the whitelist.

```json
{
  "query_is_swap_whitelist": {
    "caller": "sei...addr..."
  }
}
```

### 'query_pair_config'

Returns information about a specific swap pair config.

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
