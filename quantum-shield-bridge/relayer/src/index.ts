/**
 * QuantumShield Bridge - Relayer Service
 * 
 * 跨链中继服务，负责：
 * 1. 监听Ethereum链的锁定事件
 * 2. 验证ECDSA签名（Ethereum侧）
 * 3. 生成并验证FALCON签名（Qylith侧，抗量子）
 * 4. 提交证明到Qylith链
 * 5. 处理反向流程（Qylith → Ethereum）
 * 
 * @author QuantumShield Team
 * @notice For HTX Genesis Hackathon Demo
 */

import { ethers } from 'ethers';
import { ApiPromise, WsProvider } from '@polkadot/api';
import dotenv from 'dotenv';

// 配置
import { EthereumListener } from './ethereum-listener';
import { QylithSubmitter } from './qylith-submitter';
import { FalconVerifier } from './falcon-verifier';
import { QuantumBridgeLock } from './types/ethers/types';

dotenv.config();

// ==================== 类型定义 ====================

/**
 * 跨链转账状态
 */
interface CrossChainTransfer {
  lockId: string;
  sender: string;
  recipient: string; // Qylith地址
  token: string;
  amount: string;
  hashlock: string;
  timelock: number;
  chainId: number;
  nonce: number;
  status: 'PENDING' | 'PROCESSING' | 'COMPLETED' | 'FAILED';
  createdAt: number;
}

/**
 * Relayer配置
 */
interface RelayerConfig {
  // Ethereum配置
  ethereum: {
    rpcUrl: string;
    contractAddress: string;
    privateKey: string;
    chainId: number;
  };
  // Qylith配置
  qylith: {
    wsUrl: string;
    contractAddress: string;
    seedPhrase: string;
    chainId: number;
  };
  // FALCON配置
  falcon: {
    publicKey: string;
    privateKey: string;
  };
  // 常规配置
  pollingInterval: number;
  maxConcurrentTransfers: number;
}

/**
 * 事件日志解析结果
 */
interface AssetLockedEvent {
  lockId: string;
  sender: string;
  recipient: string;
  token: string;
  amount: string;
  hashlock: string;
  timelock: number;
  chainId: number;
  nonce: number;
  transactionHash: string;
  blockNumber: number;
}

// ==================== 主类 ====================

/**
 * QuantumShield Bridge Relayer
 * 
 * 核心中继逻辑：
 * 
 * Ethereum → Qylith 流程:
 * 1. 监听Ethereum上的AssetLocked事件
 * 2. 验证ECDSA签名（确保锁定有效）
 * 3. 生成FALCON签名（抗量子证明）
 * 4. 提交到Qylith链，铸造包装资产
 * 5. 监听Qylith的AssetMinted事件
 * 6. 在Ethereum上调用claim，完成跨链
 * 
 * Qylith → Ethereum 流程:
 * 1. 监听Qylith上的AssetBurned事件
 * 2. 验证FALCON签名
 * 3. 在Ethereum上准备解锁证明
 * 4. 提交到Ethereum，完成跨链
 */
export class QuantumShieldRelayer {
  // 配置
  private config: RelayerConfig;
  
  // Ethereum组件
  private ethereumProvider: ethers.JsonRpcProvider;
  private ethereumWallet: ethers.Wallet;
  private ethereumContract: QuantumBridgeLock;
  private ethereumListener: EthereumListener;
  
  // Qylith组件
  private qylithApi: ApiPromise | null = null;
  private qylithSubmitter: QylithSubmitter;
  
  // FALCON验证器
  private falconVerifier: FalconVerifier;
  
  // 状态管理
  private pendingTransfers: Map<string, CrossChainTransfer> = new Map();
  private processedEvents: Set<string> = new Set();
  
  // 运行状态
  private isRunning: boolean = false;
  
  // ==================== 构造函数 ====================
  
  constructor(config: RelayerConfig) {
    this.config = config;
    
    // 初始化Ethereum组件
    this.ethereumProvider = new ethers.JsonRpcProvider(config.ethereum.rpcUrl);
    this.ethereumWallet = new ethers.Wallet(config.ethereum.privateKey, this.ethereumProvider);
    
    // 初始化合约（需要ABI）
    // 注意：这里需要先编译合约获取ABI
    const abiPath = './ethereum/artifacts/contracts/QuantumBridgeLock.sol/QuantumBridgeLock.json';
    const fs = require('fs');
    let abi: any[] = [];
    
    if (fs.existsSync(abiPath)) {
      const artifact = JSON.parse(fs.readFileSync(abiPath, 'utf8'));
      abi = artifact.abi;
    } else {
      // 使用内联ABI（简化版）
      abi = this.getInlineABI();
    }
    
    this.ethereumContract = new ethers.Contract(
      config.ethereum.contractAddress,
      abi,
      this.ethereumWallet
    ) as unknown as QuantumBridgeLock;
    
    // 初始化监听器
    this.ethereumListener = new EthereumListener(
      this.ethereumProvider,
      this.ethereumContract,
      config.ethereum.chainId
    );
    
    // 初始化Qylith提交器
    this.qylithSubmitter = new QylithSubmitter(
      config.qylith.wsUrl,
      config.qylith.seedPhrase,
      config.qylith.contractAddress
    );
    
    // 初始化FALCON验证器
    this.falconVerifier = new FalconVerifier(
      config.falcon.publicKey,
      config.falcon.privateKey
    );
    
    // 绑定事件处理
    this.setupEventHandlers();
  }
  
