/**
 * EthereumListener - Ethereum链事件监听器
 * 
 * 功能：
 * 1. 连接到Ethereum节点
 * 2. 监听QuantumBridgeLock合约事件
 * 3. 解析事件数据
 * 4. 过滤和处理事件
 * 
 * @author QuantumShield Team
 */

import { ethers, Contract, Provider, Log } from 'ethers';
import { EventEmitter } from 'events';

// 类型定义
export interface AssetLockedEvent {
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
  blockTimestamp: number;
}

export interface AssetClaimedEvent {
  lockId: string;
  recipient: string;
  secret: string;
  falconsig: string;
  transactionHash: string;
  blockNumber: number;
}

export interface AssetRefundedEvent {
  lockId: string;
  recipient: string;
  reason: string;
  transactionHash: string;
  blockNumber: number;
}

// 事件过滤器接口
interface ContractFilters {
  AssetLocked: (sender?: string, recipient?: string) => ethers.DeferredLogFilter<any>;
  AssetClaimed: (lockId?: string, recipient?: string) => ethers.DeferredLogFilter<any>;
  AssetRefunded: (lockId?: string, recipient?: string) => ethers.DeferredLogFilter<any>;
}

/**
 * Ethereum事件监听器类
 * 
 * 使用 ethers 的 filter API 监听合约事件
 * 支持实时事件和历史事件回溯
 */
export class EthereumListener extends EventEmitter {
  // 区块链提供商
  private provider: Provider;
  
  // 合约实例
  private contract: Contract;
  
  // 链ID
  private chainId: number;
  
  // 区块范围
  private fromBlock: number;
  private toBlock: 'latest' | number = 'latest';
  
  // 监听状态
  private isListening: boolean = false;
  
  // 事件过滤器
  private filters: ContractFilters;
  
  // 区块处理标记（防止重复）
  private processedBlocks: Set<number> = new Set();
  
  // ==================== 构造函数 ====================
  
  constructor(
    provider: Provider,
    contract: Contract,
    chainId: number,
    fromBlock: number = -1  // -1表示从创世块开始
  ) {
    super();
    
    this.provider = provider;
    this.contract = contract;
    this.chainId = chainId;
    this.fromBlock = fromBlock;
    
    // 初始化过滤器
    this.filters = {
      AssetLocked: (sender?: string, recipient?: string) => {
        const filter = this.contract.filters.AssetLocked(sender, recipient);
        filter.fromBlock = this.fromBlock;
        filter.toBlock = this.toBlock;
        return filter;
      },
      AssetClaimed: (lockId?: string, recipient?: string) => {
        const filter = this.contract.filters.AssetClaimed(lockId, recipient);
        filter.fromBlock = this.fromBlock;
        filter.toBlock = this.toBlock;
        return filter;
      },
      AssetRefunded: (lockId?: string, recipient?: string) => {
        const filter = this.contract.filters.AssetRefunded(lockId, recipient);
        filter.fromBlock = this.fromBlock;
        filter.toBlock = this.toBlock;
        return filter;
      }
    };
  }
  
  // ==================== 启动/停止 ====================
  
  /**
   * 启动监听
   * 
   * 监听流程：
   * 1. 获取当前区块高度
   * 2. 设置事件过滤器
   * 3. 绑定事件监听器
   * 4. 开始轮询新区块
   */
  async start(): Promise<void> {
    console.log('📡 EthereumListener: Starting...');
    
    // 获取起始区块
    const currentBlock = await this.provider.getBlockNumber();
    
    if (this.fromBlock === -1) {
      // 如果没有指定起始块，从当前块开始
      this.fromBlock = currentBlock - 100; // 回溯100个块
    }
    
    console.log(`   Starting from block: ${this.fromBlock}`);
    console.log(`   Current block: ${currentBlock}`);
    console.log(`   Chain ID: ${this.chainId}`);
    
    this.isListening = true;
    
    // 监听AssetLocked事件
    await this.listenToAssetLocked();
    
    // 监听AssetClaimed事件
    await this.listenToAssetClaimed();
    
    // 监听AssetRefunded事件
    await this.listenToAssetRefunded();
    
    // 启动新区块监控
    this.startBlockMonitoring();
    
    console.log('✅ EthereumListener: Started');
  }
  
  /**
   * 停止监听
   */
  async stop(): Promise<void> {
    console.log('📡 EthereumListener: Stopping...');
    this.isListening = false;
    console.log('✅ EthereumListener: Stopped');
  }
  
  // ==================== 事件监听 ====================
  
