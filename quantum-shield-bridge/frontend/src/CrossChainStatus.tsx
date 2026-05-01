/**
 * CrossChainStatus - 跨链状态显示组件
 * 
 * 显示跨链转账的状态和历史
 * 实时展示签名验证过程
 * 
 * @author QuantumShield Team
 */

import React from 'react';

// ==================== 类型定义 ====================

interface TransferRecord {
  id: string;
  direction: 'E2Q' | 'Q2E';
  amount: string;
  status: 'PENDING' | 'PROCESSING' | 'COMPLETED' | 'FAILED';
  timestamp: number;
  txHash?: string;
}

interface SignatureData {
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

interface CrossChainStatusProps {
  transfers: TransferRecord[];
  signatureData: SignatureData;
}

// ==================== 常量 ====================

const STATUS_CONFIG = {
  PENDING: {
    label: 'Pending',
    color: 'text-yellow-400',
    bgColor: 'bg-yellow-400',
    icon: '⏳'
  },
  PROCESSING: {
    label: 'Processing',
    color: 'text-blue-400',
    bgColor: 'bg-blue-400',
    icon: '⚙️'
  },
  COMPLETED: {
    label: 'Completed',
    color: 'text-green-400',
    bgColor: 'bg-green-400',
    icon: '✅'
  },
  FAILED: {
    label: 'Failed',
    color: 'text-red-400',
    bgColor: 'bg-red-400',
    icon: '❌'
  }
};

// ==================== 组件 ====================

export function CrossChainStatus({ transfers, signatureData }: CrossChainStatusProps) {
  return (
    <div className="space-y-6">
      {/* 实时签名状态 */}
      <div className="bg-white/5 backdrop-blur-lg rounded-2xl p-6 border border-white/10">
        <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
          <span>🔐</span>
          <span>Signature Verification Status</span>
        </h3>
        
        {/* 签名流程图 */}
        <SignatureFlow signatureData={signatureData} />
      </div>
      
      {/* 转账历史 */}
      <div className="bg-white/5 backdrop-blur-lg rounded-2xl p-6 border border-white/10">
        <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
          <span>📜</span>
          <span>Transfer History</span>
        </h3>
        
        {transfers.length === 0 ? (
          <EmptyState />
        ) : (
          <div className="space-y-4">
            {transfers.map((transfer) => (
              <TransferCard key={transfer.id} transfer={transfer} />
            ))}
          </div>
        )}
      </div>
      
      {/* 统计信息 */}
      {transfers.length > 0 && (
        <div className="bg-white/5 backdrop-blur-lg rounded-2xl p-6 border border-white/10">
          <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
            <span>📊</span>
            <span>Statistics</span>
          </h3>
          
          <div className="grid grid-cols-2 gap-4">
            <StatCard
              label="Total Transfers"
              value={transfers.length.toString()}
              icon="🌉"
            />
            <StatCard
              label="Completed"
              value={transfers.filter(t => t.status === 'COMPLETED').length.toString()}
              icon="✅"
            />
            <StatCard
              label="Processing"
              value={transfers.filter(t => t.status === 'PROCESSING').length.toString()}
              icon="⚙️"
            />
            <StatCard
              label="Failed"
              value={transfers.filter(t => t.status === 'FAILED').length.toString()}
              icon="❌"
            />
          </div>
        </div>
      )}
    </div>
  );
}

// ==================== 子组件 ====================

/**
 * 签名流程图
 */
function SignatureFlow({ signatureData }: { signatureData: SignatureData }) {
  const steps = [
    {
      id: 1,
      title: 'User Signs',
      description: 'User signs with wallet',
      type: 'user' as const,
      chain: 'Ethereum',
      algorithm: signatureData.ecdsa.algorithm,
      icon: '🔏',
      color: 'from-orange-500 to-yellow-500'
    },
    {
      id: 2,
      title: 'ECDSA Verify',
      description: 'Verify Ethereum signature',
      type: 'ecdsa' as const,
      chain: 'Ethereum',
      algorithm: signatureData.ecdsa.algorithm,
      icon: '✓',
      color: 'from-orange-400 to-yellow-400'
    },
    {
      id: 3,
      title: 'FALCON Sign',
      description: 'Generate quantum-safe proof',
      type: 'falcon' as const,
      chain: 'Qylith',
      algorithm: signatureData.falcon.algorithm,
      icon: '🔐',
      color: 'from-cyan-500 to-blue-500'
    },
    {
      id: 4,
      title: 'Cross-Chain Mint',
      description: 'Mint wrapped assets',
      type: 'complete' as const,
      chain: 'Qylith',
      algorithm: 'Quantum-Resistant',
      icon: '🎉',
      color: 'from-green-500 to-emerald-500'
    }
  ];
  
  return (
    <div className="space-y-4">
      {steps.map((step, index) => (
        <div key={step.id} className="relative">
          {/* 连接线 */}
          {index < steps.length - 1 && (
            <div className="absolute left-5 top-12 w-0.5 h-8 bg-gradient-to-b from-white/20 to-white/5" />
          )}
          
          <div className="flex items-start gap-4">
            {/* 图标 */}
            <div className={`w-10 h-10 rounded-xl bg-gradient-to-r ${step.color} flex items-center justify-center text-lg flex-shrink-0`}>
              {step.icon}
            </div>
            
            {/* 内容 */}
            <div className="flex-1 bg-white/5 rounded-xl p-4">
              <div className="flex items-center justify-between mb-2">
                <h4 className="text-white font-bold">{step.title}</h4>
                <span className={`text-xs px-2 py-1 rounded-full ${
                  step.chain === 'Ethereum' 
                    ? 'bg-orange-500/20 text-orange-400' 
                    : 'bg-cyan-500/20 text-cyan-400'
                }`}>
                  {step.chain}
                </span>
              </div>
              <p className="text-gray-400 text-sm mb-2">{step.description}</p>
              <div className="flex items-center gap-2">
                <span className="text-xs text-gray-500">Algorithm:</span>
                <span className={`text-xs font-mono ${
                  step.type === 'falcon' ? 'text-cyan-400' : 'text-orange-400'
                }`}>
                  {step.algorithm}
                </span>
                {step.type === 'falcon' && (
                  <span className="text-xs text-green-400">🛡️ Quantum-Safe</span>
                )}
              </div>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}

/**
 * 转账卡片
 */
function TransferCard({ transfer }: { transfer: TransferRecord }) {
  const config = STATUS_CONFIG[transfer.status];
  const isE2Q = transfer.direction === 'E2Q';
  
  return (
    <div className="bg-white/5 rounded-xl p-4 border border-white/10">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-3">
          {/* 方向箭头 */}
          <div className={`w-10 h-10 rounded-lg bg-gradient-to-r ${isE2Q ? 'from-orange-500 to-yellow-500' : 'from-purple-500 to-pink-500'} flex items-center justify-center`}>
            <span className="text-white font-bold text-sm">{isE2Q ? 'E→Q' : 'Q→E'}</span>
          </div>
          
          <div>
            <div className="text-white font-medium">{transfer.amount} ETH</div>
            <div className="text-xs text-gray-400">
              {new Date(transfer.timestamp).toLocaleString()}
            </div>
          </div>
        </div>
        
        {/* 状态 */}
        <div className={`flex items-center gap-2 px-3 py-1 rounded-full ${config.color} bg-current/10`}>
          <span>{config.icon}</span>
          <span className={`text-sm font-medium ${config.color}`}>{config.label}</span>
        </div>
      </div>
      
      {/* 进度条 */}
      <div className="h-2 bg-gray-700 rounded-full overflow-hidden">
        <div 
          className={`h-full ${config.bgColor} transition-all duration-500`}
          style={{ 
            width: transfer.status === 'COMPLETED' ? '100%' : 
                   transfer.status === 'PROCESSING' ? '66%' :
                   transfer.status === 'PENDING' ? '33%' : '0%'
          }}
        />
      </div>
      
      {/* TX Hash */}
      {transfer.txHash && (
        <div className="mt-3 flex items-center gap-2">
          <span className="text-xs text-gray-500">TX:</span>
          <span className="text-xs text-cyan-400 font-mono">
            {transfer.txHash.slice(0, 10)}...{transfer.txHash.slice(-8)}
          </span>
        </div>
      )}
    </div>
  );
}

/**
 * 空状态
 */
function EmptyState() {
  return (
    <div className="text-center py-12">
      <div className="text-5xl mb-4">🌉</div>
      <h4 className="text-white font-bold mb-2">No Transfers Yet</h4>
      <p className="text-gray-400 text-sm">
        Start a cross-chain transfer to see it here
      </p>
    </div>
  );
}

/**
 * 统计卡片
 */
function StatCard({ label, value, icon }: { label: string; value: string; icon: string }) {
  return (
    <div className="bg-white/5 rounded-xl p-4 text-center">
      <div className="text-2xl mb-2">{icon}</div>
      <div className="text-2xl font-bold text-white">{value}</div>
      <div className="text-xs text-gray-400">{label}</div>
    </div>
  );
}
