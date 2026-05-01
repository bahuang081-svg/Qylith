/**
 * SignatureCompare - 签名对比可视化组件
 * 
 * 展示ECDSA和FALCON签名的对比
 * 突出"抗量子"的核心差异化
 * 
 * @author QuantumShield Team
 */

import React, { useState } from 'react';

// ==================== 类型定义 ====================

interface SignatureData {
  algorithm: string;
  signatureSize: number;
  publicKeySize: number;
  securityLevel: string;
  quantumResistant: boolean;
}

interface SignatureCompareProps {
  data: {
    ecdsa: SignatureData;
    falcon: SignatureData;
  };
}

// ==================== 常量 ====================

const ALGORITHM_INFO = {
  ecdsa: {
    name: 'ECDSA',
    curve: 'secp256k1',
    description: 'Elliptic Curve Digital Signature Algorithm',
    fullName: 'Elliptic Curve Digital Signature Algorithm',
    invented: '1985',
    usage: 'Bitcoin, Ethereum, most blockchains',
    vulnerability: 'Vulnerable to Shor\'s algorithm on quantum computers',
    howItWorks: [
      'Key generation: Select a point G on the elliptic curve',
      'Signing: Calculate r = x-coordinate of kG, s = k⁻¹(H(m) + dr) mod n',
      'Verification: Check if r and s satisfy the curve equation'
    ]
  },
  falcon: {
    name: 'FALCON-1024',
    curve: 'NTRU Lattice',
    description: 'Fast Fourier Lattice-based Compact Signatures over NTRU',
    fullName: 'Fast Fourier Lattice-based Compact Signature over NTRU',
    invented: '2015',
    usage: 'Post-quantum cryptography, blockchain bridges',
    vulnerability: 'Resistant to both classical and quantum attacks',
    howItWorks: [
      'Key generation: Create NTRU lattice with secret trapdoor (f, F)',
      'Hashing: Compute c = HashToCone(message)',
      'Sampling: Use trapdoor to sample s from the lattice near c',
      'Verification: Check ||s|| ≤ β and c = Hash(message || s)'
    ]
  }
};

// ==================== 组件 ====================

