/**
 * QuantumShield Bridge - Main App Component
 * 
 * 抗量子跨链桥DApp主界面
 * 支持Ethereum和Qylith双链连接
 * 
 * @author QuantumShield Team
 * @notice For HTX Genesis Hackathon Demo
 */

import React, { useState, useEffect } from 'react';
import { BridgeForm } from './BridgeForm';
import { SignatureCompare } from './SignatureCompare';
import { CrossChainStatus } from './CrossChainStatus';

// ==================== 类型定义 ====================

/**
 * 钱包连接状态
 */
interface WalletState {
  ethereum: {
    connected: boolean;
    address: string | null;
    chainId: number | null;
    balance: string | null;
  };
  qylith: {
    connected: boolean;
    address: string | null;
    balance: string | null;
  };
}

/**
 * 跨链转账记录
 */
interface TransferRecord {
  id: string;
  direction: 'E2Q' | 'Q2E'; // Ethereum to Qylith, or Qylith to Ethereum
  amount: string;
  status: 'PENDING' | 'PROCESSING' | 'COMPLETED' | 'FAILED';
  timestamp: number;
  txHash?: string;
}

/**
 * 签名对比数据
 */
interface SignatureCompareData {
  ecdsa: {
    algorithm: string;
    signatureSize: number;
    publicKeySize: number;
    securityLevel: string;
    quantumResistant: boolean;
  };
  falcon: {
    algorithm: string;
    signatureSize: number;
    publicKeySize: number;
    securityLevel: string;
    quantumResistant: boolean;
  };
}

// ==================== 初始状态 ====================

const initialWalletState: WalletState = {
  ethereum: {
    connected: false,
    address: null,
    chainId: null,
    balance: null
  },
  qylith: {
    connected: false,
    address: null,
    balance: null
  }
};

// ==================== 常量 ====================

const SIGNATURE_DATA: SignatureCompareData = {
  ecdsa: {
    algorithm: 'ECDSA (secp256k1)',
    signatureSize: 65,
    publicKeySize: 33,
    securityLevel: '128-bit',
    quantumResistant: false
  },
  falcon: {
    algorithm: 'FALCON-1024',
    signatureSize: 666,
    publicKeySize: 897,
    securityLevel: '256-bit',
    quantumResistant: true
  }
};

// ==================== 主组件 ====================

