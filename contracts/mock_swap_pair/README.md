# 这是一个简单的Mock swap的合约

1. 每一个pair合约对应一个资产对
2. 、往该合约冲对应资产数量，则表示该合约交易对的价格

# 编译

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.11
```

# 部署

```bash
seid tx wasm store artifacts/mock_swap_pair.wasm -y --from=admin \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ \
--gas=1500000 --gas-prices=0.01usei --broadcast-mode=block
```

# 实例化

```bash
seid tx wasm instantiate 753 '{"asset_infos":[{"native_token":{"denom":"usei"}},{"native_token":{"denom":"factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"}}],"swap_0_to_1_price":"121000000"}' \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=200000 --gas-prices=0.01usei --broadcast-mode=block --label "test" \
--admin sei13xy3940qrar0k82k7fzhjpqaxj0h0tep7cpuxz
```

返回合约地址：`sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m`

# 配置及查询

设置价格

```bash
seid tx wasm execute sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m \
'{"update0_to1_price":{"new_price":"11000000"}}' \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=200000 --gas-prices=0.01usei --broadcast-mode=block
```

兑换
ps: 需要将这个合约转入一定数量的usei以及usdt

```bash
seid tx wasm execute sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m \
'{"swap":{"offer_asset":{"info":{"native_token":{"denom":"usei"}},"amount":"1000"}}}' \
--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --from admin  \
--gas=200000 --gas-prices=0.01usei --broadcast-mode=block
```

查询价格 usei -> usdt

```bash
seid query wasm contract-state smart sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m \
	'{"simulation":{"offer_asset":{"info":{"native_token":{"denom":"usei"}},"amount":"1000"}}}' \
	--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --output json
```

查询价格 usdt -> usei

```bash
seid query wasm contract-state smart sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m \
	'{"simulation":{"offer_asset":{"info":{"native_token":{"denom":"factory/sei1h3ukufh4lhacftdf6kyxzum4p86rcnel35v4jk/usdt"}},"amount":"1000000"}}}' \
	--chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/ --output json
```

转账

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

查询地址余额

```bash
seid query bank balances sei13xy3940qrar0k82k7fzhjpqaxj0h0tep7cpuxz --chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/
```

```bash
seid query bank balances sei1rp6tz0nu2nr0tu2sqykulms2d6cxmvnraectkdnkt9p7uma9hceq8a303m --chain-id atlantic-2 --node https://sei-testnet-2-rpc.brocha.in/
```
