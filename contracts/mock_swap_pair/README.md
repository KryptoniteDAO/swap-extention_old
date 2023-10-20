#This is a simple contract for Mock swap

1. Each pair contract corresponds to an asset pair
2. If the corresponding asset quantity is offset against the contract, it represents the price of the contract transaction pair


# compile

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```

# deploy

```bash
seid tx wasm store artifacts/mock_swap_pair.wasm -y --from=admin \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ \
--gas=1500000 --gas-prices=0.01usei --broadcast-mode=block
```

# instantiate

```bash
seid tx wasm instantiate 753 '{"asset_infos":[{"native_token":{"denom":"usei"}},{"native_token":{"denom":"factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"}}],"swap_0_to_1_price":"121000000"}' \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=200000 --gas-prices=0.01usei --broadcast-mode=block --label "test" \
--admin sei13xy3940qrar0k82k7fzhjpqaxj0h0tep7cpuxz
```

contract address ： `sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m`

# Configuration and query

Set price

```bash
seid tx wasm execute sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m \
'{"update0_to1_price":{"new_price":"11000000"}}' \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=200000 --gas-prices=0.01usei --broadcast-mode=block
```

exchange
ps: This contract needs to be transferred to a certain amount of usei and usdt

```bash
seid tx wasm execute sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m \
'{"swap":{"offer_asset":{"info":{"native_token":{"denom":"usei"}},"amount":"1000"}}}' \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=200000 --gas-prices=0.01usei --broadcast-mode=block
```

Check price usei -> usdt

```bash
seid query wasm contract-state smart sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m \
	'{"simulation":{"offer_asset":{"info":{"native_token":{"denom":"usei"}},"amount":"1000"}}}' \
	--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --output json
```

Check price usdt -> usei

```bash
seid query wasm contract-state smart sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m \
	'{"simulation":{"offer_asset":{"info":{"native_token":{"denom":"factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"}},"amount":"1000000"}}}' \
	--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --output json
```

transfer

```bash
seid tx bank send admin sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m 20000usei \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=100000 --gas-prices=0.01usei --broadcast-mode=block 
```

```bash
seid tx bank send admin sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m 2000000factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=100000 --gas-prices=0.01usei --broadcast-mode=block 
```

Check address balance

```bash
seid query bank balances sei13xy3940qrar0k82k7fzhjpqaxj0h0tep7cpuxz --chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/
```

```bash
seid query bank balances sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m --chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/
```
