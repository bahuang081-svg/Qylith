# QuantumShield Bridge - MVP原型

> Qylith链第一个抗量子跨链桥 | HTX Genesis黑客松参赛作品

## 🎯 项目概述

QuantumShield Bridge 是连接 Ethereum ↔ Qylith 的抗量子跨链资产转移协议。

### 核心创新

| 特性 | 传统桥接 | QuantumShield |
|------|----------|---------------|
| Ethereum签名 | ECDSA (secp256k1) | ECDSA (secp256k1) |
| Qylith签名 | - | **FALCON-1024** (抗量子) |
| 安全性 | 量子计算机可攻破 | 后量子密码学保护 |
| 原子交换 | HTLC | HTLC + PQC |

### 技术架构

```
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│    Ethereum     │      │    Relayer      │      │     Qylith      │
│                 │      │                 │      │                 │
│  ┌───────────┐  │      │  ┌───────────┐  │      │  ┌───────────┐  │
│  │QuantumBr- │  │──────▶│  │Listener   │  │──────▶│  │QuantumBr- │  │
│  │idgeLock   │  │事件   │  │ + Verify  │  │FALCON │  │idgeMint   │  │
│  └───────────┘  │      │  └───────────┘  │签名   │  └───────────┘  │
│                 │◀──────│               │◀──────│                 │
│  锁定ETH/ERC20  │退款   │  中继验证      │铸造   │  铸造包装资产   │
└─────────────────┘      └─────────────────┘      └─────────────────┘
```

## 🔐 密码学对比

### ECDSA (Ethereum) - 量子脆弱
```
签名长度: 65 bytes (r:32 + s:32 + v:1)
安全性: 128-bit (可被量子计算机攻破)
用途: Ethereum交易签名
```

### FALCON-1024 (Qylith) - 量子安全
```
签名长度: ~666 bytes (NTRU格密码)
安全性: 256-bit post-quantum
特点: 基于格密码，量子计算机无法快速求解
用途: Qylith交易签名
```

## 📁 项目结构

```
quantum-shield-bridge/
├── ethereum/                    # Ethereum智能合约
│   ├── contracts/
│   │   └── QuantumBridgeLock.sol # HTLC锁定合约
│   ├── test/
│   │   └── QuantumBridge.test.ts  # Hardhat测试
│   └── hardhat.config.ts
├── qylith/                       # Qylith合约 (ink!伪代码)
│   └── quantum_bridge_mint.rs
├── relayer/                      # TypeScript中继服务
│   ├── src/
│   │   ├── index.ts              # 入口
│   │   ├── ethereum-listener.ts  # 监听Ethereum事件
│   │   ├── qylith-submitter.ts   # 提交到Qylith
│   │   └── falcon-verifier.ts    # FALCON签名验证
│   └── package.json
├── frontend/                     # React前端
│   ├── src/
│   │   ├── App.tsx
│   │   ├── BridgeForm.tsx
│   │   └── SignatureCompare.tsx
│   └── package.json
└── scripts/                      # 演示脚本
    ├── demo.sh
    └── e2e-test.ts
```

## 🚀 快速开始

### 前置条件
- Node.js 18+
- Hardhat
- MetaMask
- Polkadot.js浏览器插件

### 安装依赖

```bash
# Ethereum合约
cd ethereum && npm install

# Relayer服务
cd ../relayer && npm install

# 前端
cd ../frontend && npm install
```

### 运行测试

```bash
cd ethereum
npx hardhat test
```

### 启动Relayer

```bash
cd relayer
npm run start
```

### 启动前端

```bash
cd frontend
npm run dev
```

## 💡 使用流程

### 跨链转账 (Ethereum → Qylith)

1. 用户在DApp发起跨链转账请求
2. 前端连接MetaMask，用户授权
3. 调用`QuantumBridgeLock.lock()`锁定ETH
4. Relayer监听`AssetLocked`事件
5. Relayer验证ECDSA签名 + 跨链证明
6. Relayer使用FALCON签名提交到Qylith
7. Qylith合约验证FALCON签名，铸造包装ETH
8. 用户在Qylith获得wETH

### 跨链转账 (Qylith → Ethereum)

1. 用户在DApp发起跨链转账请求
2. 前端连接Polkadot.js钱包
3. 调用`lock()`锁定Qylith资产
4. Relayer监听Qylith铸造事件
5. Relayer验证FALCON签名
6. Relayer提交到Ethereum，调用`unlock()`
7. 用户获得原始ETH

## 🔒 安全机制

### HTLC (Hashed Time-Lock Contract)
- **哈希锁**: 防止未授权释放
- **时间锁**: 超时自动退款
- **重放保护**: nonce + chainId

### 签名验证
- Ethereum: EVM原生ecrecover
- Qylith: FALCON签名验证（格密码）

## 📝 注意事项

1. **测试网阶段**: FALCON签名使用模拟数据
2. **主网部署**: 需要完整的Qylith测试网支持
3. **中继者激励**: 需要设计合理的费用机制

## 🏆 HTX Genesis黑客松演示

```bash
# 启动完整演示
./scripts/demo.sh
```

演示流程：
1. 展示前端界面
2. MetaMask连接Ethereum
3. 发起锁定请求
4. 显示Relayer处理过程
5. 展示签名对比（FALCON vs ECDSA）
6. 完成Qylith铸造

## 📄 License

MIT License - HTX Genesis Hackathon 2024

## 👥 Team

QuantumShield Team - 抗量子跨链先驱