  /**
   * 监听AssetLocked事件
   * 
   * AssetLocked是跨链转账的起点
   * 当用户在Ethereum锁定资产时触发
   */
  private async listenToAssetLocked(): Promise<void> {
    console.log('   📋 Listening for AssetLocked events...');
    
    try {
      // 获取历史事件
      const historicalEvents = await this.contract.queryFilter(
        'AssetLocked',
        this.fromBlock,
        'latest'
      );
      
      console.log(`   Found ${historicalEvents.length} historical AssetLocked events`);
      
      // 处理历史事件
      for (const event of historicalEvents) {
        this.processAssetLockedEvent(event);
      }
      
      // 订阅新事件
      this.contract.on('AssetLocked', (lockId, sender, recipient, token, amount, hashlock, timelock, chainId, nonce, event) => {
        this.handleAssetLockedEvent(lockId, sender, recipient, token, amount, hashlock, timelock, chainId, nonce, event);
      });
      
    } catch (error) {
      console.error('   ❌ Error listening to AssetLocked:', error);
    }
  }
  
  /**
   * 监听AssetClaimed事件
   */
  private async listenToAssetClaimed(): Promise<void> {
    console.log('   📋 Listening for AssetClaimed events...');
    
    this.contract.on('AssetClaimed', (lockId, recipient, secret, falconsig, event) => {
      this.handleAssetClaimedEvent(lockId, recipient, secret, falconsig, event);
    });
  }
  
  /**
   * 监听AssetRefunded事件
   */
  private async listenToAssetRefunded(): Promise<void> {
    console.log('   📋 Listening for AssetRefunded events...');
    
    this.contract.on('AssetRefunded', (lockId, recipient, reason, event) => {
      this.handleAssetRefundedEvent(lockId, recipient, reason, event);
    });
  }
  
  // ==================== 事件处理 ====================
  
  /**
   * 处理AssetLocked事件（历史）
   */
  private processAssetLockedEvent(event: any): void {
    try {
      const parsed = this.parseAssetLockedEvent(event);
      console.log('');
      console.log('📥 Historical AssetLocked Event:');
      console.log(`   Lock ID: ${parsed.lockId}`);
      console.log(`   Sender: ${parsed.sender}`);
      console.log(`   Amount: ${ethers.formatEther(parsed.amount)} ETH`);
      console.log(`   Block: ${parsed.blockNumber}`);
      
      // 触发事件（但不标记为新事件）
      this.emit('AssetLocked', parsed);
    } catch (error) {
      console.error('   ❌ Error parsing AssetLocked event:', error);
    }
  }
  
  /**
   * 处理AssetLocked事件（实时）
   */
  private handleAssetLockedEvent(
    lockId: string,
    sender: string,
    recipient: string,
    token: string,
    amount: bigint,
    hashlock: string,
    timelock: bigint,
    chainId: bigint,
    nonce: bigint,
    event: any
  ): void {
    // 防止重复处理
    if (this.processedBlocks.has(event.blockNumber)) {
      return;
    }
    this.processedBlocks.add(event.blockNumber);
    
    const parsed: AssetLockedEvent = {
      lockId,
      sender,
      recipient,
      token,
      amount: amount.toString(),
      hashlock,
      timelock: Number(timelock),
      chainId: Number(chainId),
      nonce: Number(nonce),
      transactionHash: event.log.transactionHash,
      blockNumber: event.log.blockNumber,
      blockTimestamp: 0 // 可以在需要时获取
    };
    
    console.log('');
    console.log('📥 NEW AssetLocked Event Received!');
    console.log('   Lock ID:', lockId);
    console.log('   Sender:', sender);
    console.log('   Recipient:', recipient);
    console.log('   Token:', token === '0x0000000000000000000000000000000000000000' ? 'ETH' : token);
    console.log('   Amount:', ethers.formatEther(amount), 'ETH');
    console.log('   Hashlock:', hashlock.substring(0, 18) + '...');
    console.log('   Timelock:', new Date(Number(timelock) * 1000).toISOString());
    console.log('   Chain ID:', chainId);
    console.log('   Nonce:', nonce);
    console.log('   TX Hash:', event.log.transactionHash);
    console.log('   Block:', event.log.blockNumber);
    
    // 发出事件供Relayer处理
    this.emit('AssetLocked', parsed);
  }
  
  /**
   * 处理AssetClaimed事件
   */
  private handleAssetClaimedEvent(
    lockId: string,
    recipient: string,
    secret: string,
    falconsig: string,
    event: any
  ): void {
    const parsed: AssetClaimedEvent = {
      lockId,
      recipient,
      secret,
      falconsig,
      transactionHash: event.log.transactionHash,
      blockNumber: event.log.blockNumber
    };
    
    console.log('');
    console.log('✅ AssetClaimed Event Received!');
    console.log('   Lock ID:', lockId);
    console.log('   Recipient:', recipient);
    console.log('   FALCON Signature Length:', falconsig.length, 'bytes');
    console.log('   TX Hash:', event.log.transactionHash);
    
    this.emit('AssetClaimed', parsed);
  }
  