export default function App() {
  // 状态
  const [wallet, setWallet] = useState<WalletState>(initialWalletState);
  const [transfers, setTransfers] = useState<TransferRecord[]>([]);
  const [activeTab, setActiveTab] = useState<'bridge' | 'compare'>('bridge');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // ==================== 效果 ====================
  
  // 初始化检查
  useEffect(() => {
    console.log('🚀 QuantumShield Bridge DApp initialized');
    console.log('   Ethereum side: ECDSA signatures');
    console.log('   Qylith side: FALCON-1024 (Quantum-resistant)');
  }, []);
  
  // ==================== 钱包连接 ====================
  
  /**
   * 连接Ethereum钱包（MetaMask）
   */
  const connectEthereum = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      // 检查MetaMask
      if (typeof window.ethereum === 'undefined') {
        throw new Error('Please install MetaMask extension');
      }
      
      // 请求连接
      const accounts = await window.ethereum.request({
        method: 'eth_requestAccounts'
      });
      
      if (accounts.length > 0) {
        // 获取链ID
        const chainId = await window.ethereum.request({
          method: 'eth_chainId'
        });
        
        // 获取余额
        const balance = await window.ethereum.request({
          method: 'eth_getBalance',
          params: [accounts[0], 'latest']
        });
        
        // 更新状态
        setWallet(prev => ({
          ...prev,
          ethereum: {
            connected: true,
            address: accounts[0],
            chainId: parseInt(chainId, 16),
            balance: (parseInt(balance, 16) / 1e18).toFixed(4)
          }
        }));
        
        console.log('✅ Ethereum wallet connected:', accounts[0]);
      }
    } catch (err: any) {
      setError(err.message);
      console.error('❌ Ethereum connection failed:', err);
    } finally {
      setIsLoading(false);
    }
  };
  
  /**
   * 连接Qylith钱包（Polkadot.js）
   */
  const connectQylith = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      // 检查Polkadot扩展
      const { web3Enable, web3Accounts } = await import('@polkadot/extension-dapp');
      
      const extensions = await web3Enable('QuantumShield Bridge');
      if (extensions.length === 0) {
        throw new Error('Please install Polkadot.js extension');
      }
      
      const accounts = await web3Accounts();
      if (accounts.length > 0) {
        // 获取第一个账户
        const account = accounts[0];
        
        setWallet(prev => ({
          ...prev,
          qylith: {
            connected: true,
            address: account.address,
            balance: '0' // 需要查询实际余额
          }
        }));
        
        console.log('✅ Qylith wallet connected:', account.address);
      }
    } catch (err: any) {
      setError(err.message);
      console.error('❌ Qylith connection failed:', err);
    } finally {
      setIsLoading(false);
    }
  };
  
  /**
   * 断开Ethereum连接
   */
  const disconnectEthereum = () => {
    setWallet(prev => ({
      ...prev,
      ethereum: initialWalletState.ethereum
    }));
  };
  
  /**
   * 断开Qylith连接
   */
  const disconnectQylith = () => {
    setWallet(prev => ({
      ...prev,
      qylith: initialWalletState.qylith
    }));
  };
  
  // ==================== 跨链转账 ====================
  
  /**
   * 处理跨链转账
   */
  const handleTransfer = async (direction: 'E2Q' | 'Q2E', amount: string) => {
    setIsLoading(true);
    setError(null);
    
    try {
      // 创建转账记录
      const record: TransferRecord = {
        id: `tx_${Date.now()}`,
        direction,
        amount,
        status: 'PENDING',
        timestamp: Date.now()
      };
      
      setTransfers(prev => [record, ...prev]);
      
      // 模拟处理过程
      // 实际会调用合约和relayer
      
      // 1. 锁定资产
      record.status = 'PROCESSING';
      setTransfers(prev => prev.map(t => t.id === record.id ? record : t));
      
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // 2. 签名验证
      console.log('🔐 ECDSA signature verified');
      await new Promise(resolve => setTimeout(resolve, 500));
      
      console.log('🔐 FALCON signature generated (quantum-resistant)');
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // 3. 跨链提交
      console.log('📤 Submitting to Qylith...');
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      // 4. 完成
      record.status = 'COMPLETED';
      record.txHash = '0x' + Math.random().toString(16).slice(2).padEnd(64, '0');
      setTransfers(prev => prev.map(t => t.id === record.id ? record : t));
      
      console.log('✅ Cross-chain transfer completed!');
      
    } catch (err: any) {
      setError(err.message);
      // 标记为失败
      setTransfers(prev => prev.map(t => 
        t.id === transfers[0]?.id ? { ...t, status: 'FAILED' } : t
      ));
    } finally {
      setIsLoading(false);
    }
  };
  
  // ==================== 渲染 ====================
  
  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900">
      {/* 头部 */}
      <header className="border-b border-white/10 bg-black/20 backdrop-blur-lg">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              {/* Logo */}
              <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-cyan-400 to-blue-500 flex items-center justify-center">
                <span className="text-white font-bold text-lg">Q</span>
              </div>
              <div>
                <h1 className="text-xl font-bold text-white">QuantumShield Bridge</h1>
                <p className="text-xs text-cyan-400">抗量子跨链桥</p>
              </div>
            </div>
            
            {/* 钱包状态 */}
            <div className="flex items-center gap-4">
              {/* Ethereum钱包 */}
              <WalletBadge
                chain="Ethereum"
                connected={wallet.ethereum.connected}
                address={wallet.ethereum.address}
                balance={wallet.ethereum.balance}
                onConnect={connectEthereum}
                onDisconnect={disconnectEthereum}
              />
              
              {/* Qylith钱包 */}
              <WalletBadge
                chain="Qylith"
                connected={wallet.qylith.connected}
                address={wallet.qylith.address}
                balance={wallet.qylith.balance}
                onConnect={connectQylith}
                onDisconnect={disconnectQylith}
              />
            </div>
          </div>
        </div>
      </header>
      
      {/* 主内容 */}
      <main className="container mx-auto px-4 py-8">
        {/* Tab切换 */}
        <div className="flex gap-4 mb-8">
          <TabButton
            active={activeTab === 'bridge'}
            onClick={() => setActiveTab('bridge')}
          >
            🌉 跨链转账
          </TabButton>
          <TabButton
            active={activeTab === 'compare'}
            onClick={() => setActiveTab('compare')}
          >
            🔐 签名对比
          </TabButton>
        </div>
        
        {/* 错误提示 */}
        {error && (
          <div className="mb-6 p-4 bg-red-500/20 border border-red-500/50 rounded-lg text-red-300">
            ❌ {error}
          </div>
        )}
        
        {/* 内容区域 */}
        {activeTab === 'bridge' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
            {/* 左侧：表单 */}
            <div className="bg-white/5 backdrop-blur-lg rounded-2xl p-6 border border-white/10">
              <BridgeForm
                wallet={wallet}
                onTransfer={handleTransfer}
                isLoading={isLoading}
              />
            </div>
            
            {/* 右侧：状态 */}
            <div className="space-y-6">
              <CrossChainStatus
                transfers={transfers}
                signatureData={SIGNATURE_DATA}
              />
            </div>
          </div>
        )}
        
        {activeTab === 'compare' && (
          <SignatureCompare data={SIGNATURE_DATA} />
        )}
      </main>
      
      {/* 加载指示器 */}
      {isLoading && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-8 text-center">
            <div className="animate-spin w-12 h-12 border-4 border-cyan-400 border-t-transparent rounded-full mx-auto mb-4"></div>
            <p className="text-white">Processing cross-chain transfer...</p>
          </div>
        </div>
      )}
      
      {/* 页脚 */}
      <footer className="fixed bottom-0 left-0 right-0 bg-black/20 backdrop-blur-lg border-t border-white/10 py-3">
        <div className="container mx-auto px-4 text-center text-sm text-gray-400">
          QuantumShield Bridge - HTX Genesis Hackathon Demo | 
          <span className="text-cyan-400"> Ethereum ↔ Qylith</span> | 
          Powered by <span className="text-purple-400">FALCON-1024</span>
        </div>
      </footer>
    </div>
  );
}

