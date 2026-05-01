/**
 * BridgeForm - 跨链转账表单组件
 * 
 * 提供Ethereum ↔ Qylith的跨链转账界面
 * 支持ETH和ERC20代币
 * 
 * @author QuantumShield Team
 */

import React, { useState } from 'react';

// ==================== 类型定义 ====================

interface WalletInfo {
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

interface BridgeFormProps {
  wallet: WalletInfo;
  onTransfer: (direction: 'E2Q' | 'Q2E', amount: string) => Promise<void>;
  isLoading: boolean;
}

// ==================== 常量 ====================

const TOKENS = [
  { symbol: 'ETH', name: 'Ethereum', icon: 'Ξ', color: 'from-orange-500 to-yellow-500' },
  { symbol: 'wETH', name: 'Wrapped Ether', icon: 'Ξ', color: 'from-orange-400 to-yellow-400' },
  { symbol: 'USDC', name: 'USD Coin', icon: '$', color: 'from-blue-500 to-cyan-500' },
  { symbol: 'USDT', name: 'Tether', icon: '$', color: 'from-green-500 to-teal-500' }
];

// ==================== 组件 ====================

export function BridgeForm({ wallet, onTransfer, isLoading }: BridgeFormProps) {
  // 状态
  const [direction, setDirection] = useState<'E2Q' | 'Q2E'>('E2Q');
  const [amount, setAmount] = useState('');
  const [selectedToken, setSelectedToken] = useState('ETH');
  const [recipient, setRecipient] = useState('');
  const [showTokenSelector, setShowTokenSelector] = useState(false);
  
  // 计算最大金额
  const maxAmount = direction === 'E2Q'
    ? wallet.ethereum.balance
    : wallet.qylith.balance;
  
  // 设置最大值
  const handleSetMax = () => {
    if (maxAmount) {
      setAmount(maxAmount);
    }
  };
  
  // 处理提交
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!amount || parseFloat(amount) <= 0) {
      return;
    }
    
    // 验证钱包连接
    if (direction === 'E2Q' && !wallet.ethereum.connected) {
      alert('Please connect your Ethereum wallet first');
      return;
    }
    
    if (direction === 'Q2E' && !wallet.qylith.connected) {
      alert('Please connect your Qylith wallet first');
      return;
    }
    