  /**
   * 处理AssetRefunded事件
   */
  private handleAssetRefundedEvent(
    lockId: string,
    recipient: string,
    reason: string,
    event: any
  ): void {
    const parsed: AssetRefundedEvent = {
      lockId,
      recipient,
      reason,
      transactionHash: event.log.transactionHash,
      blockNumber: event.log.blockNumber
    };
    
    console.log('');
    console.log('↩️ AssetRefunded Event Received!');
    console.log('   Lock ID:', lockId);
    console.log('   Recipient:', recipient);
    console.log('   Reason:', reason);
    console.log('   TX Hash:', event.log.transactionHash);
    
    this.emit('AssetRefunded', parsed);
  }
  
  // ==================== 事件解析 ====================
  
  /**
   * 解析AssetLocked事件
   */
  private parseAssetLockedEvent(event: Log): AssetLockedEvent {
    const iface = this.contract.interface;
    const parsed = iface.parseLog({
      topics: Array.from(event.topics),
      data: event.data
    });
    
    if (!parsed) {
      throw new Error('Failed to parse event');
    }
    
    const args = parsed.args;
    
    return {
      lockId: args.lockId,
      sender: args.sender,
      recipient: args.recipient,
      token: args.token,
      amount: args.amount.toString(),
      hashlock: args.hashlock,
      timelock: Number(args.timelock),
      chainId: Number(args.chainId),
      nonce: Number(args.nonce),
      transactionHash: event.transactionHash,
      blockNumber: event.blockNumber,
      blockTimestamp: 0
    };
  }
  
  // ==================== 区块监控 ====================
  
  /**
   * 启动新区块监控
   * 
   * 持续检查新区块，获取最新事件
   */
  private startBlockMonitoring(): void {
    let lastBlock = 0;
    
    const checkNewBlocks = async () => {
      if (!this.isListening) return;
      
      try {
        const currentBlock = await this.provider.getBlockNumber();
        
        if (currentBlock > lastBlock) {
          // 有新区块
          if (currentBlock - lastBlock > 1) {
            console.log('');
            console.log('📦 Detected new blocks:', currentBlock - lastBlock);
          }
          
          lastBlock = currentBlock;
          
          // 可以在这里进行额外的处理
          // 例如：检查待处理的转账是否超时
        }
      } catch (error) {
        console.error('   ⚠️ Error checking blocks:', error);
      }
      
      // 继续监控
      setTimeout(checkNewBlocks, 3000);
    };
    
    // 启动监控
    setTimeout(checkNewBlocks, 1000);
  }
  
  // ==================== 查询方法 ====================
  
  /**
   * 查询特定锁定信息
   */
  async getLockInfo(lockId: string): Promise<any> {
    return await this.contract.getLock(lockId);
  }
  
  /**
   * 查询地址的nonce
   */
  async getNonce(address: string): Promise<number> {
    return await this.contract.nonces(address);
  }
  
  /**
   * 获取合约余额
   */
  async getContractBalance(): Promise<string> {
    const balance = await this.provider.getBalance(this.contract.target);
    return ethers.formatEther(balance);
  }
  
  /**
   * 获取事件历史
   */
  async getEventHistory(
    eventName: 'AssetLocked' | 'AssetClaimed' | 'AssetRefunded',
    fromBlock: number,
    toBlock: number = -1
  ): Promise<any[]> {
    const filter = this.contract.filters[eventName]();
    filter.fromBlock = fromBlock;
    filter.toBlock = toBlock === -1 ? 'latest' : toBlock;
    
    const events = await this.provider.getLogs(filter);
    return events.map(event => this.parseLog(event));
  }
  
  /**
   * 解析日志
   */
  private parseLog(log: Log): any {
    const iface = this.contract.interface;
    const parsed = iface.parseLog({
      topics: Array.from(log.topics),
      data: log.data
    });
    
    return {
      ...parsed?.args,
      transactionHash: log.transactionHash,
      blockNumber: log.blockNumber,
      logIndex: log.index
    };
  }
  
  // ==================== 状态 ====================
  
  /**
   * 获取监听状态
   */
  public isActive(): boolean {
    return this.isListening;
  }
  
  /**
   * 获取已处理的区块数
   */
  public getProcessedBlockCount(): number {
    return this.processedBlocks.size;
  }
}
