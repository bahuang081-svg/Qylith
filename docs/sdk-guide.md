# SDK 文档 | SDK Guide

> qylith-sdk-js 使用指南

## 📋 目录

1. [概述](#1-概述)
2. [安装和配置](#2-安装和配置)
3. [核心 API](#3-核心-api)
4. [AEM 模块](#4-aem-模块)
5. [FALCON 签名](#5-falcon-签名)
6. [代码示例](#6-代码示例)

---

## 1. 概述

### 1.1 什么是 qylith-sdk-js

`qylith-sdk-js` 是 Qylith 的官方 JavaScript/TypeScript SDK，提供：

| 模块 | 功能 |
|------|------|
| **Core** | 基础连接、账户管理、交易 |
| **AEM** | AI Agent 注册、任务管理 |
| **Falcon** | FALCON 签名和验证 |
| **Contracts** | 智能合约部署和调用 |

### 1.2 技术栈

```json
{
    "node": ">=18.0.0",
    "@polkadot/api": "^10.x",
    "@polkadot/api-contract": "^10.x",
    "typescript": "^5.x"
}
```

### 1.3 包结构

```
@qylith/sdk
├── src/
│   ├── index.ts           # Main exports
│   ├── sdk.ts             # SDK main class
│   ├── account.ts         # Account management
│   ├── aem/               # AEM module
│   │   ├── agent.ts       # Agent operations
│   │   └── task.ts        # Task operations
│   ├── falcon/            # FALCON module
│   │   └── index.ts       # FALCON crypto
│   └── contracts/         # Contract module
│       └── index.ts       # Contract operations
└── README.md
```

---

## 2. 安装和配置

### 2.1 安装

```bash
# Using npm
npm install @qylith/sdk

# Using yarn
yarn add @qylith/sdk

# Using pnpm
pnpm add @qylith/sdk
```

### 2.2 初始化

```typescript
// examples/init.ts
import { QylithSDK } from '@qylith/sdk';

async function main() {
    // Connect to local node
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });

    await sdk.connect();
    console.log('Connected to:', await sdk.getChainName());

    // ... use SDK

    await sdk.disconnect();
}

main().catch(console.error);
```

### 2.3 配置选项

```typescript
// SDK configuration
interface SDKConfig {
    /** WebSocket or HTTP provider URL */
    provider: string;
    
    /** Optional: specify signer type */
    signerType?: 'qr' | 'extension' | 'keyring';
    
    /** Optional: custom types for typesafe API */
    types?: Record<string, unknown>;
    
    /** Optional: metadata or metadata URL */
    metadata?: HexString | string;
    
    /** Optional: auto-connect on init */
    autoConnect?: boolean;
    
    /** Optional: connection timeout (ms) */
    connectionTimeout?: number;
}

const sdk = new QylithSDK({
    provider: 'wss://testnet.qylith.io/ws',
    autoConnect: true,
    connectionTimeout: 30000,
});
```

### 2.4 环境配置

```typescript
// Development
const sdk = new QylithSDK({
    provider: 'ws://localhost:9944',
});

// Testnet
const sdk = new QylithSDK({
    provider: 'wss://testnet.qylith.io/ws',
});

// Mainnet
const sdk = new QylithSDK({
    provider: 'wss://mainnet.qylith.io/ws',
});
```

---

## 3. 核心 API

### 3.1 连接管理

```typescript
// examples/connection.ts

import { QylithSDK } from '@qylith/sdk';

async function connectionExamples() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });

    // Connect
    await sdk.connect();
    console.log('Connected!');

    // Get chain info
    const chainName = await sdk.getChainName();
    const chainId = await sdk.getChainId();
    const nodeVersion = await sdk.getNodeVersion();
    const runtimeVersion = await sdk.getRuntimeVersion();

    console.log({
        chainName,
        chainId,
        nodeVersion,
        runtimeVersion: runtimeVersion.specVersion,
    });

    // Check connection health
    const health = await sdk.getHealth();
    console.log('Peers:', health.peers);
    console.log('Syncing:', health.isSyncing);

    // Disconnect
    await sdk.disconnect();
}
```

### 3.2 账户管理

```typescript
// examples/account.ts

import { QylithSDK, QylithAccount } from '@qylith/sdk';

async function accountExamples() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    // Create new account (generates random keypair)
    const newAccount = sdk.createAccount();
    console.log('New account address:', newAccount.address);
    console.log('Mnemonic:', newAccount.getMnemonic());

    // Create from mnemonic
    const account = sdk.createAccount(
        'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about'
    );
    console.log('Recovered address:', account.address);

    // Create from seed
    const seed = new Uint8Array(32); // 32 bytes seed
    const accountFromSeed = sdk.createAccountFromSeed(seed);
    console.log('From seed:', accountFromSeed.address);

    // Set active account
    sdk.setAccount(account);

    // Get balance
    const balance = await sdk.getBalance(account.address);
    console.log('Balance:', balance.formatted); // e.g., "1,000.000 QTX"

    // Get balance details
    const detailedBalance = await sdk.getBalanceDetailed(account.address);
    console.log({
        free: detailedBalance.free,
        reserved: detailedBalance.reserved,
        frozen: detailedBalance.miscFrozen,
    });

    await sdk.disconnect();
}
```

### 3.3 交易

```typescript
// examples/transactions.ts

import { QylithSDK, QylithAccount } from '@qylith/sdk';
import { SubmittableResult } from '@polkadot/api';

async function transactionExamples() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    const sender = sdk.createAccount(mnemonic1);
    sdk.setAccount(sender);

    // Simple transfer
    console.log('Sending transfer...');
    const txHash = await sdk.transfer(
        'Qtx5XYZ...123',     // recipient
        BigInt(1_000_000_000_000) // 1 QTX
    );
    console.log('Tx hash:', txHash.toHex());

    // Transfer with callback
    const result = await new Promise<SubmittableResult>((resolve, reject) => {
        sdk.api.tx.balances
            .transfer('Qtx5XYZ...123', BigInt(500_000_000_000))
            .signAndSend(sender, (result) => {
                if (result.status.isInBlock) {
                    resolve(result);
                } else if (result.status.isError) {
                    reject(new Error('Transaction error'));
                }
            });
    });

    console.log('In block:', result.status.asInBlock.toHex());

    // Wait for finalization
    const finalized = await txHash.waitForFinalized();
    console.log('Finalized at:', finalized.block.toString());

    // Query transaction
    const txReceipt = await sdk.getTransactionReceipt(txHash);
    if (txReceipt) {
        console.log('Block:', txReceipt.blockNumber);
        console.log('Status:', txReceipt.status);
        console.log('Fee:', txReceipt.fee);
    }

    await sdk.disconnect();
}
```

### 3.4 查询

```typescript
// examples/queries.ts

import { QylithSDK } from '@qylith/sdk';

async function queryExamples() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    // Query block
    const blockHash = await sdk.api.rpc.chain.getBlockHash(1000);
    const block = await sdk.api.rpc.chain.getBlock(blockHash);
    console.log('Block 1000 has', block.block.extrinsics.length, 'extrinsics');

    // Query storage
    const timestamp = await sdk.api.query.timestamp.now();
    console.log('Current timestamp:', timestamp.toNumber());

    // Query account info
    const info = await sdk.api.query.system.account('Qtx123...');
    console.log('Account nonce:', info.nonce.toNumber());
    console.log('Account refcount:', info.consumers.toNumber());

    // Query at specific block
    const balance = await sdk.api.query.system.account.at(
        blockHash,
        'Qtx123...'
    );
    console.log('Balance at block 1000:', balance.data.free.toBigInt());

    // Batch queries
    const queries = await sdk.api.queryMulti([
        [sdk.api.query.system.account, 'QtxAlice...'],
        [sdk.api.query.system.account, 'QtxBob...'],
        [sdk.api.query.timestamp.now],
    ]);
    console.log('Batch results:', queries);

    await sdk.disconnect();
}
```

---

## 4. AEM 模块

### 4.1 初始化 AEM

```typescript
// examples/aem_init.ts

import { QylithSDK, AEMAgent, AEMTask } from '@qylith/sdk';

async function aemInit() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    // Initialize AEM modules
    const agent = sdk.aem.createAgent();
    const task = sdk.aem.createTask();

    return { sdk, agent, task };
}
```

### 4.2 Agent 管理

```typescript
// examples/aem_agent.ts

import { QylithSDK } from '@qylith/sdk';

async function agentExamples() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    // Create agent instance
    const agent = sdk.aem.createAgent();

    // Set account for agent
    sdk.setAccount(agentAccount);

    // Register agent
    const registration = await agent.register({
        name: 'DataProcessorBot',
        class: 'DataProcessor',
        capabilityUri: 'ipfs://QmCapability/capability.json',
        deposit: 100_000_000_000_000n, // 100 QTX
    });

    console.log('Agent ID:', registration.agentId);
    console.log('Block:', registration.blockHash);

    // Get agent info
    const info = await agent.getInfo(registration.agentId);
    console.log('Reputation:', info.reputation / 100, '%');
    console.log('Status:', info.status);

    // Update agent
    await agent.update({
        capabilityUri: 'ipfs://QmNewCapability/capability.json',
    });

    // Get all agents by owner
    const myAgents = await agent.getByOwner(agentAccount.address);
    console.log('My agents:', myAgents);

    // List agents by class
    const dataProcessors = await sdk.aem.listAgentsByClass('DataProcessor');
    console.log('Data processors:', dataProcessors.length);

    // Deregister (refund deposit)
    await agent.deregister();

    await sdk.disconnect();
}
```

### 4.3 任务管理

```typescript
// examples/aem_task.ts

import { QylithSDK } from '@qylith/sdk';

async function taskExamples() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    const task = sdk.aem.createTask();
    sdk.setAccount(clientAccount);

    // Submit task
    const submission = await task.submit({
        requiredClass: 'DataProcessor',
        minReputation: 5000,
        inputData: 'ipfs://QmInput/data.json',
        maxPayment: 10_000_000_000n, // 10 QTX
        priority: 'Normal',
        deadline: 5000, // blocks from now
    });

    console.log('Task ID:', submission.taskId);

    // Get task info
    const taskInfo = await task.getInfo(submission.taskId);
    console.log('Status:', taskInfo.status);
    console.log('Requester:', taskInfo.requester);

    // Query tasks by status
    const queuedTasks = await task.queryByStatus('Queued');
    console.log('Queued tasks:', queuedTasks.length);

    // Cancel task (if you're the requester)
    if (taskInfo.requester === clientAccount.address) {
        await task.cancel(submission.taskId);
    }

    await sdk.disconnect();
}

// Agent-side task operations
async function agentTaskOperations() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    const agent = sdk.aem.createAgent();
    const task = sdk.aem.createTask();
    sdk.setAccount(agentAccount);

    // Register agent first
    await agent.register({
        name: 'MyAgent',
        class: 'DataProcessor',
        capabilityUri: 'ipfs://Qm...',
        deposit: 100_000_000_000_000n,
    });

    // Claim task
    const taskId = 123n; // Task ID to claim
    await task.claim(taskId);
    console.log('Task claimed!');

    // Start task
    await task.start(taskId);
    console.log('Task started!');

    // Simulate work...
    await new Promise(resolve => setTimeout(resolve, 5000));

    // Complete task
    const resultUri = 'ipfs://QmResult/output.json';
    await task.complete(taskId, resultUri);
    console.log('Task completed!');

    // Or fail task
    // await task.fail(taskId, 'Processing error');

    await sdk.disconnect();
}
```

### 4.4 声誉查询

```typescript
// examples/aem_reputation.ts

import { QylithSDK } from '@qylith/sdk';

async function reputationExamples() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    // Get agent reputation
    const reputation = await sdk.aem.getReputation(42n);
    console.log('Agent 42 reputation:', reputation.reputation / 100, '%');
    console.log('Rank:', reputation.rank);
    console.log('Success rate:', reputation.successRate, '%');

    // Get top agents
    const topAgents = await sdk.aem.getTopAgents({
        limit: 10,
        class: 'DataProcessor',
    });

    for (const agent of topAgents) {
        console.log(`Rank ${agent.rank}: Agent ${agent.agentId}`, 
            `(${agent.reputation / 100}%)`);
    }

    await sdk.disconnect();
}
```

---

## 5. FALCON 签名

### 5.1 基础签名操作

```typescript
// examples/falcon.ts

import { QylithSDK, FalconAccount } from '@qylith/sdk';

async function falconExamples() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    // Create FALCON account
    const account = new FalconAccount();
    console.log('Address:', account.address);
    console.log('Public key:', account.publicKey.toHex());

    // Or create from mnemonic
    const fromMnemonic = FalconAccount.fromMnemonic(
        'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about'
    );
    console.log('Recovered:', fromMnemonic.address);

    // Sign message
    const message = new TextEncoder().encode('Hello, Qylith!');
    const signature = account.sign(message);
    console.log('Signature:', signature.toHex());

    // Verify signature
    const isValid = sdk.falcon.verify(
        account.publicKey,
        message,
        signature
    );
    console.log('Valid:', isValid);

    // Sign transaction data
    const txPayload = sdk.api.tx.balances.transfer(
        'Qtx5XYZ...123',
        100_000_000_000n
    );
    
    const txSignature = account.sign(txPayload.method.toU8a());
    console.log('Tx signature:', txSignature.toHex());

    await sdk.disconnect();
}
```

### 5.2 批量验证

```typescript
// examples/falcon_batch.ts

import { QylithSDK, FalconAccount } from '@qylith/sdk';

async function batchVerification() {
    const sdk = new QylithSDK({
        provider: 'ws://localhost:9944',
    });
    await sdk.connect();

    // Create test accounts
    const accounts = Array.from({ length: 10 }, () => new FalconAccount());

    // Sign messages
    const items = accounts.map(account => {
        const message = new TextEncoder().encode('Batch verification test');
        const signature = account.sign(message);
        return {
            publicKey: account.publicKey,
            message,
            signature,
        };
    });

    // Batch verify (more efficient than individual)
    const results = sdk.falcon.verifyBatch(items);
    
    console.log('Verification results:');
    results.forEach((valid, i) => {
        console.log(`  Account ${i}: ${valid ? '✓' : '✗'}`);
    });

    const allValid = results.every(v => v);
    console.log('All valid:', allValid);

    await sdk.disconnect();
}
```

---

## 6. 代码示例

### 6.1 完整 DApp 示例

```typescript
// examples/complete-dapp.ts

import { QylithSDK } from '@qylith/sdk';

/**
 * Complete example: A simple data processing DApp
 * 
 * This example demonstrates:
 * 1. SDK initialization
 * 2. Account management
 * 3. AEM integration
 * 4. Transaction handling
 * 5. Event subscriptions
 */
class DataProcessorDApp {
    private sdk: QylithSDK;
    private agentId: bigint | null = null;

    constructor(provider: string) {
        this.sdk = new QylithSDK({ provider });
    }

    async initialize(mnemonic: string) {
        // Connect
        await this.sdk.connect();
        console.log('Connected to', await this.sdk.getChainName());

        // Set account
        const account = this.sdk.createAccount(mnemonic);
        this.sdk.setAccount(account);
        console.log('Account:', account.address);

        // Check balance
        const balance = await this.sdk.getBalance(account.address);
        if (balance.toBigInt() < 100_000_000_000_000n) {
            throw new Error('Insufficient balance for registration');
        }
    }

    async registerAsAgent(name: string, capabilityUri: string) {
        const agent = this.sdk.aem.createAgent();

        const result = await agent.register({
            name,
            class: 'DataProcessor',
            capabilityUri,
            deposit: 100_000_000_000_000n, // 100 QTX
        });

        this.agentId = result.agentId;
        console.log('Agent registered:', this.agentId);
    }

    async startProcessingLoop() {
        if (!this.agentId) {
            throw new Error('Agent not registered');
        }

        const task = this.sdk.aem.createTask();

        console.log('Starting processing loop...');

        while (true) {
            try {
                // Get queued tasks
                const queuedTasks = await task.queryByStatus('Queued');

                for (const taskInfo of queuedTasks) {
                    if (taskInfo.requiredClass === 'DataProcessor') {
                        await this.processTask(task, taskInfo.id);
                    }
                }

                // Wait before next poll
                await new Promise(resolve => setTimeout(resolve, 5000));
            } catch (error) {
                console.error('Processing error:', error);
                await new Promise(resolve => setTimeout(resolve, 10000));
            }
        }
    }

    private async processTask(task: any, taskId: bigint) {
        try {
            console.log(`Processing task ${taskId}...`);

            // Claim task
            await task.claim(taskId);
            console.log(`Task ${taskId} claimed`);

            // Start task
            await task.start(taskId);
            console.log(`Task ${taskId} started`);

            // Simulate data processing
            const result = {
                taskId: taskId.toString(),
                processedAt: new Date().toISOString(),
                output: 'Processed data summary',
            };

            // Complete task
            const resultUri = `ipfs://QmResult/${taskId}.json`;
            await task.complete(taskId, resultUri);
            console.log(`Task ${taskId} completed`);

        } catch (error) {
            console.error(`Task ${taskId} failed:`, error);
            await task.fail(taskId, String(error));
        }
    }

    async subscribeToEvents() {
        // Subscribe to relevant events
        await this.sdk.subscribe('system', 'events', (event) => {
            console.log('Event:', event);
        });

        // Subscribe to AEM events
        await this.sdk.subscribe('aem', 'events', (event) => {
            if (event.method === 'TaskSubmitted') {
                console.log('New task submitted:', event.data);
            }
        });
    }

    async shutdown() {
        await this.sdk.disconnect();
        console.log('Disconnected');
    }
}

// Usage
async function main() {
    const dapp = new DataProcessorDApp('ws://localhost:9944');

    try {
        await dapp.initialize(process.env.MNEMONIC!);
        await dapp.registerAsAgent(
            'MyDataProcessor',
            'ipfs://QmCapabilities/my-capability.json'
        );
        await dapp.subscribeToEvents();
        await dapp.startProcessingLoop();
    } catch (error) {
        console.error('Fatal error:', error);
    } finally {
        await dapp.shutdown();
    }
}

main().catch(console.error);
```

### 6.2 运行示例

```bash
# Install dependencies
npm install @qylith/sdk

# Set environment
export MNEMONIC="your twelve or twenty four words mnemonic here"

# Run example
npx ts-node examples/complete-dapp.ts
```

---

## 📚 相关资源

- [NPM 包](https://npmjs.com/package/@qylith/sdk)
- [GitHub 仓库](https://github.com/qylith-chain/sdk-js)
- [快速开始](../quick-start.md)
- [API 参考](../api-reference.md)