export function SignatureCompare({ data }: SignatureCompareProps) {
  const [activeAlgorithm, setActiveAlgorithm] = useState<'ecdsa' | 'falcon' | 'compare'>('compare');
  const [animated, setAnimated] = useState(false);
  
  // 触发动画
  React.useEffect(() => {
    setAnimated(true);
    const timer = setTimeout(() => setAnimated(false), 500);
    return () => clearTimeout(timer);
  }, [activeAlgorithm]);
  
  // ==================== 渲染 ====================
  
  return (
    <div className="space-y-8">
      {/* 标题 */}
      <div className="text-center">
        <h2 className="text-3xl font-bold text-white mb-2">🔐 Signature Algorithm Comparison</h2>
        <p className="text-gray-400">Understanding the quantum-resistant advantage of FALCON</p>
      </div>
      
      {/* Tab切换 */}
      <div className="flex justify-center gap-4">
        <TabButton
          active={activeAlgorithm === 'ecdsa'}
          onClick={() => setActiveAlgorithm('ecdsa')}
          color="from-orange-500 to-yellow-500"
        >
          ECDSA (Current)
        </TabButton>
        <TabButton
          active={activeAlgorithm === 'falcon'}
          onClick={() => setActiveAlgorithm('falcon')}
          color="from-cyan-500 to-blue-500"
        >
          FALCON-1024 (Quantum-Safe)
        </TabButton>
        <TabButton
          active={activeAlgorithm === 'compare'}
          onClick={() => setActiveAlgorithm('compare')}
          color="from-purple-500 to-pink-500"
        >
          Side by Side
        </TabButton>
      </div>
      
      {/* 算法详情卡片 */}
      {(activeAlgorithm === 'ecdsa' || activeAlgorithm === 'falcon') && (
        <div className={`max-w-4xl mx-auto ${animated ? 'animate-pulse' : ''}`}>
          <AlgorithmCard
            type={activeAlgorithm}
            data={data[activeAlgorithm]}
            info={ALGORITHM_INFO[activeAlgorithm]}
          />
        </div>
      )}
      
      {/* 对比视图 */}
      {activeAlgorithm === 'compare' && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* ECDSA卡片 */}
          <div className="bg-gradient-to-br from-orange-500/20 to-yellow-500/20 rounded-2xl p-6 border border-orange-500/30">
            <div className="flex items-center gap-3 mb-6">
              <div className="w-12 h-12 rounded-xl bg-gradient-to-r from-orange-500 to-yellow-500 flex items-center justify-center text-2xl">
                ⚠️
              </div>
              <div>
                <h3 className="text-xl font-bold text-white">{data.ecdsa.algorithm}</h3>
                <p className="text-sm text-orange-400">Quantum-Vulnerable</p>
              </div>
            </div>
            
            <ComparisonStats data={data.ecdsa} />
            
            <div className="mt-6 p-4 bg-red-500/20 rounded-xl border border-red-500/30">
              <p className="text-red-300 text-sm">
                ⚠️ <strong>Quantum Threat:</strong> Shor's algorithm can solve the discrete logarithm 
                problem in polynomial time on a quantum computer, breaking ECDSA signatures.
              </p>
            </div>
          </div>
          
          {/* FALCON卡片 */}
          <div className="bg-gradient-to-br from-cyan-500/20 to-blue-500/20 rounded-2xl p-6 border border-cyan-500/30">
            <div className="flex items-center gap-3 mb-6">
              <div className="w-12 h-12 rounded-xl bg-gradient-to-r from-cyan-500 to-blue-500 flex items-center justify-center text-2xl">
                🛡️
              </div>
              <div>
                <h3 className="text-xl font-bold text-white">{data.falcon.algorithm}</h3>
                <p className="text-sm text-cyan-400">Quantum-Resistant</p>
              </div>
            </div>
            
            <ComparisonStats data={data.falcon} />
            
            <div className="mt-6 p-4 bg-green-500/20 rounded-xl border border-green-500/30">
              <p className="text-green-300 text-sm">
                ✅ <strong>Quantum-Safe:</strong> Based on NTRU lattice problems, which are believed 
                to be hard for both classical and quantum computers to solve.
              </p>
            </div>
          </div>
        </div>
      )}
      
      {/* 可视化签名大小对比 */}
      <div className="bg-white/5 backdrop-blur-lg rounded-2xl p-8 border border-white/10">
        <h3 className="text-xl font-bold text-white text-center mb-8">📏 Signature Size Comparison</h3>
        
        <div className="space-y-6">
          {/* ECDSA签名 */}
          <div>
            <div className="flex justify-between text-sm mb-2">
              <span className="text-gray-400">ECDSA Signature</span>
              <span className="text-orange-400">{data.ecdsa.signatureSize} bytes</span>
            </div>
            <div className="h-4 bg-gray-700 rounded-full overflow-hidden">
              <div 
                className="h-full bg-gradient-to-r from-orange-500 to-yellow-500 rounded-full transition-all duration-1000"
                style={{ width: `${(data.ecdsa.signatureSize / data.falcon.signatureSize) * 100}%` }}
              />
            </div>
          </div>
          
          {/* FALCON签名 */}
          <div>
            <div className="flex justify-between text-sm mb-2">
              <span className="text-gray-400">FALCON-1024 Signature</span>
              <span className="text-cyan-400">{data.falcon.signatureSize} bytes</span>
            </div>
            <div className="h-4 bg-gray-700 rounded-full overflow-hidden">
              <div 
                className="h-full bg-gradient-to-r from-cyan-500 to-blue-500 rounded-full transition-all duration-1000"
                style={{ width: '100%' }}
              />
            </div>
          </div>
        </div>
        
        <div className="mt-6 text-center text-gray-400 text-sm">
          FALCON signatures are <span className="text-cyan-400 font-bold">{Math.round(data.falcon.signatureSize / data.ecdsa.signatureSize)}x</span> larger, 
          but provide <span className="text-green-400 font-bold">256-bit</span> quantum-resistant security
        </div>
      </div>
      
      {/* 为什么选择FALCON */}
      <div className="bg-gradient-to-r from-cyan-500/10 to-purple-500/10 rounded-2xl p-8 border border-cyan-500/20">
        <h3 className="text-xl font-bold text-white text-center mb-6">💡 Why FALCON for QuantumShield Bridge?</h3>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {[
            {
              icon: '🔒',
              title: 'Long-term Security',
              description: 'Cross-chain bridges hold funds for extended periods. FALCON protects against "harvest now, decrypt later" attacks.'
            },
            {
              icon: '⚡',
              title: 'Efficient Verification',
              description: 'FFT-based verification is faster than lattice operations, suitable for high-throughput bridge operations.'
            },
            {
              icon: '🌐',
              title: 'Standardized',
              description: 'FALCON is NIST PQC standardized and used by leading projects like qTesla and PQCRYPTO.'
            }
          ].map((item, i) => (
            <div key={i} className="text-center p-4">
              <div className="text-4xl mb-3">{item.icon}</div>
              <h4 className="text-white font-bold mb-2">{item.title}</h4>
              <p className="text-gray-400 text-sm">{item.description}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

// ==================== 子组件 ====================

function TabButton({
  active,
  onClick,
  color,
  children
}: {
  active: boolean;
  onClick: () => void;
  color: string;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={`px-6 py-3 rounded-xl font-medium transition ${
        active
          ? `bg-gradient-to-r ${color} text-white shadow-lg`
          : 'bg-white/10 text-gray-400 hover:bg-white/20'
      }`}
    >
      {children}
    </button>
  );
}

function ComparisonStats({ data }: { data: SignatureData }) {
  const stats = [
    { label: 'Signature Size', value: `${data.signatureSize} bytes`, icon: '📝' },
    { label: 'Public Key Size', value: `${data.publicKeySize} bytes`, icon: '🔑' },
    { label: 'Security Level', value: data.securityLevel, icon: '🛡️' },
    { label: 'Quantum Resistant', value: data.quantumResistant ? 'Yes ✓' : 'No ✗', icon: '🔐' }
  ];
  
  return (
    <div className="grid grid-cols-2 gap-4">
      {stats.map((stat, i) => (
        <div key={i} className="bg-white/5 rounded-xl p-4">
          <div className="text-lg mb-1">{stat.icon}</div>
          <div className="text-xs text-gray-400">{stat.label}</div>
          <div className={`text-lg font-bold ${stat.value.includes('Yes') ? 'text-green-400' : stat.value.includes('No') ? 'text-red-400' : 'text-white'}`}>
            {stat.value}
          </div>
        </div>
      ))}
    </div>
  );
}

function AlgorithmCard({
  type,
  data,
  info
}: {
  type: 'ecdsa' | 'falcon';
  data: SignatureData;
  info: typeof ALGORITHM_INFO.ecdsa;
}) {
  return (
    <div className={`rounded-2xl p-8 border ${
      type === 'ecdsa' 
        ? 'bg-gradient-to-br from-orange-500/10 to-yellow-500/10 border-orange-500/30'
        : 'bg-gradient-to-br from-cyan-500/10 to-blue-500/10 border-cyan-500/30'
    }`}>
      <div className="text-center mb-8">
        <div className={`inline-block px-4 py-2 rounded-full text-sm font-medium mb-4 ${
          type === 'ecdsa' ? 'bg-orange-500/20 text-orange-400' : 'bg-cyan-500/20 text-cyan-400'
        }`}>
          {type === 'ecdsa' ? '⚠️ Quantum-Vulnerable' : '🛡️ Quantum-Safe'}
        </div>
        <h3 className="text-3xl font-bold text-white mb-2">{info.name}</h3>
        <p className="text-gray-400">{info.description}</p>
      </div>
      
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
        {[
          { label: 'Signature Size', value: `${data.signatureSize} bytes` },
          { label: 'Public Key Size', value: `${data.publicKeySize} bytes` },
          { label: 'Security Level', value: data.securityLevel },
          { label: 'Quantum Resistant', value: data.quantumResistant ? 'Yes' : 'No' }
        ].map((item, i) => (
          <div key={i} className="text-center p-4 bg-white/5 rounded-xl">
            <div className="text-gray-400 text-sm mb-1">{item.label}</div>
            <div className={`text-xl font-bold ${item.value === 'Yes' ? 'text-green-400' : item.value === 'No' ? 'text-red-400' : 'text-white'}`}>
              {item.value}
            </div>
          </div>
        ))}
      </div>
      
      <div className="space-y-3">
        <h4 className="text-lg font-bold text-white">How It Works:</h4>
        {info.howItWorks.map((step, i) => (
          <div key={i} className="flex gap-3">
            <span className={`w-6 h-6 rounded-full flex items-center justify-center text-sm font-bold flex-shrink-0 ${
              type === 'ecdsa' ? 'bg-orange-500/30 text-orange-400' : 'bg-cyan-500/30 text-cyan-400'
            }`}>
              {i + 1}
            </span>
            <p className="text-gray-300 text-sm">{step}</p>
          </div>
        ))}
      </div>
      
      <div className={`mt-6 p-4 rounded-xl ${
        type === 'ecdsa' ? 'bg-red-500/10 border border-red-500/30' : 'bg-green-500/10 border border-green-500/30'
      }`}>
        <p className={`text-sm ${type === 'ecdsa' ? 'text-red-300' : 'text-green-300'}`}>
          <strong>{type === 'ecdsa' ? 'Vulnerability:' : 'Security:'}</strong> {info.vulnerability}
        </p>
      </div>
    </div>
  );
}