// ==================== 子组件 ====================

/**
 * 钱包连接状态徽章
 */
function WalletBadge({
  chain,
  connected,
  address,
  balance,
  onConnect,
  onDisconnect
}: {
  chain: 'Ethereum' | 'Qylith';
  connected: boolean;
  address: string | null;
  balance: string | null;
  onConnect: () => void;
  onDisconnect: () => void;
}) {
  const colors = chain === 'Ethereum'
    ? 'from-orange-500 to-yellow-500'
    : 'from-pink-500 to-purple-500';
  
  const icon = chain === 'Ethereum' ? 'Ξ' : 'Q';
  
  return (
    <div className={`flex items-center gap-2 px-3 py-2 rounded-lg bg-gradient-to-r ${colors} ${connected ? 'opacity-100' : 'opacity-60'}`}>
      <span className="text-white font-bold">{icon}</span>
      
      {connected && address ? (
        <>
          <div className="text-left">
            <div className="text-xs text-white/80">{chain}</div>
            <div className="text-sm text-white font-mono">
              {address.slice(0, 6)}...{address.slice(-4)}
            </div>
          </div>
          {balance && (
            <div className="text-sm text-white font-medium">
              {balance} ETH
            </div>
          )}
          <button
            onClick={onDisconnect}
            className="ml-2 text-xs text-white/80 hover:text-white"
          >
            ✕
          </button>
        </>
      ) : (
        <button
          onClick={onConnect}
          className="text-sm text-white hover:text-white/80 transition"
        >
          Connect
        </button>
      )}
    </div>
  );
}

/**
 * Tab按钮
 */
function TabButton({
  active,
  onClick,
  children
}: {
  active: boolean;
  onClick: () => void;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={`px-6 py-3 rounded-lg font-medium transition ${
        active
          ? 'bg-gradient-to-r from-cyan-500 to-blue-500 text-white shadow-lg'
          : 'bg-white/10 text-gray-300 hover:bg-white/20'
      }`}
    >
      {children}
    </button>
  );
}

// 类型声明
declare global {
  interface Window {
    ethereum?: any;
  }
}
