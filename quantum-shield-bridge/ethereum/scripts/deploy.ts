/**
 * Deployment Script for QuantumBridgeLock
 * 
 * Deploys the contract to local Hardhat network or testnet
 */

import { ethers } from 'hardhat';

async function main() {
  console.log('📦 Deploying QuantumBridgeLock...');
  
  // 获取部署者账户
  const [deployer, relayer] = await ethers.getSigners();
  
  console.log('   Deployer:', deployer.address);
  console.log('   Relayer:', relayer.address);
  
  // 部署合约
  const ContractFactory = await ethers.getContractFactory('QuantumBridgeLock');
  const contract = await ContractFactory.connect(deployer).deploy(relayer.address);
  
  // 等待部署确认
  await contract.deployed();
  
  console.log('');
  console.log('✅ Contract deployed successfully!');
  console.log('   Contract Address:', contract.address);
  console.log('   Transaction Hash:', contract.deployTransaction.hash);
  console.log('');
  
  // 输出部署信息
  console.log('📋 Deployment Summary:');
  console.log('   Network: ', (await ethers.provider.getNetwork()).name);
  console.log('   Chain ID: ', (await ethers.provider.getNetwork()).chainId.toString());
  console.log('');
  
  // 保存部署地址供后续使用
  console.log('📝 Copy this address to your Relayer .env file:');
  console.log(`   ETHEREUM_CONTRACT_ADDRESS=${contract.address}`);
  console.log('');
  
  // 验证合约
  console.log('🔍 Verifying contract...');
  const owner = await contract.owner();
  const contractRelayer = await contract.relayer();
  
  console.log('   Owner:', owner);
  console.log('   Relayer:', contractRelayer);
  console.log('');
  
  if (owner === deployer.address && contractRelayer === relayer.address) {
    console.log('✅ Contract verification passed!');
  } else {
    console.log('❌ Contract verification failed!');
  }
}

main()
  .then(() => {
    console.log('');
    console.log('🚀 Deployment complete!');
    process.exit(0);
  })
  .catch((error) => {
    console.error('❌ Deployment failed:', error);
    process.exit(1);
  });
