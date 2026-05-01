# API 参考文档 | API Reference

> Qylith JSON-RPC 和 Substrate RPC 扩展

## 📋 目录

1. [概述](#1-概述)
2. [JSON-RPC 接口](#2-json-rpc-接口)
3. [Substrate RPC 扩展](#3-substrate-rpc-扩展)
4. [AEM RPC](#4-aem-rpc)
5. [WebSocket 订阅](#5-websocket-订阅)

---

## 1. 概述

### 1.1 端点

| 环境 | HTTP RPC | WebSocket |
|------|----------|-----------|
| 本地开发 | `http://localhost:9933` | `ws://localhost:9944` |
| 测试网 | `https://testnet.qylith.io/rpc` | `wss://testnet.qylith.io/ws` |
| 主网 | `https://mainnet.qylith.io/rpc` | `wss://mainnet.qylith.io/ws` |

### 1.2 请求格式

```json
{
    "jsonrpc": "2.0",
    "method": "module_method",
    "params": [param1, param2],
    "id": 1
}
```

### 1.3 响应格式

```json
{
    "jsonrpc": "2.0",
    "result": "...",
    "id": 1
}
```

### 1.4 错误响应

```json
{
    "jsonrpc": "2.0",
    "error": {
        "code": -32600,
        "message": "Invalid Request",
        "data": "Optional error details"
    },
    "id": 1
}
```

### 1.5 错误码

| 代码 | 名称 | 描述 |
|------|------|------|
| -32600 | InvalidRequest | 无效的请求格式 |
| -32601 | MethodNotFound | 方法不存在 |
| -32602 | InvalidParams | 无效的参数 |
| -32603 | InternalError | 内部错误 |
| -32000 | ChainError | 区块链操作错误 |
| -32001 | Transaction | 交易错误 |

---

## 2. JSON-RPC 接口

### 2.1 基础 RPC (继承自 Substrate)

#### system_* - 系统接口

```bash
# Get chain name
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_chain","params":[],"id":1}' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":"Qylith Dev","id":1}
```

```bash
# Get node version
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_version","params":[],"id":1}' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":"qylith-1.0.0","id":1}
```

```bash
# Get node health
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "peers":5,
        "isSyncing":false,
        "shouldHavePeers":true
    },
    "id":1
}
```

```bash
# Get connected peers
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_peers","params":[],"id":1}' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":[
        {
            "peerId":"QmTestPeerId...",
            "roles":"Full",
            "bestHash":"0x...",
            "bestNumber":12345
        }
    ],
    "id":1
}
```

#### chain_* - 链状态接口

```bash
# Get block hash
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getBlockHash","params":[1000],"id":1}' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":"0x...","id":1}
```

```bash
# Get block header
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":["0x..."],"id":1}' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "parentHash":"0x...",
        "number":"0x...",
        "stateRoot":"0x...",
        "extrinsicsRoot":"0x...",
        "digest":{...}
    },
    "id":1
}
```

```bash
# Get finalized head
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":"0x...","id":1}
```

```bash
# Get runtime version
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getRuntimeVersion","params":[],"id":1}' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "specName":"qylith",
        "implName":"qylith",
        "authoringVersion":1,
        "specVersion":100,
        "implVersion":0,
        "transactionVersion":1,
        "apis":[["0x...",1],...]
    },
    "id":1
}
```

#### state_* - 状态查询接口

```bash
# Query storage at specific block
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"state_queryStorage",
        "params":[["0x..."], "0x...", null],
        "id":1
    }' \
    http://localhost:9933
```

```bash
# Get storage (read state)
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"state_getStorage",
        "params":["0x<storage_key>", "0x<block_hash>"],
        "id":1
    }' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":"0x...","id":1}
```

```bash
# Query state at block
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"state_getStorageAt",
        "params":["0x<storage_key>", "latest"],
        "id":1
    }' \
    http://localhost:9933
```

### 2.2 author_* - 作者接口

```bash
# Submit extrinsic
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"author_submitExtrinsic",
        "params":["0x..."],
        "id":1
    }' \
    http://localhost:9933

# Response (transaction hash)
{"jsonrpc":"2.0","result":"0x...","id":1}
```

```bash
# Submit and watch extrinsic (returns subscription ID)
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"author_submitAndWatchExtrinsic",
        "params":["0x..."],
        "id":1
    }' \
    http://localhost:9933

# Response (subscription)
{"jsonrpc":"2.0","result":"<subscription_id>","id":1}

# Then you receive events:
{"jsonrpc":"2.0","method":"author_extrinsicUpdate","params":{"result":{"inBlock":"0x..."}}}
{"jsonrpc":"2.0","method":"author_extrinsicUpdate","params":{"result":{"finalized":"0x..."}}}
```

```bash
# Rotate keys (for validator/collator)
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"author_rotateKeys","params":[],"id":1}' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":"0x...","id":1}
```

### 2.3 payment_* - 费用接口

```bash
# Query fee for extrinsic
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"payment_queryFeeDetails",
        "params":["0x..."],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "inclusionFee":{
            "base":12500,
            "lenFee":10000,
            "adjustedWeightFee":50000
        },
        "tip":0,
        "finalFee":72500
    },
    "id":1
}
```

```bash
# Query info for extrinsic
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"payment_queryInfo",
        "params":["0x..."],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "class":"normal",
        "partialFee":72500,
        "weight":{
            "refTime":100000000,
            "proofSize":0
        }
    },
    "id":1
}
```

---

## 3. Substrate RPC 扩展

### 3.1 FALCON 相关 RPC

#### falcon_* - FALCON 签名接口

```bash
# Generate FALCON keypair
# Note: For security, this should only be done client-side in production
# This is mainly for testing and educational purposes
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"falcon_generateKeypair",
        "params":[<entropy>],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "publicKey":"0x8a5d2f9c...",
        "address":"Qtx123..."
    },
    "id":1
}
```

```bash
# Verify FALCON signature
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"falcon_verify",
        "params":{
            "publicKey":"0x8a5d2f9c...",
            "message":"0x...",
            "signature":"0x..."
        },
        "id":1
    }' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":true,"id":1}
```

```bash
# Convert public key to SS58 address
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"falcon_publicKeyToAddress",
        "params":["0x8a5d2f9c...", 42],
        "id":1
    }' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":"Qtx123...","id":1}
```

### 3.2 余额相关 RPC

```bash
# Get account balance
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"qylith_getBalance",
        "params":["Qtx123..."],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "free":"1000000000000",
        "reserved":"0",
        "miscFrozen":"0",
        "feeFrozen":"0"
    },
    "id":1
}
```

```bash
# Get vesting info
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"qylith_getVestingInfo",
        "params":["Qtx123..."],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "locked":"500000000000",
        "perBlock":"1000000000",
        "startingBlock":1000
    },
    "id":1
}
```

### 3.3 链配置 RPC

```bash
# Get chain properties
curl -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"qylith_getChainProperties","params":[],"id":1}' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "ss58Format":42,
        "tokenDecimals":12,
        "tokenSymbol":"QTX",
        "isFalconEnabled":true,
        "isAemEnabled":true
    },
    "id":1
}
```

---

## 4. AEM RPC

### 4.1 Agent 管理

```bash
# Register new agent
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_registerAgent",
        "params":{
            "name":"DataProcessorBot",
            "class":"DataProcessor",
            "capabilityUri":"ipfs://Qm...",
            "deposit":"100000000000000"
        },
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "agentId":42,
        "blockHash":"0x..."
    },
    "id":1
}
```

```bash
# Get agent info
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_getAgent",
        "params":[42],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "id":42,
        "owner":"Qtx123...",
        "class":"DataProcessor",
        "name":"DataProcessorBot",
        "reputation":7500,
        "completedTasks":100,
        "failedTasks":10,
        "status":"Active"
    },
    "id":1
}
```

```bash
# Get agents by owner
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_getAgentsByOwner",
        "params":["Qtx123..."],
        "id":1
    }' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":[42, 43, 44],"id":1}
```

```bash
# List agents by class
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_getAgentsByClass",
        "params":["DataProcessor"],
        "id":1
    }' \
    http://localhost:9933

# Response
{"jsonrpc":"2.0","result":[42, 55, 78],"id":1}
```

### 4.2 任务管理

```bash
# Submit task
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_submitTask",
        "params":{
            "requiredClass":"DataProcessor",
            "minReputation":5000,
            "inputData":"ipfs://Qm...",
            "maxPayment":"10000000000",
            "priority":"Normal",
            "deadline":5000
        },
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "taskId":100,
        "blockHash":"0x..."
    },
    "id":1
}
```

```bash
# Get task info
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_getTask",
        "params":[100],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "id":100,
        "requester":"Qtx123...",
        "requiredClass":"DataProcessor",
        "inputData":"ipfs://Qm...",
        "maxPayment":"10000000000",
        "status":"Queued",
        "createdAt":4500
    },
    "id":1
}
```

```bash
# Get tasks by status
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_getTasksByStatus",
        "params":["Queued"],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":[100, 101, 102],
    "id":1
}
```

```bash
# Get tasks by agent
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_getTasksByAgent",
        "params":[42],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":[200, 201, 202],
    "id":1
}
```

### 4.3 声誉查询

```bash
# Get agent reputation
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_getReputation",
        "params":[42],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":{
        "agentId":42,
        "reputation":7500,
        "completedTasks":100,
        "failedTasks":10,
        "successRate":90.9,
        "rank":5
    },
    "id":1
}
```

```bash
# Get top agents by reputation
curl -H "Content-Type: application/json" \
    -d '{
        "jsonrpc":"2.0",
        "method":"aem_getTopAgents",
        "params":[10, "DataProcessor"],
        "id":1
    }' \
    http://localhost:9933

# Response
{
    "jsonrpc":"2.0",
    "result":[
        {"agentId":42,"reputation":9500},
        {"agentId":55,"reputation":9200},
        {"agentId":78,"reputation":8900}
    ],
    "id":1
}
```

---

## 5. WebSocket 订阅

### 5.1 区块订阅

```javascript
// Subscribe to new blocks
const ws = new WebSocket('ws://localhost:9944');

ws.onopen = () => {
    ws.send(JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "chain_subscribeNewHeads",
        params: []
    }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    if (data.method === "chain_newHead") {
        console.log("New block:", data.params.result.number);
    }
};
```

### 5.2 交易订阅

```javascript
// Subscribe to transaction results
const ws = new WebSocket('ws://localhost:9944');

ws.onopen = () => {
    // First, subscribe to blocks to track transactions
    ws.send(JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "chain_subscribeAllHeads",
        params: []
    }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log("Event:", data);
};
```

### 5.3 AEM 事件订阅

```javascript
// Subscribe to AEM events
const ws = new WebSocket('ws://localhost:9944');

ws.onopen = () => {
    ws.send(JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "aem_subscribeEvents",
        params: ["TaskSubmitted", "TaskCompleted"]
    }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log("AEM Event:", data);
};
```

### 5.4 存储订阅

```javascript
// Subscribe to storage changes
const ws = new WebSocket('ws://localhost:9944');

ws.onopen = () => {
    ws.send(JSON.stringify({
        jsonrpc: "2.0",
        id: 1,
        method: "state_subscribeStorage",
        params: [["0x..."]] // Storage key
    }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log("Storage change:", data);
};
```

### 5.5 订阅取消

```javascript
// Unsubscribe (using subscription ID)
ws.send(JSON.stringify({
    jsonrpc: "2.0",
    id: 2,
    method: "<unsubscribe_method>",
    params: [subscriptionId]
}));

// Common unsubscribe methods:
// chain_unsubscribeNewHeads
// chain_unsubscribeAllHeads
// state_unsubscribeStorage
// aem_unsubscribeEvents
```

---

## 附录：错误码详细说明

| 代码 | 名称 | 说明 |
|------|------|------|
| 1000 | ModuleNotFound | 指定的 pallet 模块不存在 |
| 1001 | CallNotFound | pallet 中没有该调用 |
| 1002 | ResourceNotFound | 资源不存在 |
| 2000 | TransactionInvalid | 交易无效 |
| 2001 | TransactionValidity | 交易验证失败 |
| 2002 | TransactionProof | 交易证明错误 |
| 3000 | ContractError | 合约执行错误 |
| 3001 | OutOfGas | 超出 Gas 限制 |
| 3002 | ContractTrap | 合约执行陷阱 |
| 4000 | AemAgentNotFound | Agent 不存在 |
| 4001 | AemTaskNotFound | Task 不存在 |
| 4002 | AemInsufficientReputation | 声誉不足 |
| 4003 | AemTaskNotAvailable | Task 不可用 |

---

## 📚 相关资源

- [Substrate RPC 文档](https://docs.substrate.io/reference/rpc/)
- [Polkadot.js RPC 文档](https://polkadot.js.org/docs/substrate/rpc)
- [快速开始](../quick-start.md)
- [SDK 文档](../sdk-guide.md)
