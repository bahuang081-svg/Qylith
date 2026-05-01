# Qylith 开发者文档

> 全球第一条 AI 原生抗量子 L1 公链

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Substrate](https://img.shields.io/badge/Substrate-4.0-brightgreen)](https://substrate.io/)

## 概述

Qylith 是基于 Substrate 框架定制的 AI 原生抗量子 L1 公链，集成以下核心技术：

| 技术模块 | 描述 |
|---------|------|
| **FALCON-1024** | 抗量子数字签名算法，基于格密码学 |
| **AEM** | AI Agent Execution Module，原生 AI Agent 执行环境 |
| **混合共识** | BABE + Grandpa，支持量子安全的区块签名 |

## 文档目录

### 🚀 快速开始
- **[快速上手指南](./quick-start.md)** - 30分钟内启动本地节点

### 🔐 FALCON 签名
- **[FALCON 签名集成指南](./falcon-integration.md)** - 抗量子密钥、签名和验证

### 🤖 AEM 开发
- **[AEM 开发指南](./aem-development.md)** - AI Agent 注册、任务提交、声誉系统

### 📜 智能合约
- **[智能合约开发](./smart-contracts.md)** - ink! 合约 + PQC 预编译接口

### 🔌 API 参考
- **[API 参考文档](./api-reference.md)** - JSON-RPC、Substrate RPC 扩展、AEM RPC

### 🛠️ SDK
- **[SDK 文档](./sdk-guide.md)** - qylith-sdk-js 使用指南

## 技术架构

```
┌─────────────────────────────────────────────────────────────┐
│                        Qylith L1                            │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐   │
│  │  Runtime    │  │   AEM       │  │  PQC Precompiles   │   │
│  │  (FRAME)    │  │  Module     │  │  (FALCON)          │   │
│  └─────────────┘  └─────────────┘  └─────────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Substrate Core (Rust)                  │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────────────────┐ │    │
│  │  │  BABE     │ │ Grandpa  │ │  FALCON-1024        │ │    │
│  │  │  (Block)  │ │ (Final)  │ │  (Signatures)       │ │    │
│  │  └──────────┘ └──────────┘ └──────────────────────┘ │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## 环境要求

| 组件 | 最低版本 | 推荐版本 |
|------|---------|---------|
| Rust | 1.70+ | 1.75+ |
| Node.js | 18+ | 20 LTS |
| Cargo | 1.70+ | latest |
| Linux/macOS | - | Ubuntu 22.04 / macOS 14 |

> ⚠️ **Windows 用户**：推荐使用 WSL2 或 Docker

## 快速链接

- [Qylith 官网](https://qylith.io) (建设中)
- [Substrate 文档](https://docs.substrate.io/)
- [Polkadot Wiki](https://wiki.polkadot.network/)
- [FALCON 签名规范](https://falcon-signature.info/)

## 贡献指南

欢迎提交 PR 和 Issue！请参阅 [CONTRIBUTING.md](./CONTRIBUTING.md) 了解详情。

## 许可证

本项目采用 [MIT 许可证](./LICENSE)。