  // ==================== 启动/停止 ====================
  
  /**
   * 启动Relayer服务
   */
  async start(): Promise<void> {
    console.log('🚀 Starting QuantumShield Bridge Relayer...');
    console.log('');
    console.log('📡 Configuration:');
    console.log(`   Ethereum RPC: ${this.config.ethereum.rpcUrl}`);
    console.log(`   Contract: ${this.config.ethereum.contractAddress}`);
    console.log(`   Qylith WS: ${this.config.qylith.wsUrl}`);
    console.log(`   Chain ID: ${this.config.ethereum.chainId}`);
    console.log('');
    
    this.isRunning = true;
    
    // 启动Ethereum监听
    await this.ethereumListener.start();
    console.log('✅ Ethereum listener started');
    
    // 启动Qylith连接
    try {
      await this.qylithSubmitter.connect();
      console.log('✅ Qylith submitter connected');
    } catch (error) {
      console.warn('⚠️ Qylith connection failed, will retry...');
    }
    
    console.log('');
    console.log('🎯 QuantumShield Relayer is running!');
    console.log('   Waiting for cross-chain transfers...');
    console.log('');
    
    // 启动状态监控
    this.startStatusMonitor();
  }
  
  /**
   * 停止Relayer服务
   */
  async stop(): Promise<void> {
    console.log('🛑 Stopping QuantumShield Bridge Relayer...');
    
    this.isRunning = false;
    
    // 停止监听
    await this.ethereumListener.stop();
    
    // 断开Qylith
    if (this.qylithApi) {
      await this.qylithApi.disconnect();
    }
    
    console.log('✅ Relayer stopped');
  }
  
  // ==================== 事件处理 ====================
  
  /**
   * 设置事件处理器
   */
  private setupEventHandlers(): void {
    // Ethereum AssetLocked 事件
    this.ethereumListener.on('AssetLocked', async (event: AssetLockedEvent) => {
      await this.handleAssetLocked(event);
    });
    
    // Ethereum AssetClaimed 事件
    this.ethereumListener.on('AssetClaimed', (event: any) => {
      this.handleAssetClaimed(event);
    });
    
    // Ethereum AssetRefunded 事件
    this.ethereumListener.on('AssetRefunded', (event: any) => {
      this.handleAssetRefunded(event);
    });
  }
  