    await onTransfer(direction, amount);
  };
  
  // 获取当前选中的代币
  const currentToken = TOKENS.find(t => t.symbol === selectedToken) || TOKENS[0];
  
  // ==================== 渲染 ====================
  
  return (
    <div className="space-y-6">
      {/* 标题 */}
      <div className="text-center">
        <h2 className="text-2xl font-bold text-white mb-2">Cross-Chain Transfer</h2>
        <p className="text-gray-400">Transfer assets between Ethereum and Qylith</p>
      </div>
      
      {/* 方向切换 */}
      <div className="flex bg-white/10 rounded-xl p-1">
        <button
          onClick={() => setDirection('E2Q')}
          className={`flex-1 py-3 px-4 rounded-lg font-medium transition ${
            direction === 'E2Q'
              ? 'bg-gradient-to-r from-orange-500 to-yellow-500 text-white'
              : 'text-gray-400 hover:text-white'
          }`}
        >
          <div className="flex items-center justify-center gap-2">
            <span>Ethereum</span>
            <span>→</span>
            <span className="text-cyan-400">Qylith</span>
          </div>
        </button>
        <button
          onClick={() => setDirection('Q2E')}
          className={`flex-1 py-3 px-4 rounded-lg font-medium transition ${
            direction === 'Q2E'
              ? 'bg-gradient-to-r from-purple-500 to-pink-500 text-white'
              : 'text-gray-400 hover:text-white'
          }`}
        >
          <div className="flex items-center justify-center gap-2">
            <span className="text-cyan-400">Qylith</span>
            <span>→</span>
            <span>Ethereum</span>
          </div>
        </button>
      </div>
      
      {/* 表单 */}
      <form onSubmit={handleSubmit} className="space-y-4">
        {/* 金额输入 */}
        <div className="space-y-2">
          <label className="text-sm text-gray-400">Amount</label>
          <div className="relative">
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="0.0"
              step="0.001"
              min="0"
              className="w-full bg-white/5 border border-white/10 rounded-xl py-4 px-4 pr-24 text-2xl text-white placeholder-gray-500 focus:outline-none focus:border-cyan-500/50"
            />
            <button
              type="button"
              onClick={handleSetMax}
              className="absolute right-4 top-1/2 -translate-y-1/2 px-3 py-1 text-sm text-cyan-400 hover:text-cyan-300"
            >
              MAX
            </button>
          </div>
          
          {/* 余额显示 */}
          <div className="flex justify-between text-sm">
            <span className="text-gray-500">
              {direction === 'E2Q' ? 'Ethereum Balance' : 'Qylith Balance'}:
            </span>
            <span className="text-gray-300">
              {maxAmount || '0'} {selectedToken}
            </span>
          </div>
        </div>
        
        {/* 代币选择 */}
        <div className="space-y-2">
          <label className="text-sm text-gray-400">Token</label>
          <div className="relative">
            <button
              type="button"
              onClick={() => setShowTokenSelector(!showTokenSelector)}
              className="w-full flex items-center gap-3 bg-white/5 border border-white/10 rounded-xl py-3 px-4 hover:bg-white/10 transition"
            >
              <div className={`w-8 h-8 rounded-full bg-gradient-to-br ${currentToken.color} flex items-center justify-center`}>
                <span className="text-white font-bold">{currentToken.icon}</span>
              </div>
              <div className="text-left">
                <div className="text-white font-medium">{currentToken.symbol}</div>
                <div className="text-xs text-gray-400">{currentToken.name}</div>
              </div>
              <svg className="w-5 h-5 text-gray-400 ml-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
              </svg>
            </button>
            
            {/* 代币选择器 */}
            {showTokenSelector && (
              <div className="absolute top-full left-0 right-0 mt-2 bg-gray-900 border border-white/10 rounded-xl overflow-hidden z-10">
                {TOKENS.map((token) => (
                  <button
                    key={token.symbol}
                    type="button"
                    onClick={() => {
                      setSelectedToken(token.symbol);
                      setShowTokenSelector(false);
                    }}
                    className="w-full flex items-center gap-3 p-3 hover:bg-white/10 transition"
                  >
                    <div className={`w-8 h-8 rounded-full bg-gradient-to-br ${token.color} flex items-center justify-center`}>
                      <span className="text-white font-bold">{token.icon}</span>
                    </div>
                    <div className="text-left">
                      <div className="text-white font-medium">{token.symbol}</div>
                      <div className="text-xs text-gray-400">{token.name}</div>
                    </div>
                    {selectedToken === token.symbol && (
                      <span className="ml-auto text-cyan-400">✓</span>
                    )}
                  </button>
                ))}
              </div>
            )}
          </div>
        </div>
        
        {/* 目标地址（仅显示） */}
        <div className="space-y-2">
          <label className="text-sm text-gray-400">
            {direction === 'E2Q' ? 'Qylith Recipient' : 'Ethereum Recipient'}
          </label>
          <div className="bg-white/5 border border-white/10 rounded-xl py-3 px-4">
            {direction === 'E2Q' ? (
              wallet.qylith.connected ? (
                <span className="text-gray-300 font-mono">
                  {wallet.qylith.address?.slice(0, 10)}...{wallet.qylith.address?.slice(-8)}
                </span>
              ) : (
                <span className="text-gray-500">Connect Qylith wallet to auto-fill</span>
              )
            ) : (
              wallet.ethereum.connected ? (
                <span className="text-gray-300 font-mono">
                  {wallet.ethereum.address?.slice(0, 10)}...{wallet.ethereum.address?.slice(-8)}
                </span>
              ) : (
                <span className="text-gray-500">Connect Ethereum wallet to auto-fill</span>
              )
            )}
          </div>
        </div>
        
        {/* 提交按钮 */}
        <button
          type="submit"
          disabled={isLoading || !amount || parseFloat(amount) <= 0}
          className={`w-full py-4 rounded-xl font-bold text-lg transition ${
            isLoading || !amount || parseFloat(amount) <= 0
              ? 'bg-gray-600 text-gray-400 cursor-not-allowed'
              : direction === 'E2Q'
                ? 'bg-gradient-to-r from-orange-500 to-yellow-500 text-white hover:shadow-lg hover:shadow-orange-500/25'
                : 'bg-gradient-to-r from-purple-500 to-pink-500 text-white hover:shadow-lg hover:shadow-purple-500/25'
          }`}
        >
          {isLoading ? (
            <span className="flex items-center justify-center gap-2">
              <svg className="animate-spin w-5 h-5" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
              </svg>
              Processing...
            </span>
          ) : (
            `Transfer to ${direction === 'E2Q' ? 'Qylith' : 'Ethereum'}`
          )}
        </button>
        
        {/* 安全提示 */}
        <div className="flex items-start gap-3 p-4 bg-cyan-500/10 rounded-xl border border-cyan-500/20">
          <span className="text-2xl">🔐</span>
          <div className="text-sm">
            <p className="text-cyan-400 font-medium mb-1">Quantum-Resistant Security</p>
            <p className="text-gray-400">
              Your transaction is protected by <span className="text-cyan-400">FALCON-1024</span> signatures, 
              resistant to quantum computer attacks.
            </p>
          </div>
        </div>
      </form>
      
      {/* 流程说明 */}
      <div className="space-y-3">
        <h3 className="text-sm text-gray-400 font-medium">Transfer Process</h3>
        <div className="grid grid-cols-4 gap-2">
          {[
            { step: 1, icon: '🔒', label: 'Lock' },
            { step: 2, icon: '✓', label: 'Verify' },
            { step: 3, icon: '🔐', label: 'FALCON' },
            { step: 4, icon: '🎉', label: 'Mint' }
          ].map((item) => (
            <div key={item.step} className="text-center">
              <div className="w-10 h-10 mx-auto mb-1 rounded-full bg-white/10 flex items-center justify-center text-lg">
                {item.icon}
              </div>
              <div className="text-xs text-gray-400">{item.label}</div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
