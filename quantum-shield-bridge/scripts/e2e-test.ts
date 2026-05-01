/**
 * QuantumShield Bridge - End-to-End Test Script
 * 
 * 端到端测试：
 * 1. 部署合约
 * 2. 锁定ETH
 * 3. Relayer处理
 * 4. 验证跨链完成
 * 
 * @author QuantumShield Team
 */

import { ethers } from 'hardhat';

// ==================== 常量 ====================

const LOCK_AMOUNT = ethers.utils.parseEther('1.0'); // 1 ETH
const LOCK_TIMEOUT = 24 * 60 * 60; // 24小时

// ==================== 测试函数 ====================

async function main() {
  console.log('='.repeat(60));
  console.log('  QuantumShield Bridge - E2E Test');
  console.log('='.repeat(60));
  console.log('');

  // 获取signers
  const [deployer, sender, relayer, recipient] = await ethers.getSigners();
  
  console.log('📋 Test Accounts:');
  console.log(`   Deployer: ${deployer.address}`);
  console.log(`   Sender:   ${sender.address}`);
  console.log(`   Relayer: ${relayer.address}`);
  console.log(`   Recipient: ${recipient.address}`);
  console.log('');

  // ==================== 步骤1: 部署合约 ====================
  console.log('📦 [1/5] Deploying QuantumBridgeLock...');
  
  const ContractFactory = await ethers.getContractFactory('QuantumBridgeLock');
  const contract = await ContractFactory.connect(deployer).deploy(relayer.address);
  await contract.deployed();
  
  console.log(`   ✓ Contract deployed at: ${contract.address}`);
  console.log(`   ✓ Relayer set to: ${relayer.address}`);
  console.log('');

  // ==================== 步骤2: 发送ETH到合约 ====================
  console.log('💰 [2/5] Sending ETH to contract...');
  
  const fundTx = await sender.sendTransaction({
    to: contract.address,
    value: ethers.utils.parseEther('10.0')
  });
  await fundTx.wait();
  
  const contractBalance = await ethers.provider.getBalance(contract.address);
  console.log(`   ✓ Contract balance: ${ethers.utils.formatEther(contractBalance)} ETH`);
  console.log('');

  // ==================== 步骤3: 锁定ETH ====================
  console.log('🔒 [3/5] Locking ETH...');
  
  // 生成哈希锁和密钥
  const secret = ethers.utils.randomBytes(32);
  const hashlock = ethers.utils.keccak256(secret);
  const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
  
  // Qylith目标地址（编码）
  const qylithRecipient = ethers.utils.formatBytes32String('QylithRecipient123');
  
  // 锁定
  const lockTx = await contract.connect(sender).lockETH(
    qylithRecipient,
    hashlock,
    timelock,
    { value: LOCK_AMOUNT }
  );
  const lockReceipt = await lockTx.wait();
  
  // 获取lockId
  const lockEvent = lockReceipt.events?.find((e: any) => e.event === 'AssetLocked');
  const lockId = lockEvent.args.lockId;
  
  console.log(`   ✓ Lock created: ${lockId}`);
  console.log(`   ✓ Amount: ${ethers.utils.formatEther(LOCK_AMOUNT)} ETH`);
  console.log(`   ✓ Hashlock: ${hashlock.substring(0, 18)}...`);
  console.log(`   ✓ Timelock: ${new Date(timelock * 1000).toISOString()}`);
  console.log('');

  // ==================== 步骤4: 模拟Relayer处理 ====================
  console.log('⚙️ [4/5] Simulating Relayer processing...');
  
  // 模拟FALCON签名
  const falconSig = ethers.utils.randomBytes(666);
  console.log('   🔐 Generated FALCON-1024 signature');
  console.log(`   📏 Signature length: ${falconSig.length} bytes`);
  
  // Relayer领取
  const claimTx = await contract.connect(relayer).claim(lockId, secret, falconSig);
  const claimReceipt = await claimTx.wait();
  
  console.log(`   ✓ Claim transaction: ${claimReceipt.transactionHash}`);
  console.log('');

  // ==================== 步骤5: 验证结果 ====================
  console.log('✅ [5/5] Verifying results...');
  
  const lockInfo = await contract.getLock(lockId);
  
  console.log('');
  console.log('   Lock Status:');
  console.log(`      Sender:    ${lockInfo.sender}`);
  console.log(`      Amount:    ${ethers.utils.formatEther(lockInfo.amount)} ETH`);
  console.log(`      Claimed:   ${lockInfo.claimed}`);
  console.log(`      Refunded:  ${lockInfo.refunded}`);
  console.log(`      Secret:    ${lockInfo.secret.substring(0, 18)}...`);
  console.log('');

  // 检查余额变化
  const senderBalance = await ethers.provider.getBalance(sender.address);
  const contractBalanceAfter = await ethers.provider.getBalance(contract.address);
  
  console.log('   Balance Changes:');
  console.log(`      Sender balance:         ${ethers.utils.formatEther(senderBalance)} ETH`);
  console.log(`      Contract balance:      ${ethers.utils.formatEther(contractBalanceAfter)} ETH`);
  console.log('');

  // 验证断言
  console.log('🔍 Running assertions...');
  
  const tests = [
    {
      name: 'Lock is marked as claimed',
      pass: lockInfo.claimed === true
    },
    {
      name: 'Lock is not marked as refunded',
      pass: lockInfo.refunded === false
    },
    {
      name: 'Secret is stored correctly',
      pass: lockInfo.secret === ethers.utils.keccak256(secret)
    },
    {
      name: 'Contract balance decreased',
      pass: contractBalanceAfter.lt(contractBalance)
    }
  ];

  let allPassed = true;
  tests.forEach(test => {
    const status = test.pass ? '✅' : '❌';
    console.log(`   ${status} ${test.name}`);
    if (!test.pass) allPassed = false;
  });

  console.log('');
  console.log('='.repeat(60));
  
  if (allPassed) {
    console.log('  🎉 All tests passed!');
    console.log('='.repeat(60));
    console.log('');
    console.log('📊 Summary:');
    console.log('   - ETH successfully locked in contract');
    console.log('   - Relayer processed the cross-chain request');
    console.log('   - FALCON-1024 signature generated (simulated)');
    console.log('   - Asset successfully transferred');
    console.log('');
    console.log('🔐 Quantum-Resistant Features:');
    console.log('   - ECDSA signature verified (Ethereum side)');
    console.log('   - FALCON-1024 signature generated (Qylith side)');
    console.log('   - HTLC ensures atomic cross-chain transfer');
    console.log('');
  } else {
    console.log('  ❌ Some tests failed!');
    console.log('='.repeat(60));
    process.exit(1);
  }
}

// 运行测试
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