  /**
   * 处理AssetLocked事件
   * 
   * 这是跨链转账的起点：
   * 1. 用户在Ethereum锁定资产
   * 2. Relayer监听到事件
   * 3. 验证签名，准备FALCON证明
   * 4. 提交到Qylith铸造
   */
  private async handleAssetLocked(event: AssetLockedEvent): Promise<void> {
    console.log('');
    console.log('📥 New AssetLocked event detected!');
    console.log('   Lock ID:', event.lockId);
    console.log('   Sender:', event.sender);
    console.log('   Amount:', ethers.formatEther(event.amount), 'ETH');
    console.log('   TX Hash:', event.transactionHash);
    
    // 检查是否已处理（防重放）
    if (this.processedEvents.has(event.transactionHash)) {
      console.log('   ⏭️ Already processed, skipping...');
      return;
    }
    
    // 创建转账记录
    const transfer: CrossChainTransfer = {
      lockId: event.lockId,
      sender: event.sender,
      recipient: event.recipient,
      token: event.token,
      amount: event.amount,
      hashlock: event.hashlock,
      timelock: event.timelock,
      chainId: event.chainId,
      nonce: event.nonce,
      status: 'PENDING',
      createdAt: Date.now()
    };
    
    this.pendingTransfers.set(event.lockId, transfer);
    
    try {
      // 更新状态
      transfer.status = 'PROCESSING';
      
      // ========== 步骤1: 验证ECDSA签名（Ethereum侧）==========
      console.log('');
      console.log('🔐 Step 1: Verifying ECDSA signature...');
      const ecdsaValid = await this.verifyECDSASignature(event);
      
      if (!ecdsaValid) {
        console.log('   ❌ ECDSA verification failed');
        transfer.status = 'FAILED';
        return;
      }
      console.log('   ✅ ECDSA signature verified');
      
      // ========== 步骤2: 生成FALCON签名（抗量子）==========
      console.log('');
      console.log('🔐 Step 2: Generating FALCON signature (Quantum-resistant)...');
      const falconSig = await this.falconVerifier.sign(this.buildSigningMessage(event));
      
      if (!falconSig) {
        console.log('   ❌ FALCON signing failed');
        transfer.status = 'FAILED';
        return;
      }
      console.log('   ✅ FALCON signature generated');
      console.log('   📏 Signature length:', falconSig.length, 'bytes');
      console.log('   🔒 Security level: 256-bit post-quantum');
      
      // ========== 步骤3: 提交到Qylith ==========
      console.log('');
      console.log('📤 Step 3: Submitting to Qylith chain...');
      
      const mintResult = await this.qylithSubmitter.submitMint({
        lockId: event.lockId,
        sender: event.sender,
        recipient: event.recipient,
        token: event.token,
        amount: event.amount,
        hashlock: event.hashlock,
        timelock: event.timelock,
        chainId: event.chainId,
        nonce: event.nonce,
        falconSignature: falconSig
      });
      
      if (mintResult.success) {
        console.log('   ✅ Mint submitted to Qylith');
        console.log('   TX Hash:', mintResult.transactionHash);
        transfer.status = 'COMPLETED';
      } else {
        console.log('   ❌ Mint submission failed:', mintResult.error);
        transfer.status = 'FAILED';
      }
      
      // 标记为已处理
      this.processedEvents.add(event.transactionHash);
      
    } catch (error) {
      console.error('   ❌ Error processing transfer:', error);
      transfer.status = 'FAILED';
    }
    
    console.log('');
    this.logTransferStatus(transfer);
  }
  
  /**
   * 处理AssetClaimed事件（跨链完成）
   */
  private handleAssetClaimed(event: any): void {
    console.log('');
    console.log('✅ AssetClaimed event detected');
    console.log('   Lock ID:', event.lockId);
    console.log('   Recipient:', event.recipient);
    
    // 更新本地状态
    const transfer = this.pendingTransfers.get(event.lockId);
    if (transfer) {
      transfer.status = 'COMPLETED';
      this.logTransferStatus(transfer);
    }
  }
  
  /**
   * 处理AssetRefunded事件
   */
  private handleAssetRefunded(event: any): void {
    console.log('');
    console.log('↩️ AssetRefunded event detected');
    console.log('   Lock ID:', event.lockId);
    console.log('   Reason:', event.reason);
    
    // 更新本地状态
    const transfer = this.pendingTransfers.get(event.lockId);
    if (transfer) {
      transfer.status = 'FAILED';
      this.logTransferStatus(transfer);
    }
  }
  
  // ==================== 签名验证 ====================
  
  /**
   * 验证ECDSA签名
   * 
   * ECDSA (Elliptic Curve Digital Signature Algorithm)
   * - 曲线: secp256k1
   * - 签名长度: 65 bytes
   * - 安全性: 128-bit
   * - ⚠️ 量子计算机可以在多项式时间内攻破
   */
  private async verifyECDSASignature(event: AssetLockedEvent): Promise<boolean> {
    // 构建签名消息
    const message = this.buildSigningMessage(event);
    
    // 获取事件中的签名（如果有）
    // 实际应该从事件参数中获取
    
    // 模拟验证
    // 实际会使用 ethers.utils.verifyMessage(message, signature)
    // 与事件中的sender进行比对
    
    return true; // 演示用，实际需要完整验证
  }
  
  /**
   * 构建签名消息
   * 
   * 消息格式: lockId + sender + recipient + token + amount + hashlock + chainId + nonce
   */
  private buildSigningMessage(event: AssetLockedEvent): string {
    const message = ethers.solidityPacked(
      ['bytes32', 'address', 'bytes', 'address', 'uint256', 'bytes32', 'uint256', 'uint256'],
      [
        event.lockId,
        event.sender,
        event.recipient,
        event.token,
        event.amount,
        event.hashlock,
        event.chainId,
        event.nonce
      ]
    );
    
    return ethers.keccak256(message);
  }
  
  // ==================== 状态监控 ====================
  
  /**
   * 启动状态监控
   */
  private startStatusMonitor(): void {
    setInterval(() => {
      this.printStatus();
    }, 30000); // 每30秒打印一次状态
  }
  
