import { expect } from "chai";
import { ethers } from "hardhat";
import { BigNumber } from "ethers";
import { QuantumBridgeLock } from "../typechain-types";

// 测试常量
const LOCK_TIMEOUT = 24 * 60 * 60; // 24小时
const MIN_LOCK_AMOUNT = ethers.utils.parseEther("0.001"); // 0.001 ETH
const TEST_AMOUNT = ethers.utils.parseEther("1.0"); // 1 ETH

describe("QuantumBridgeLock", function () {
  let contract: QuantumBridgeLock;
  let owner: any;
  let sender: any;
  let relayer: any;
  let recipient: any;
  
  // 测试用的哈希锁和密钥
  const SECRET = ethers.utils.randomBytes(32);
  const HASHLOCK = ethers.utils.keccak256(SECRET);
  
  beforeEach(async function () {
    // 获取测试账户
    [owner, sender, relayer, recipient] = await ethers.getSigners();
    
    // 部署合约
    const ContractFactory = await ethers.getContractFactory("QuantumBridgeLock");
    contract = await ContractFactory.connect(owner).deploy(relayer.address) as QuantumBridgeLock;
    await contract.deployed();
  });
  
  describe("部署", function () {
    it("应该正确设置owner", async function () {
      expect(await contract.owner()).to.equal(owner.address);
    });
    
    it("应该正确设置relayer", async function () {
      expect(await contract.relayer()).to.equal(relayer.address);
    });
    
    it("应该设置默认超时时间", async function () {
      const timeout = await contract.lockTimeout();
      expect(timeout).to.equal(LOCK_TIMEOUT);
    });
    
    it("应该设置最小锁定金额", async function () {
      const minAmount = await contract.minLockAmount();
      expect(minAmount).to.equal(MIN_LOCK_AMOUNT);
    });
  });
  
  describe("lockETH - 锁定ETH", function () {
    let qylithRecipient = "0x516e706f6c6b616464727a747175616e74756d736869656c64"; // "1epolkaddrquantumshield" 编码
  
    it("应该成功锁定ETH", async function () {
      const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      
      // 监听AssetLocked事件
      const tx = await contract.connect(sender).lockETH(
        qylithRecipient,
        HASHLOCK,
        timelock,
        { value: TEST_AMOUNT }
      );
      
      const receipt = await tx.wait();
      
      // 验证事件
      const event = receipt.events?.find((e: any) => e.event === "AssetLocked");
      expect(event).to.not.be.undefined;
      expect(event.args.sender).to.equal(sender.address);
      expect(event.args.token).to.equal(ethers.constants.AddressZero);
      expect(event.args.amount).to.equal(TEST_AMOUNT);
      
      // 验证锁定记录
      const lockId = event.args.lockId;
      const lockInfo = await contract.getLock(lockId);
      expect(lockInfo.sender).to.equal(sender.address);
      expect(lockInfo.claimed).to.equal(false);
      expect(lockInfo.refunded).to.equal(false);
    });
    
    it("应该拒绝小于最小金额的锁定", async function () {
      const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      const smallAmount = ethers.utils.parseEther("0.0001");
      
      await expect(
        contract.connect(sender).lockETH(
          qylithRecipient,
          HASHLOCK,
          timelock,
          { value: smallAmount }
        )
      ).to.be.revertedWithCustomError(contract, "AmountTooSmall");
    });
    
    it("应该拒绝过去的时间锁", async function () {
      const pastTimelock = Math.floor(Date.now() / 1000) - 1000;
      
      await expect(
        contract.connect(sender).lockETH(
          qylithRecipient,
          HASHLOCK,
          pastTimelock,
          { value: TEST_AMOUNT }
        )
      ).to.be.revertedWithCustomError(contract, "InvalidTimelock");
    });
    
    it("应该拒绝空的哈希锁", async function () {
      const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      
      await expect(
        contract.connect(sender).lockETH(
          qylithRecipient,
          ethers.constants.HashZero,
          timelock,
          { value: TEST_AMOUNT }
        )
      ).to.be.revertedWithCustomError(contract, "InvalidHashlock");
    });
    
    it("应该正确增加nonce", async function () {
      const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      const initialNonce = await contract.nonces(sender.address);
      
      await contract.connect(sender).lockETH(
        qylithRecipient,
        HASHLOCK,
        timelock,
        { value: TEST_AMOUNT }
      );
      
      const newNonce = await contract.nonces(sender.address);
      expect(newNonce).to.equal(initialNonce.add(1));
    });
  });
  
  describe("claim - 领取资产", function () {
    let lockId: string;
    let timelock: number;
    let qylithRecipient = "0x516e706f6c6b616464727a747175616e74756d736869656c64";
    let falconsig: string;
    
    beforeEach(async function () {
      // 创建一个锁定
      timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      const tx = await contract.connect(sender).lockETH(
        qylithRecipient,
        HASHLOCK,
        timelock,
        { value: TEST_AMOUNT }
      );
      const receipt = await tx.wait();
      lockId = receipt.events?.find((e: any) => e.event === "AssetLocked").args.lockId;
      
      // 模拟FALCON签名（实际由Relayer生成）
      falconsig = ethers.utils.randomBytes(666); // FALCON-1024签名约666字节
    });
    
    it("应该成功领取资产", async function () {
      const initialBalance = await ethers.provider.getBalance(sender.address);
      
      // Relayer调用claim
      const tx = await contract.connect(relayer).claim(lockId, SECRET, falconsig);
      const receipt = await tx.wait();
      
      // 验证事件
      const event = receipt.events?.find((e: any) => e.event === "AssetClaimed");
      expect(event).to.not.be.undefined;
      expect(event.args.lockId).to.equal(lockId);
      
      // 验证锁定状态
      const lockInfo = await contract.getLock(lockId);
      expect(lockInfo.claimed).to.equal(true);
      expect(lockInfo.secret).to.equal(ethers.utils.keccak256(SECRET));
    });
    
    it("应该拒绝非Relayer调用", async function () {
      await expect(
        contract.connect(sender).claim(lockId, SECRET, falconsig)
      ).to.be.revertedWithCustomError(contract, "OnlyRelayer");
    });
    
    it("应该拒绝错误的密钥", async function () {
      const wrongSecret = ethers.utils.randomBytes(32);
      
      await expect(
        contract.connect(relayer).claim(lockId, wrongSecret, falconsig)
      ).to.be.revertedWithCustomError(contract, "InvalidSecret");
    });
    
    it("应该拒绝重复领取", async function () {
      await contract.connect(relayer).claim(lockId, SECRET, falconsig);
      
      await expect(
        contract.connect(relayer).claim(lockId, SECRET, falconsig)
      ).to.be.revertedWithCustomError(contract, "LockAlreadyClaimed");
    });
  });
  
  describe("refund - 超时退款", function () {
    let lockId: string;
    let qylithRecipient = "0x516e706f6c6b616464727a747175616e74756d736869656c64";
    
    it("应该成功退款", async function () {
      // 创建锁定，超时时间设为1秒（用于测试）
      const ContractFactory = await ethers.getContractFactory("QuantumBridgeLock");
      const testContract = await ContractFactory.connect(owner).deploy(relayer.address) as QuantumBridgeLock;
      await testContract.deployed();
      
      // 设置超短超时（测试用）
      await testContract.updateLockTimeout(1);
      
      const timelock = Math.floor(Date.now() / 1000) + 1;
      const tx = await testContract.connect(sender).lockETH(
        qylithRecipient,
        HASHLOCK,
        timelock,
        { value: TEST_AMOUNT }
      );
      const receipt = await tx.wait();
      lockId = receipt.events?.find((e: any) => e.event === "AssetLocked").args.lockId;
      
      // 等待超时
      await ethers.provider.send("evm_increaseTime", [2]);
      await ethers.provider.send("evm_mine", []);
      
      // 任何人可以触发退款
      const tx2 = await testContract.connect(recipient).refund(lockId);
      const receipt2 = await tx2.wait();
      
      // 验证事件
      const event = receipt2.events?.find((e: any) => e.event === "AssetRefunded");
      expect(event).to.not.be.undefined;
      expect(event.args.reason).to.equal("TIMEOUT");
    });
    
    it("应该拒绝未超时的退款", async function () {
      // 创建锁定，超时时间设为24小时
      const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      const tx = await contract.connect(sender).lockETH(
        qylithRecipient,
        HASHLOCK,
        timelock,
        { value: TEST_AMOUNT }
      );
      const receipt = await tx.wait();
      lockId = receipt.events?.find((e: any) => e.event === "AssetLocked").args.lockId;
      
      await expect(
        contract.connect(sender).refund(lockId)
      ).to.be.revertedWithCustomError(contract, "LockNotExpired");
    });
  });
  
  describe("管理函数", function () {
    it("应该允许owner更新relayer", async function () {
      const newRelayer = recipient.address;
      
      await expect(contract.connect(owner).updateRelayer(newRelayer))
        .to.emit(contract, "RelayerUpdated")
        .withArgs(relayer.address, newRelayer);
      
      expect(await contract.relayer()).to.equal(newRelayer);
    });
    
    it("应该拒绝非owner更新relayer", async function () {
      await expect(
        contract.connect(sender).updateRelayer(recipient.address)
      ).to.be.revertedWith("Only owner");
    });
    
    it("应该允许owner更新超时时间", async function () {
      const newTimeout = 48 * 60 * 60; // 48小时
      
      await expect(contract.connect(owner).updateLockTimeout(newTimeout))
        .to.emit(contract, "TimeoutUpdated")
        .withArgs(LOCK_TIMEOUT, newTimeout);
      
      expect(await contract.lockTimeout()).to.equal(newTimeout);
    });
  });
  
  describe("getLock - 查询锁定", function () {
    it("应该返回正确的锁定信息", async function () {
      const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      const qylithRecipient = "0x516e706f6c6b616464727a747175616e74756d736869656c64";
      
      const tx = await contract.connect(sender).lockETH(
        qylithRecipient,
        HASHLOCK,
        timelock,
        { value: TEST_AMOUNT }
      );
      const receipt = await tx.wait();
      const lockId = receipt.events?.find((e: any) => e.event === "AssetLocked").args.lockId;
      
      const lockInfo = await contract.getLock(lockId);
      
      expect(lockInfo.sender).to.equal(sender.address);
      expect(lockInfo.token).to.equal(ethers.constants.AddressZero);
      expect(lockInfo.amount).to.equal(TEST_AMOUNT);
      expect(lockInfo.hashlock).to.equal(HASHLOCK);
      expect(lockInfo.timelock).to.equal(timelock);
      expect(lockInfo.claimed).to.equal(false);
      expect(lockInfo.refunded).to.equal(false);
    });
    
    it("应该返回空锁定（不存在的lockId）", async function () {
      const fakeLockId = ethers.utils.keccak256(ethers.utils.randomBytes(32));
      const lockInfo = await contract.getLock(fakeLockId);
      
      expect(lockInfo.sender).to.equal(ethers.constants.AddressZero);
    });
  });
  
  describe("事件验证", function () {
    it("应该触发正确的AssetLocked事件", async function () {
      const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      const qylithRecipient = "0x516e706f6c6b616464727a747175616e74756d736869656c64";
      
      await expect(
        contract.connect(sender).lockETH(
          qylithRecipient,
          HASHLOCK,
          timelock,
          { value: TEST_AMOUNT }
        )
      ).to.emit(contract, "AssetLocked");
    });
    
    it("应该触发正确的AssetClaimed事件", async function () {
      const timelock = Math.floor(Date.now() / 1000) + LOCK_TIMEOUT;
      const qylithRecipient = "0x516e706f6c6b616464727a747175616e74756d736869656c64";
      const falconsig = ethers.utils.randomBytes(666);
      
      // 先锁定
      const tx = await contract.connect(sender).lockETH(
        qylithRecipient,
        HASHLOCK,
        timelock,
        { value: TEST_AMOUNT }
      );
      const receipt = await tx.wait();
      const lockId = receipt.events?.find((e: any) => e.event === "AssetLocked").args.lockId;
      
      // 再领取
      await expect(
        contract.connect(relayer).claim(lockId, SECRET, falconsig)
      ).to.emit(contract, "AssetClaimed");
    });
  });
});