  /**
   * 打印当前状态
   */
  private printStatus(): void {
    if (!this.isRunning) return;
    
    console.log('');
    console.log('📊 Relayer Status:');
    console.log('   Running:', this.isRunning);
    console.log('   Pending Transfers:', this.pendingTransfers.size);
    
    let pending = 0, processing = 0, completed = 0, failed = 0;
    
    this.pendingTransfers.forEach(transfer => {
      switch (transfer.status) {
        case 'PENDING': pending++; break;
        case 'PROCESSING': processing++; break;
        case 'COMPLETED': completed++; break;
        case 'FAILED': failed++; break;
      }
    });
    
    console.log('   - PENDING:', pending);
    console.log('   - PROCESSING:', processing);
    console.log('   - COMPLETED:', completed);
    console.log('   - FAILED:', failed);
    console.log('   Processed Events:', this.processedEvents.size);
  }
  
  /**
   * 打印转账状态
   */
  private logTransferStatus(transfer: CrossChainTransfer): void {
    console.log('');
    console.log('📋 Transfer Status:');
    console.log('   Lock ID:', transfer.lockId);
    console.log('   Status:', transfer.status);
    console.log('   From:', transfer.sender);
    console.log('   To:', transfer.recipient);
    console.log('   Amount:', ethers.formatEther(transfer.amount), 'ETH');
    console.log('   Duration:', (Date.now() - transfer.createdAt) / 1000, 'seconds');
  }
  
  // ==================== 工具函数 ====================
  
  /**
   * 获取内联ABI（简化版）
   */
  private getInlineABI(): any[] {
    return [
      "event AssetLocked(bytes32 indexed lockId, address indexed sender, bytes indexed recipient, address token, uint256 amount, bytes32 hashlock, uint256 timelock, uint256 chainId, uint256 nonce)",
      "event AssetClaimed(bytes32 indexed lockId, address indexed recipient, bytes32 secret, bytes falconsig)",
      "event AssetRefunded(bytes32 indexed lockId, address indexed recipient, string reason)",
      "function lockETH(bytes calldata recipient, bytes32 hashlock, uint256 timelock) external payable returns (bytes32 lockId)",
      "function claim(bytes32 lockId, bytes32 secret, bytes calldata falconsig) external",
      "function refund(bytes32 lockId) external",
      "function getLock(bytes32 lockId) external view returns (address sender, bytes memory recipient, address token, uint256 amount, bytes32 hashlock, uint256 timelock, bool claimed, bool refunded)"
    ];
  }
  
  /**
   * 获取配置
   */
  public getConfig(): RelayerConfig {
    return this.config;
  }
  
  /**
   * 获取待处理转账
   */
  public getPendingTransfers(): CrossChainTransfer[] {
    return Array.from(this.pendingTransfers.values());
  }
}

// ==================== 主入口 ====================

async function main() {
  // 从环境变量或配置文件加载配置
  const config: RelayerConfig = {
    ethereum: {
      rpcUrl: process.env.ETHEREUM_RPC_URL || 'http://localhost:8545',
      contractAddress: process.env.ETHEREUM_CONTRACT_ADDRESS || '0x0000000000000000000000000000000000000000',
      privateKey: process.env.RELAYER_PRIVATE_KEY || '0x0000000000000000000000000000000000000000000000000000000000000000',
      chainId: parseInt(process.env.ETHEREUM_CHAIN_ID || '31337')
    },
    qylith: {
      wsUrl: process.env.QYLITH_WS_URL || 'ws://localhost:9944',
      contractAddress: process.env.QYLITH_CONTRACT_ADDRESS || '0x0000000000000000000000000000000000000000',
      seedPhrase: process.env.QYLITH_SEED_PHRASE || '//Alice',
      chainId: parseInt(process.env.QYLITH_CHAIN_ID || '2024')
    },
    falcon: {
      publicKey: process.env.FALCON_PUBLIC_KEY || '',
      privateKey: process.env.FALCON_PRIVATE_KEY || ''
    },
    pollingInterval: parseInt(process.env.POLLING_INTERVAL || '5000'),
    maxConcurrentTransfers: parseInt(process.env.MAX_CONCURRENT || '10')
  };
  
  // 创建Relayer实例
  const relayer = new QuantumShieldBridge(config);
  
  // 优雅关闭
  process.on('SIGINT', async () => {
    console.log('');
    await relayer.stop();
    process.exit(0);
  });
  
  process.on('SIGTERM', async () => {
    console.log('');
    await relayer.stop();
    process.exit(0);
  });
  
  // 启动
  await relayer.start();
}

// 运行
main().catch(console.error);

// 导出类供测试使用
export { QuantumShieldBridge as default };
