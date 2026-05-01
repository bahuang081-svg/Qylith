# Qylith链杀手级DApp设计

## 执行摘要

**Qylith定位**：全球第一条AI原生抗量子L1公链，核心技术栈为ML-DSA-87+ML-KEM-768密码学算法，配合AI Agent Execution Module (AEM)预编译模块。

**设计原则**：每个DApp必须回答"为什么必须在Qylith上跑"——答案在于AEM的原生AI执行能力与抗量子密码学的独家组合，这是以太坊、Solana等现有公链无法复制的差异化优势。

---

## 应用一：QylithAI Agent Hub

### 1.1 产品定义与愿景

**产品定位**：首个链上AI Agent市场与执行环境，让AI Agent成为可注册身份、积累声誉、提供服务并自主收取报酬的数字公民。

**核心价值主张**：
- **对Agent开发者**：一次开发，多链可验证声誉，ML-DSA签名确保Agent身份不可伪造
- **对Agent用户**：可组合的AI服务市场，智能合约自动结算，无需信任中心化中间商
- **对Qylith链**：AEM模块的杀手级用例，AI执行即GAS消耗，TVL自然增长

**用户故事**：
```
作为Agent开发者Alice：
我可以在Qylith上注册我的TradingBot Agent，
它用ML-DSA签名证明"这是我开发的正版Agent"，
用户质押$QYLITH代币才能调用我的Agent服务，
服务完成后智能合约自动分账——50%归我，50%进入Agent发展基金。
```

### 1.2 核心智能合约架构

#### 合约1：AgentRegistry（代理注册合约）

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface IAEModule {
    // AEM预编译模块的接口
    function executeAIAgent(bytes32 agentId, bytes calldata taskInput) 
        external returns (bytes memory output);
}

contract AgentRegistry {
    
    // ============ 数据结构 ============
    
    struct Agent {
        bytes32 agentId;              // Agent唯一标识（keccak256 hash）
        address developer;            // 开发者地址
        string metadataURI;          // IPFS上的Agent元数据（名称、能力描述等）
        uint256 reputationScore;     // 声誉分数（0-1000）
        uint256 totalTasks;          // 累计完成任务数
        uint256 totalEarnings;       // 累计收益（wei）
        uint8 serviceCount;           // 提供的服务种类数
        bool isActive;               // 是否启用
        MLDSASignature identityProof; // ML-DSA身份证明
    }
    
    struct MLDSASignature {
        bytes signature;             // ML-DSA-87签名
        bytes publicKey;             // ML-DSA公钥
        uint256 timestamp;           // 签名时间戳
        bytes32 signedAgentId;       // AgentId的签名
    }
    
    struct Service {
        bytes32 serviceId;
        bytes32 agentId;
        string name;
        string description;
        uint256 pricePerCall;        // 每次调用价格（wei）
        uint256 minStakeRequired;    // 最低质押要求
        uint256 maxConcurrentCalls; // 最大并发调用数
        bytes aemTaskType;           // AEM任务类型标识
    }
    
    // ============ 状态变量 ============
    
    IAEModule public immutable AEM_MODULE;
    uint256 public constant PLATFORM_FEE_RATE = 250; // 2.5%平台费
    uint256 public constant MIN_STAKE_DURATION = 1 days;
    
    mapping(bytes32 => Agent) public agents;
    mapping(bytes32 => Service) public services;
    mapping(bytes32 => mapping(address => uint256)) public developerStakes; // Agent质押
    mapping(bytes32 => bytes32[]) public agentCollaborations; // 协作关系
    
    // ============ 事件 ============
    
    event AgentRegistered(bytes32 indexed agentId, address indexed developer);
    event ServiceCreated(bytes32 indexed serviceId, bytes32 indexed agentId);
    event AgentReputationUpdated(bytes32 indexed agentId, uint256 newScore);
    event CollaborationFormed(bytes32 indexed agentA, bytes32 indexed agentB);
    
    // ============ 核心函数 ============
    
    /**
     * @notice 注册新AI Agent
     * @param metadataURI Agent元数据IPFS URI
     * @param identityProof ML-DSA身份证明
     */
    function registerAgent(
        string calldata metadataURI,
        MLDSASignature calldata identityProof
    ) external returns (bytes32 agentId) {
        // 1. 验证ML-DSA身份证明
        require(verifyIdentityProof(identityProof), "Invalid identity proof");
        
        // 2. 生成AgentId
        agentId = keccak256(abi.encode(identityProof.publicKey, msg.sender));
        
        // 3. 防止重复注册
        require(agents[agentId].developer == address(0), "Agent already registered");
        
        // 4. 创建Agent记录
        agents[agentId] = Agent({
            agentId: agentId,
            developer: msg.sender,
            metadataURI: metadataURI,
            reputationScore: 500, // 初始500分
            totalTasks: 0,
            totalEarnings: 0,
            serviceCount: 0,
            isActive: true,
            identityProof: identityProof
        });
        
        emit AgentRegistered(agentId, msg.sender);
    }
    
    /**
     * @notice 验证ML-DSA身份证明
     * @dev 调用Qylith链的预编译PQC验证合约
     */
    function verifyIdentityProof(MLDSASignature calldata proof) 
        internal view returns (bool) {
        // 使用Qylith预编译的PQC验证合约
        // 验证签名内容包含签名者的公钥和地址
        bytes32 signedData = keccak256(abi.encode(proof.publicKey, msg.sender));
        return _verifyMLDSASignature(proof.signature, proof.publicKey, signedData);
    }
    
    /**
     * @notice 创建Agent服务
     */
    function createService(
        bytes32 agentId,
        string calldata name,
        string calldata description,
        uint256 pricePerCall,
        uint256 minStakeRequired,
        bytes calldata aemTaskType
    ) external returns (bytes32 serviceId) {
        require(agents[agentId].developer == msg.sender, "Not the developer");
        require(agents[agentId].isActive, "Agent not active");
        
        serviceId = keccak256(abi.encode(
            agentId, name, block.timestamp, msg.sender
        ));
        
        services[serviceId] = Service({
            serviceId: serviceId,
            agentId: agentId,
            name: name,
            description: description,
            pricePerCall: pricePerCall,
            minStakeRequired: minStakeRequired,
            maxConcurrentCalls: 10,
            aemTaskType: aemTaskType
        });
        
        agents[agentId].serviceCount++;
        emit ServiceCreated(serviceId, agentId);
    }
    
    /**
     * @notice 调用AI Agent服务（核心交互）
     */
    function callAgentService(
        bytes32 serviceId,
        bytes calldata taskInput,
        uint256 stakeAmount
    ) external payable returns (bytes memory output) {
        Service storage service = services[serviceId];
        Agent storage agent = agents[service.agentId];
        
        require(agent.isActive, "Agent not active");
        require(msg.value >= service.pricePerCall + stakeAmount, "Insufficient payment");
        
        // 1. 处理质押（可选）
        if (stakeAmount > 0) {
            require(stakeAmount >= service.minStakeRequired, "Stake too low");
            developerStakes[service.serviceId][msg.sender] += stakeAmount;
        }
        
        // 2. 调用AEM模块执行AI任务
        output = AEM_MODULE.executeAIAgent(service.agentId, taskInput);
        
        // 3. 计算分账
        uint256 basePayment = service.pricePerCall;
        uint256 platformFee = (basePayment * PLATFORM_FEE_RATE) / 10000;
        uint256 developerPayment = basePayment - platformFee;
        
        // 4. 链上分账
        payable(agent.developer).transfer(developerPayment);
        // 平台费进入生态基金
        
        // 5. 更新统计
        agent.totalTasks++;
        agent.totalEarnings += developerPayment;
        
        return output;
    }
    
    /**
     * @notice Multi-Agent协作编排
     */
    function createCollaboration(
        bytes32[] calldata agentIds,
        bytes32[] calldata serviceIds,
        uint256[] calldata weightDistribution
    ) external returns (bytes32 collabId) {
        require(agentIds.length == serviceIds.length, "Mismatch");
        require(_validateWeights(weightDistribution), "Invalid weights");
        
        collabId = keccak256(abi.encode(agentIds, block.timestamp));
        
        for (uint i = 0; i < agentIds.length; i++) {
            agentCollaborations[agentIds[i]].push(collabId);
        }
        
        emit CollaborationFormed(agentIds[0], agentIds[1]);
    }
    
    /**
     * @notice 声誉系统更新
     */
    function updateReputation(
        bytes32 agentId,
        int256 delta,
        string calldata reason
    ) external {
        Agent storage agent = agents[agentId];
        
        // 声誉计算公式：
        // newScore = oldScore + delta * (completeness_factor) * (time_factor)
        // delta范围: +50(完美) 到 -100(欺诈)
        
        uint256 newScore = uint256(
            int256(agent.reputationScore) + delta
        );
        newScore = newScore > 1000 ? 1000 : newScore; // 上限1000
        
        agent.reputationScore = newScore;
        emit AgentReputationUpdated(agentId, newScore);
    }
    
    // ============ AEM模块集成接口 ============
    
    /**
     * @notice AEM预编译模块调用接口
     * @dev AEM_MODULE_ADDRESS 是Qylith链上的预编译合约地址
     */
    function _verifyMLDSASignature(
        bytes memory signature,
        bytes memory publicKey,
        bytes32 message
    ) internal view returns (bool) {
        // 委托给AEM模块验证
        (bool success, bytes memory result) = address(AEM_MODULE).staticcall(
            abi.encodeWithSignature(
                "verifyMLDSA(bytes,bytes,bytes32)",
                signature,
                publicKey,
                message
            )
        );
        return success && abi.decode(result, (bool));
    }
}
```

#### 合约2：AgentReputation（声誉系统）

```solidity
contract AgentReputation {
    
    struct ReputationRecord {
        uint256 score;              // 综合声誉分数
        uint256 completionRate;     // 任务完成率
        uint256 avgRating;          // 平均评分 (1-5, 放大100倍)
        uint256 responseTime;       // 平均响应时间(ms)
        uint256 totalReviews;       // 总评价数
        uint256 stakeWeight;        // 质押加权系数
    }
    
    struct Review {
        address reviewer;
        uint8 rating;               // 1-5星
        string comment;
        uint256 timestamp;
        bool isPositiveOutcome;     // 结果是否正面
    }
    
    mapping(bytes32 => ReputationRecord) public agentReputations;
    mapping(bytes32 => Review[]) public agentReviews;
    mapping(bytes32 => mapping(address => bool)) public hasReviewed;
    
    // 声誉计算参数
    uint256 public constant MIN_STAKE_FOR_REPUTATION = 1 ether;
    uint256 public constant REPUTATION_HALF_LIFE = 90 days;
    
    /**
     * @notice 提交评价并更新声誉
     */
    function submitReview(
        bytes32 agentId,
        bytes32 serviceId,
        uint8 rating,
        string calldata comment,
        bool isPositiveOutcome
    ) external {
        require(!hasReviewed[agentId][msg.sender], "Already reviewed");
        require(rating >= 1 && rating <= 5, "Invalid rating");
        
        // 存储评价
        Review memory review = Review({
            reviewer: msg.sender,
            rating: rating,
            comment: comment,
            timestamp: block.timestamp,
            isPositiveOutcome: isPositiveOutcome
        });
        agentReviews[agentId].push(review);
        hasReviewed[agentId][msg.sender] = true;
        
        // 声誉计算
        ReputationRecord storage record = agentReputations[agentId];
        
        // 增量更新声誉分数
        uint256 ratingImpact = (rating - 3) * 20; // +40 to -40
        if (isPositiveOutcome) {
            ratingImpact += 10;
        } else {
            ratingImpact -= 30;
        }
        
        record.score = _calculateNewScore(record.score, ratingImpact);
        record.avgRating = (record.avgRating * record.totalReviews + rating * 100) 
            / (record.totalReviews + 1);
        record.totalReviews++;
        
        // 时间衰减因子更新
        record.stakeWeight = _calculateStakeWeight(agentId);
    }
    
    /**
     * @notice 计算声誉分数（含时间衰减）
     */
    function _calculateNewScore(uint256 currentScore, int256 delta) 
        internal pure returns (uint256) {
        int256 newScore = int256(currentScore) + delta;
        if (newScore < 0) return 0;
        if (newScore > 1000) return 1000;
        return uint256(newScore);
    }
    
    /**
     * @notice 计算质押加权系数
     */
    function _calculateStakeWeight(bytes32 agentId) 
        internal view returns (uint256) {
        // 质押越多，声誉权重越高（防女巫攻击）
        uint256 totalStake = IAgentRegistry(registry).getAgentTotalStake(agentId);
        if (totalStake < MIN_STAKE_FOR_REPUTATION) {
            return 100; // 基准100%
        }
        // 质押每增加10倍，权重+50%，上限300%
        uint256 multiplier = 100 + (totalStake / (MIN_STAKE_FOR_REPUTATION * 10)) * 50;
        return multiplier > 300 ? 300 : multiplier;
    }
    
    /**
     * @notice 获取Agent综合声誉（用于服务质量保证）
     */
    function getAgentTrustScore(bytes32 agentId) external view returns (uint256) {
        ReputationRecord memory record = agentReputations[agentId];
        
        // 综合信任分数 = 声誉 * 质押权重 / 100
        return (record.score * record.stakeWeight) / 100;
    }
}
```

#### 合约3：AgentEscrow（托管支付合约）

```solidity
contract AgentEscrow {
    
    enum TaskStatus { 
        Created, 
        InProgress, 
        Submitted, 
        Approved, 
        Disputed, 
        Resolved,
        Cancelled 
    }
    
    struct Task {
        bytes32 taskId;
        bytes32 serviceId;
        bytes32 agentId;
        address client;
        uint256 budget;
        uint256 stakeAmount;
        uint256 deadline;
        TaskStatus status;
        bytes32 submissionHash;     // 提交内容的哈希
        uint256 clientRating;
        uint256 agentRating;
    }
    
    mapping(bytes32 => Task) public tasks;
    mapping(bytes32 => bytes32[]) public taskHistory; // Agent历史任务
    
    // ============ 核心函数 ============
    
    /**
     * @notice 创建托管任务
     */
    function createTask(
        bytes32 serviceId,
        bytes32 agentId,
        uint256 budget,
        uint256 deadline
    ) external payable returns (bytes32 taskId) {
        require(msg.value >= budget, "Insufficient funds");
        
        taskId = keccak256(abi.encode(serviceId, agentId, msg.sender, block.timestamp));
        
        tasks[taskId] = Task({
            taskId: taskId,
            serviceId: serviceId,
            agentId: agentId,
            client: msg.sender,
            budget: msg.value,
            stakeAmount: 0,
            deadline: block.timestamp + deadline,
            status: TaskStatus.Created,
            submissionHash: bytes32(0),
            clientRating: 0,
            agentRating: 0
        });
        
        taskHistory[agentId].push(taskId);
    }
    
    /**
     * @notice Agent提交任务结果
     */
    function submitTaskResult(
        bytes32 taskId,
        bytes32 contentHash
    ) external {
        Task storage task = tasks[taskId];
        require(task.status == TaskStatus.InProgress, "Invalid status");
        require(msg.sender == task.client, "Not the client");
        
        task.submissionHash = contentHash;
        task.status = TaskStatus.Submitted;
    }
    
    /**
     * @notice 客户确认验收
     */
    function approveTask(bytes32 taskId) external {
        Task storage task = tasks[taskId];
        require(task.status == TaskStatus.Submitted, "Invalid status");
        require(msg.sender == task.client, "Not the client");
        
        // 计算分账
        uint256 platformFee = (task.budget * 250) / 10000;
        uint256 agentPayment = task.budget - platformFee;
        
        // 释放支付
        address agentDeveloper = IAgentRegistry(registry).getAgentDeveloper(task.agentId);
        payable(agentDeveloper).transfer(agentPayment);
        
        task.status = TaskStatus.Approved;
    }
    
    /**
     * @notice 开启争议
     */
    function raiseDispute(bytes32 taskId, string calldata reason) external {
        Task storage task = tasks[taskId];
        require(task.status == TaskStatus.Submitted, "Invalid status");
        require(msg.sender == task.client, "Only client can dispute");
        require(block.timestamp > task.deadline, "Deadline not passed");
        
        task.status = TaskStatus.Disputed;
        // 争议进入仲裁流程
    }
}
```

### 1.3 与Qylith AEM模块的集成

**AEM（AI Agent Execution Module）**是Qylith链的预编译模块，提供原生AI执行能力：

```solidity
// AEM模块接口定义
interface IAEModule {
    
    /**
     * @notice 执行AI Agent任务
     * @param agentId Agent唯一标识
     * @param taskInput 任务输入数据
     * @return output AI执行结果
     */
    function executeAIAgent(
        bytes32 agentId,
        bytes calldata taskInput
    ) external returns (bytes memory output);
    
    /**
     * @notice 批量执行多个Agent协作任务
     * @param orchestrationPlan 编排计划（JSON格式的任务依赖图）
     */
    function batchExecuteAgents(
        bytes calldata orchestrationPlan
    ) external returns (bytes[] memory outputs);
    
    /**
     * @notice 验证AI推理结果的正确性（ZK证明）
     */
    function verifyInferenceProof(
        bytes32 agentId,
        bytes calldata inferenceResult,
        bytes calldata zkProof
    ) external returns (bool);
    
    /**
     * @notice 获取Agent的AI模型哈希（确保不可篡改）
     */
    function getAgentModelHash(bytes32 agentId) external view returns (bytes32);
}
```

**集成方式**：
1. Agent注册时，AEM模块记录Agent的AI模型哈希
2. Agent调用时，任务输入直接传入AEM执行
3. 执行结果自动签名并返回，附带ML-DSA验证证明
4. 声誉系统基于执行结果的ZK证明进行评分

### 1.4 代币经济模型

#### $QOR 代币用途

| 用途 | 比例 | 说明 |
|------|------|------|
| 支付服务费 | 30% | 用户调用Agent服务必须质押$QOR |
| 开发者质押 | 25% | 开发者需质押$QOR才能发布服务 |
| 生态基金 | 20% | Grants、赏金、黑客松奖励 |
| 节点质押 | 15% | 验证节点需质押运行Agent服务 |
| 燃烧机制 | 10% | 高频交易服务费部分燃烧 |

#### 质押收益模型

```solidity
// 质押收益计算
struct StakingRewards {
    uint256 baseAPY;           // 基础年化收益率：8%
    uint256 agentMultiplier;   // Agent服务质押加成：+3%
    uint256 durationBonus;     // 长期质押bonus：每90天+1%
    uint256 volumeBonus;       // 高交易量bonus：每月+2%
}

// 最大APY = 8% + 3% + 2% + 2% = 15%
```

### 1.5 MVP范围（黑客松可交付）

**Phase 1 (HTX Hackathon)**：
- [x] AgentRegistry合约（注册、验证、状态管理）
- [x] ML-DSA身份证明验证集成
- [x] 基础声誉系统（评分、评论）
- [x] 简单服务调用（不包含复杂编排）
- [x] 前端Demo：Agent浏览、注册、调用

**Phase 2**：
- [ ] Multi-Agent协作编排
- [ ] 完整托管支付系统
- [ ] 开发者API SDK

**Phase 3**：
- [ ] 跨链Agent互操作
- [ ] ZK证明推理验证
- [ ] 高级声誉算法

### 1.6 竞品对比

| 维度 | QylithAI Hub | SingularityNET | Fetch.ai | Autonolas |
|------|--------------|----------------|----------|-----------|
| 密码学安全 | ML-DSA-87 ✅ | ECDSA | ECDSA | ECDSA |
| 原生AI执行 | AEM模块 ✅ | 需要中间件 | 需要中间件 | 需要中间件 |
| 链上声誉 | 完整实现 ✅ | 部分实现 | 无 | 无 |
| Multi-Agent编排 | 链上原生 ✅ | API层 | API层 | API层 |
| 抗量子 | 是 ✅ | 否 | 否 | 否 |
| 开发难度 | 中等 | 高 | 高 | 高 |

**核心差异化**：Qylith是唯一在密码学层和执行层同时原生支持AI Agent的区块链。AEM模块让Agent执行发生在共识层，而不是像其他项目那样需要依赖链下中间件。

---

## 应用二：QuantumShield Bridge

### 2.1 产品定义与愿景

**产品定位**：首个端到端抗量子跨链桥，连接Qylith与Ethereum、Solana、TRON等主流公链，通过混合签名验证和PQC加密确保跨链通信的长期安全性。

**核心价值主张**：
- **量子威胁的不可逆性**：一旦量子计算机破解ECDSA，历史上的所有跨链交易记录都将被伪造。QuantumShield保护的是"未来的安全"
- **混合签名验证**：源链保持ECDSA兼容性，Qylith侧强制ML-DSA，确保即使源链被攻破，Qylith侧仍然安全
- **HTLC + PQC原子交换**：哈希时间锁合约结合后量子密钥封装，实现真正安全的跨链资产交换

**用户故事**：
```
作为机构用户Bob（持有大量ETH的量化基金）：
我担心10年后量子计算机可能破解我跨链资产的签名。
通过QuantumShield将ETH跨到Qylith，
我的资产用ML-DSA重新加密，
即使量子计算机时代来临，我的资产仍然安全。
同时我可以将资产跨回任何链，享受Qylith的安全保护。
```

### 2.2 核心智能合约架构

#### 合约4：QuantumBridge Core（跨链桥核心合约）

```solidity
contract QuantumBridge {
    
    // ============ 数据结构 ============
    
    enum BridgeStatus { Active, Paused, Deprecated }
    
    struct ChainConfig {
        bool isSupported;
        uint8 chainId;
        uint256 minConfirmations;
        uint256 lockDuration;        // 锁定时间
        bytes32 chainHashAlgorithm;  // 链的哈希算法
        address validatorContract;  // 验证者合约地址
    }
    
    struct TransferRequest {
        bytes32 transferId;
        uint8 sourceChain;
        uint8 destChain;
        address sender;
        address recipient;
        address token;
        uint256 amount;
        uint256 fee;
        uint256 timestamp;
        TransferStatus status;
        bytes32 hashLock;
        uint256 timelockExpiry;
        bytes pqcEncryptedKey;      // PQC加密的密钥
        bytes mlDsaSignature;       // ML-DSA签名证明
    }
    
    enum TransferStatus { 
        Pending, 
        Locked, 
        Minted, 
        Completed, 
        Refunded, 
        Expired 
    }
    
    // ============ 状态变量 ============
    
    // 支持的链配置
    mapping(uint8 => ChainConfig) public chainConfigs;
    
    // 传输记录
    mapping(bytes32 => TransferRequest) public transfers;
    
    // 验证者集合（Qylith验证节点）
    mapping(address => bool) public isValidator;
    uint256 public validatorCount;
    uint256 public constant MIN_VALIDATORS = 3;
    
    // 手续费配置
    uint256 public constant BASE_FEE = 0.001 ether;
    uint256 public constant FEE_RATE = 50; // 0.5%
    
    // 链ID常量
    uint8 public constant QYLITH_CHAIN_ID = 1;
    uint8 public constant ETHEREUM_CHAIN_ID = 2;
    uint8 public constant SOLANA_CHAIN_ID = 3;
    uint8 public constant TRON_CHAIN_ID = 4;
    
    // ============ 事件 ============
    
    event TransferInitiated(
        bytes32 indexed transferId,
        uint8 indexed sourceChain,
        uint8 indexed destChain,
        address sender,
        address recipient,
        uint256 amount
    );
    
    event TransferCompleted(
        bytes32 indexed transferId,
        address indexed recipient,
        uint256 amount
    );
    
    event PQCKeyEncapsulated(
        bytes32 indexed transferId,
        bytes pqcEncryptedKey
    );
    
    // ============ 核心函数 ============
    
    /**
     * @notice 发起跨链转账（锁定资产）
     * @param destChain 目标链ID
     * @param recipient 接收地址（目标链格式）
     * @param token 代币地址（address(0)为原生币）
     * @param amount 金额
     * @param hashLock HTLC哈希锁
     */
    function initiateTransfer(
        uint8 destChain,
        bytes calldata recipient,
        address token,
        uint256 amount,
        bytes32 hashLock
    ) external payable returns (bytes32 transferId) {
        require(chainConfigs[destChain].isSupported, "Unsupported chain");
        
        // 计算手续费
        uint256 fee = _calculateFee(amount);
        require(msg.value >= fee, "Insufficient fee");
        
        // 生成传输ID
        transferId = keccak256(abi.encode(
            msg.sender, recipient, token, amount, hashLock, block.timestamp
        ));
        
        // 锁定源资产
        if (token == address(0)) {
            require(msg.value >= amount + fee, "Insufficient native token");
        } else {
            IERC20(token).transferFrom(msg.sender, address(this), amount);
        }
        
        // PQC密钥封装（核心创新）
        bytes memory pqcKey = _encapsulatePQCKey(transferId);
        
        // 创建传输记录
        transfers[transferId] = TransferRequest({
            transferId: transferId,
            sourceChain: QYLITH_CHAIN_ID,
            destChain: destChain,
            sender: msg.sender,
            recipient: recipient,
            token: token,
            amount: amount,
            fee: fee,
            timestamp: block.timestamp,
            status: TransferStatus.Locked,
            hashLock: hashLock,
            timelockExpiry: block.timestamp + chainConfigs[destChain].lockDuration,
            pqcEncryptedKey: pqcKey,
            mlDsaSignature: bytes("") // 待签名
        });
        
        emit TransferInitiated(transferId, QYLITH_CHAIN_ID, destChain, 
            msg.sender, recipient, amount);
        
        // PQC密钥事件
        emit PQCKeyEncapsulated(transferId, pqcKey);
        
        return transferId;
    }
    
    /**
     * @notice 完成跨链转账（验证并释放资产）
     * @dev 在目标链上调用，验证ML-DSA签名和PQC密钥
     */
    function completeTransfer(
        bytes32 transferId,
        bytes32 secret,                    // HTLC密钥
        bytes memory mlDsaSignature,       // ML-DSA签名
        bytes memory pqcDecryptedKey,      // PQC解密后的密钥
        uint8[] calldata validatorV,        // v值
        bytes32[] calldata validatorR,      // r值
        bytes32[] calldata validatorS       // s值
    ) external {
        TransferRequest storage transfer = transfers[transferId];
        require(transfer.status == TransferStatus.Locked, "Invalid status");
        
        // 1. 验证HTLC哈希锁
        require(transfer.hashLock == keccak256(abi.encodePacked(secret)), 
            "Invalid secret");
        
        // 2. 验证PQC解密结果
        require(_verifyPQCDecryption(transferId, pqcDecryptedKey), 
            "PQC verification failed");
        
        // 3. 验证ML-DSA签名（Qylith链上验证）
        require(_verifyMLDSASignature(transfer, mlDsaSignature), 
            "ML-DSA verification failed");
        
        // 4. 多签验证（来自Qylith验证节点）
        require(_verifyMultiSignature(transferId, validatorV, validatorR, validatorS),
            "Multi-signature verification failed");
        
        // 5. 释放资产
        transfer.status = TransferStatus.Completed;
        
        if (transfer.token == address(0)) {
            payable(msg.sender).transfer(transfer.amount);
        } else {
            IERC20(transfer.token).transfer(msg.sender, transfer.amount);
        }
        
        emit TransferCompleted(transferId, msg.sender, transfer.amount);
    }
    
    /**
     * @notice 取消过期转账并退款
     */
    function refundTransfer(bytes32 transferId) external {
        TransferRequest storage transfer = transfers[transferId];
        require(transfer.status == TransferStatus.Locked, "Invalid status");
        require(block.timestamp > transfer.timelockExpiry, "Not expired");
        require(msg.sender == transfer.sender, "Not the sender");
        
        transfer.status = TransferStatus.Refunded;
        
        // 退款（扣除手续费）
        uint256 refundAmount = transfer.amount;
        if (transfer.token == address(0)) {
            payable(transfer.sender).transfer(refundAmount);
        } else {
            IERC20(transfer.token).transfer(transfer.sender, refundAmount);
        }
    }
    
    // ============ PQC加密层 ============
    
    /**
     * @notice PQC密钥封装
     * @dev 使用ML-KEM-768进行密钥封装
     */
    function _encapsulatePQCKey(bytes32 transferId) 
        internal returns (bytes memory) {
        // 调用Qylith链的ML-KEM-768预编译合约
        // 返回: (encapsulatedKey, sharedSecret)
        (bytes memory encapsulatedKey, bytes memory sharedSecret) = 
            IPQCModule(pqcModuleAddress).kemEncapsulate(
                bytes32ToBytes(transferId)
            );
        
        // 存储共享密钥的哈希（用于后续验证）
        _pqcKeyHash[transferId] = keccak256(sharedSecret);
        
        return encapsulatedKey;
    }
    
    /**
     * @notice PQC密钥验证
     */
    function _verifyPQCDecryption(bytes32 transferId, bytes memory decryptedKey) 
        internal view returns (bool) {
        return _pqcKeyHash[transferId] == keccak256(decryptedKey);
    }
    
    // ============ ML-DSA签名验证 ============
    
    /**
     * @notice 验证ML-DSA签名
     */
    function _verifyMLDSASignature(
        TransferRequest memory transfer,
        bytes memory signature
    ) internal view returns (bool) {
        // 构造待签名消息
        bytes32 message = keccak256(abi.encode(
            transfer.transferId,
            transfer.sourceChain,
            transfer.destChain,
            transfer.recipient,
            transfer.token,
            transfer.amount,
            transfer.pqcEncryptedKey
        ));
        
        // 调用Qylith链的ML-DSA预编译验证
        return IPQCModule(pqcModuleAddress).verifyMLDSA(
            signature,
            _bridgePublicKey,
            message
        );
    }
    
    // ============ 多签验证 ============
    
    /**
     * @notice 验证多签（Qylith验证节点集合签名）
     */
    function _verifyMultiSignature(
        bytes32 transferId,
        uint8[] memory v,
        bytes32[] memory r,
        bytes32[] memory s
    ) internal view returns (bool) {
        require(v.length >= MIN_VALIDATORS, "Not enough validators");
        require(v.length == r.length && r.length == s.length, "Signature mismatch");
        
        // 消息哈希
        bytes32 messageHash = keccak256(abi.encodePacked(transferId));
        
        uint256 validCount = 0;
        for (uint i = 0; i < v.length; i++) {
            address signer = ecrecover(messageHash, v[i], r[i], s[i]);
            if (isValidator[signer]) {
                validCount++;
            }
        }
        
        return validCount >= MIN_VALIDATORS;
    }
    
    /**
     * @notice 计算手续费
     */
    function _calculateFee(uint256 amount) internal pure returns (uint256) {
        uint256 percentageFee = (amount * FEE_RATE) / 10000;
        return percentageFee > BASE_FEE ? percentageFee : BASE_FEE;
    }
}
```

#### 合约5：HTLC Atomic Swap（原子交换合约）

```solidity
contract HTLCAtomicSwap {
    
    // ============ 数据结构 ============
    
    struct Swap {
        bytes32 swapId;
        address partyA;            // Qylith侧
        address partyB;            // 目标链侧
        address tokenA;
        address tokenB;
        uint256 amountA;
        uint256 amountB;
        bytes32 hashLock;
        uint256 timelock;
        bool isCompleted;
        bytes secret;              // 后量子加密的密钥
    }
    
    mapping(bytes32 => Swap) public swaps;
    mapping(bytes32 => bytes32) public secretHashes; // 哈希锁 -> swapId
    
    // ============ 事件 ============
    
    event SwapInitiated(
        bytes32 indexed swapId,
        address indexed partyA,
        bytes32 hashLock,
        uint256 timelock
    );
    
    event SwapCompleted(
        bytes32 indexed swapId,
        bytes secret
    );
    
    event SwapRefunded(
        bytes32 indexed swapId
    );
    
    // ============ 核心函数 ============
    
    /**
     * @notice 发起原子交换（Qylith侧）
     */
    function initiateSwap(
        address partyB,
        address tokenA,
        address tokenB,
        uint256 amountA,
        uint256 amountB,
        bytes32 hashLock,
        uint256 timelockDuration
    ) external returns (bytes32 swapId) {
        require(timelockDuration > 0 && timelockDuration <= 7 days, "Invalid timelock");
        
        swapId = keccak256(abi.encode(
            msg.sender, partyB, tokenA, tokenB, amountA, amountB, 
            hashLock, block.timestamp
        ));
        
        // 锁定PartyA的资产
        require(IERC20(tokenA).transferFrom(msg.sender, address(this), amountA),
            "Transfer failed");
        
        swaps[swapId] = Swap({
            swapId: swapId,
            partyA: msg.sender,
            partyB: partyB,
            tokenA: tokenA,
            tokenB: tokenB,
            amountA: amountA,
            amountB: amountB,
            hashLock: hashLock,
            timelock: block.timestamp + timelockDuration,
            isCompleted: false,
            secret: bytes("")
        });
        
        secretHashes[hashLock] = swapId;
        
        emit SwapInitiated(swapId, msg.sender, hashLock, swaps[swapId].timelock);
    }
    
    /**
     * @notice 完成原子交换（提供密钥）
     * @param _swapId Swap ID
     * @param secret 用PQC加密的密钥
     */
    function completeSwap(bytes32 _swapId, bytes memory secret) external {
        Swap storage swap = swaps[_swapId];
        require(!swap.isCompleted, "Already completed");
        require(block.timestamp <= swap.timelock, "Expired");
        
        // 验证哈希锁
        bytes32 computedHash = keccak256(abi.encodePacked(secret));
        require(computedHash == swap.hashLock, "Invalid secret");
        
        // PQC密钥验证（确保密钥未被篡改）
        require(_verifyPQCKey(swapId, secret), "PQC verification failed");
        
        // 解锁资产
        swap.isCompleted = true;
        swap.secret = secret;
        
        // PartyA获得tokenB（通过跨链桥）
        // PartyB获得tokenA
        require(IERC20(swap.tokenA).transfer(swap.partyB, swap.amountA),
            "Transfer failed");
        
        emit SwapCompleted(_swapId, secret);
    }
    
    /**
     * @notice 超时退款
     */
    function refundSwap(bytes32 _swapId) external {
        Swap storage swap = swaps[_swapId];
        require(!swap.isCompleted, "Already completed");
        require(block.timestamp > swap.timelock, "Not expired");
        require(msg.sender == swap.partyA, "Not partyA");
        
        swap.isCompleted = true;
        
        require(IERC20(swap.tokenA).transfer(swap.partyA, swap.amountA),
            "Refund failed");
        
        emit SwapRefunded(_swapId);
    }
    
    /**
     * @notice PQC密钥验证
     */
    function _verifyPQCKey(bytes32 swapId, bytes memory secret) 
        internal view returns (bool) {
        // 与QuantumBridge相同的PQC验证逻辑
        bytes32 keyHash = keccak256(secret);
        return _storedKeyHash[swapId] == keyHash;
    }
}
```

### 2.3 混合签名验证机制

**跨链安全性模型**：

```
源链 (Ethereum/Solana/TRON)          Qylith链
         |                                    |
         |  ECDSA签名                         |  ML-DSA签名
         |  (传统安全)                         |  (抗量子安全)
         v                                    v
    +------------+                      +------------+
    | Validator  |  ---- HTLC ---->    | Quantum    |
    | (ECDSA多签)|                      | Bridge     |
    +------------+                      +------------+
                                               |
                                               | PQC密钥封装
                                               v
                                         +------------+
                                         | ML-KEM-768 |
                                         | 解密验证   |
                                         +------------+
```

**为什么这样设计？**

1. **源链兼容性**：ECDSA是所有主流链的标准，确保跨链消息能被正确解析
2. **Qylith安全性**：Qylith侧强制ML-DSA签名，防止量子计算机伪造Qylith侧的交易
3. **PQC密钥封装**：即使源链ECDSA被破解，攻击者也需要同时破解ML-KEM-768才能完成跨链攻击
4. **HTLC时间锁**：即使所有签名被伪造，在时间锁期间内仍有反应窗口

### 2.4 与AEM模块的集成

```solidity
// 跨链AI推理验证（使用AEM模块）
contract CrossChainAIOracle {
    
    IAEMModule public immutable AEM;
    
    /**
     * @notice 跨链AI推理请求
     * @dev 用于将AI推理任务从其他链发送到Qylith执行
     */
    function requestAIInference(
        uint8 sourceChain,
        bytes32 sourceTxHash,
        bytes calldata modelId,
        bytes calldata inputData,
        bytes memory mlDsaProof
    ) external returns (bytes32 requestId) {
        // 1. 验证源链的ECDSA签名
        require(verifySourceChainSignature(sourceChain, sourceTxHash), 
            "Invalid source signature");
        
        // 2. 验证ML-DSA身份证明
        require(_verifyMLDSAIdentity(mlDsaProof), "Invalid ML-DSA proof");
        
        // 3. 在Qylith上执行AI推理
        requestId = keccak256(abi.encode(sourceChain, sourceTxHash, block.timestamp));
        
        bytes memory inferenceResult = AEM.executeAIAgent(
            bytes32(0), // AI推理任务
            abi.encode(modelId, inputData)
        );
        
        // 4. 返回带ML-DSA签名的推理结果
        bytes memory signedResult = abi.encode(
            requestId,
            inferenceResult,
            AEM.getAgentModelHash(bytes32(0)),
            _signWithMLDSA(inferenceResult)
        );
        
        emit AIInferenceCompleted(requestId, inferenceResult);
    }
}
```

### 2.5 代币经济模型

| 用途 | 比例 | 说明 |
|------|------|------|
| 跨链转账手续费 | 0.5% | 每笔转账金额的0.5% |
| 固定Gas费 | 0.001 ETH | 防止DDoS |
| PQC验证费 | 0.0001 ETH | PQC运算gas补贴 |
| 验证者奖励 | 70%手续费 | 分给跨链验证节点 |
| 燃烧机制 | 30%手续费 | 每季度燃烧直到总量50% |

**价值捕获逻辑**：
- 每笔跨链转账都产生手续费
- TVL越高，跨链需求越大，手续费收入越高
- $QYLITH代币质押可以成为跨链桥验证节点
- 长期看，量子威胁认知度提升会持续推高桥的TVL

### 2.6 MVP范围（黑客松可交付）

**Phase 1 (HTX Hackathon)**：
- [x] Qylith ↔ Ethereum 的单链跨链演示
- [x] ML-DSA签名验证合约
- [x] HTLC原子交换基础功能
- [x] PQC密钥封装演示
- [x] Demo：展示"传统跨链桥 vs QuantumShield"的安全性对比

**Phase 2**：
- [ ] Qylith ↔ Solana 跨链
- [ ] 多链聚合合约
- [ ] 验证者质押系统

**Phase 3**：
- [ ] 全链支持
- [ ] 去中心化验证者网络
- [ ] 闪电贷防护

### 2. 7竞品对比

| 维度 | QuantumShield | Wormhole | Stargate | LayerZero |
|------|---------------|----------|----------|-----------|
| 抗量子签名 | ML-DSA ✅ | 无 | 无 | 无 |
| PQC密钥封装 | ML-KEM ✅ | 无 | 无 | 无 |
| HTLC原子交换 | 原生 ✅ | 部分 | 部分 | 无 |
| AI预言机集成 | AEM ✅ | 无 | 无 | 无 |
| 混合安全模型 | 是 ✅ | 否 | 否 | 否 |
| 开发成熟度 | 早期 | 成熟 | 成熟 | 成熟 |

**核心差异化**：QuantumShield是目前唯一在协议层实现端到端抗量子保护的跨链桥。对于机构用户和长期持有者，这是唯一能够保护资产免受"先收割后量子攻击"（harvest now, decrypt later）的方案。

---

## 应用三：量子安全DeFi套件

### 3.1 产品定义与愿景

**产品定位**：首个端到端抗量子DeFi协议栈，包括AMMDEX、借贷市场和衍生品交易所，所有合约签名均使用ML-DSA-87，永久抵抗量子计算机攻击。

**核心价值主张**：
- **长期DeFi安全**：用户的LP头寸、借贷抵押品、衍生品仓位均受ML-DSA保护
- **量子保险库**：针对长期hodler的专属产品，锁仓资产享受PQC加密保护
- **AI增强的风险管理**：结合AEM模块的AI风险评估，降低清算风险

**用户故事**：
```
作为DeFi老用户Carol：
我在QuantumSwap提供ETH/USDC流动性，
LP代币用我的ML-DSA密钥签名，即使量子计算机出现，我的头寸仍然安全。
同时我可以在QuantumLend用ETH作为抵押借出USDC，
AI风控系统根据我的链上历史自动评估贷款额度。
我还可以将部分仓位锁入量子保险库，享受额外的年化收益。
```

### 3.2 核心智能合约架构

#### 合约6：QuantumSwap（抗量子AMM）

```solidity
contract QuantumSwap {
    
    // ============ 数据结构 ============
    
    struct Pair {
        address token0;
        address token1;
        uint256 reserve0;
        uint256 reserve1;
        uint256 totalLiquidity;
        bytes32 pairId;
    }
    
    struct LiquidityPosition {
        uint256 liquidity;
        uint256 balance0;
        uint256 balance1;
        uint256 lastUpdate;
        bytes mlDsaProof;          // ML-DSA签名证明
    }
    
    struct SwapRequest {
        bytes32 requestId;
        address sender;
        address tokenIn;
        address tokenOut;
        uint256 amountIn;
        uint256 amountOut;
        uint256 minAmountOut;
        uint256 deadline;
        bytes mlDsaSignature;       // 用户签名（抗量子）
        uint256 timestamp;
    }
    
    // ============ 状态变量 ============
    
    mapping(bytes32 => Pair) public pairs;
    mapping(address => mapping(bytes32 => LiquidityPosition)) public positions;
    mapping(bytes32 => SwapRequest) public swapRequests;
    
    uint256 public constant FEE_DENOMINATOR = 10000;
    uint256 public constant SWAP_FEE = 30;      // 0.3%
    uint256 public constant PROTOCOL_FEE = 5000; // 协议收取50%的swap fee
    
    // ============ 事件 ============
    
    event PairCreated(bytes32 indexed pairId, address token0, address token1);
    event LiquidityAdded(bytes32 indexed pairId, uint256 liquidity, uint256 amount0, uint256 amount1);
    event LiquidityRemoved(bytes32 indexed pairId, uint256 liquidity, uint256 amount0, uint256 amount1);
    event SwapExecuted(bytes32 indexed requestId, uint256 amountIn, uint256 amountOut);
    
    // ============ 核心函数 ============
    
    /**
     * @notice 创建交易对
     */
    function createPair(address tokenA, address tokenB) 
        external returns (bytes32 pairId) {
        require(tokenA != tokenB, "Identical addresses");
        
        (address token0, address token1) = tokenA < tokenB 
            ? (tokenA, tokenB) 
            : (tokenB, tokenA);
        
        pairId = keccak256(abi.encode(token0, token1));
        
        pairs[pairId] = Pair({
            token0: token0,
            token1: token1,
            reserve0: 0,
            reserve1: 0,
            totalLiquidity: 0,
            pairId: pairId
        });
        
        emit PairCreated(pairId, token0, token1);
    }
    
    /**
     * @notice 添加流动性
     * @param pairId 交易对ID
     * @param amount0Desired token0数量
     * @param amount1Desired token1数量
     * @param mlDsaSignature ML-DSA签名证明（验证流动性归属）
     */
    function addLiquidity(
        bytes32 pairId,
        uint256 amount0Desired,
        uint256 amount1Desired,
        uint256 minAmount0,
        uint256 minAmount1,
        uint256 deadline,
        bytes calldata mlDsaSignature
    ) external returns (uint256 liquidity, uint256 amount0, uint256 amount1) {
        Pair storage pair = pairs[pairId];
        require(pair.pairId == pairId, "Pair not found");
        require(deadline >= block.timestamp, "Expired");
        
        // 验证ML-DSA签名（确保添加者身份）
        require(_verifyUserSignature(
            msg.sender, pairId, amount0Desired, amount1Desired, mlDsaSignature
        ), "Invalid signature");
        
        // 计算流动性代币数量
        if (pair.totalLiquidity == 0) {
            liquidity = sqrt(amount0Desired * amount1Desired);
        } else {
            liquidity = min(
                (amount0Desired * pair.totalLiquidity) / pair.reserve0,
                (amount1Desired * pair.totalLiquidity) / pair.reserve1
            );
        }
        require(liquidity > 0, "Insufficient liquidity");
        
        // 转账代币
        require(IERC20(pair.token0).transferFrom(msg.sender, address(this), amount0Desired),
            "Transfer0 failed");
        require(IERC20(pair.token1).transferFrom(msg.sender, address(this), amount1Desired),
            "Transfer1 failed");
        
        // 更新储备
        pair.reserve0 += amount0Desired;
        pair.reserve1 += amount1Desired;
        pair.totalLiquidity += liquidity;
        
        // 记录头寸
        LiquidityPosition storage position = positions[msg.sender][pairId];
        position.liquidity += liquidity;
        position.lastUpdate = block.timestamp;
        position.mlDsaProof = mlDsaSignature;
        
        emit LiquidityAdded(pairId, liquidity, amount0Desired, amount1Desired);
        
        return (liquidity, amount0Desired, amount1Desired);
    }
    
    /**
     * @notice 移除流动性
     */
    function removeLiquidity(
        bytes32 pairId,
        uint256 liquidity,
        uint256 minAmount0,
        uint256 minAmount1,
        uint256 deadline,
        bytes calldata mlDsaSignature
    ) external returns (uint256 amount0, uint256 amount1) {
        Pair storage pair = pairs[pairId];
        LiquidityPosition storage position = positions[msg.sender][pairId];
        
        require(position.liquidity >= liquidity, "Insufficient liquidity");
        require(deadline >= block.timestamp, "Expired");
        
        // 验证ML-DSA签名
        require(_verifyUserSignature(
            msg.sender, pairId, liquidity, 0, mlDsaSignature
        ), "Invalid signature");
        
        // 计算提取数量
        amount0 = (pair.reserve0 * liquidity) / pair.totalLiquidity;
        amount1 = (pair.reserve1 * liquidity) / pair.totalLiquidity;
        
        require(amount0 >= minAmount0 && amount1 >= minAmount1, "Slippage");
        
        // 更新状态
        pair.reserve0 -= amount0;
        pair.reserve1 -= amount1;
        pair.totalLiquidity -= liquidity;
        position.liquidity -= liquidity;
        
        // 转账
        require(IERC20(pair.token0).transfer(msg.sender, amount0), "Transfer0 failed");
        require(IERC20(pair.token1).transfer(msg.sender, amount1), "Transfer1 failed");
        
        emit LiquidityRemoved(pairId, liquidity, amount0, amount1);
    }
    
    /**
     * @notice 执行兑换（抗量子签名）
     */
    function swap(
        bytes32 pairId,
        uint256 amountIn,
        uint256 minAmountOut,
        address to,
        uint256 deadline,
        bytes calldata mlDsaSignature
    ) external returns (uint256 amountOut) {
        Pair storage pair = pairs[pairId];
        require(deadline >= block.timestamp, "Expired");
        
        // 验证ML-DSA签名
        require(_verifySwapSignature(
            msg.sender, pairId, amountIn, minAmountOut, to, deadline, mlDsaSignature
        ), "Invalid signature");
        
        // 计算输出金额（constant product formula）
        uint256 amountInWithFee = amountIn * (FEE_DENOMINATOR - SWAP_FEE);
        
        if (msg.sender == pair.token0) {
            // token0 -> token1
            amountOut = (pair.reserve1 * amountInWithFee) / 
                (pair.reserve0 * FEE_DENOMINATOR + amountInWithFee);
            
            require(amountOut >= minAmountOut, "Insufficient output");
            
            require(IERC20(pair.token0).transferFrom(msg.sender, address(this), amountIn),
                "Transfer failed");
            require(IERC20(pair.token1).transfer(to, amountOut), "Transfer failed");
            
            pair.reserve0 += amountIn;
            pair.reserve1 -= amountOut;
        } else {
            // token1 -> token0
            amountOut = (pair.reserve0 * amountInWithFee) / 
                (pair.reserve1 * FEE_DENOMINATOR + amountInWithFee);
            
            require(amountOut >= minAmountOut, "Insufficient output");
            
            require(IERC20(pair.token1).transferFrom(msg.sender, address(this), amountIn),
                "Transfer failed");
            require(IERC20(pair.token0).transfer(to, amountOut), "Transfer failed");
            
            pair.reserve1 += amountIn;
            pair.reserve0 -= amountOut;
        }
        
        emit SwapExecuted(
            keccak256(abi.encode(msg.sender, pairId, block.timestamp)),
            amountIn, amountOut
        );
    }
    
    // ============ ML-DSA验证 ============
    
    /**
     * @notice 验证用户签名
     */
    function _verifyUserSignature(
        address user,
        bytes32 pairId,
        uint256 amount0,
        uint256 amount1,
        bytes calldata signature
    ) internal view returns (bool) {
        bytes32 message = keccak256(abi.encode(
            "ADD_LIQUIDITY",
            user,
            pairId,
            amount0,
            amount1,
            block.chainid
        ));
        
        return IPQCModule(pqcModuleAddress).verifyMLDSA(
            signature,
            userPublicKeys[user], // 用户注册的ML-DSA公钥
            message
        );
    }
    
    /**
     * @notice 验证兑换签名
     */
    function _verifySwapSignature(
        address user,
        bytes32 pairId,
        uint256 amountIn,
        uint256 minAmountOut,
        address to,
        uint256 deadline,
        bytes calldata signature
    ) internal view returns (bool) {
        bytes32 message = keccak256(abi.encode(
            "SWAP",
            user,
            pairId,
            amountIn,
            minAmountOut,
            to,
            deadline,
            block.chainid
        ));
        
        return IPQCModule(pqcModuleAddress).verifyMLDSA(
            signature,
            userPublicKeys[user],
            message
        );
    }
}
```

#### 合约7：QuantumLend（抗量子借贷协议）

```solidity
contract QuantumLend {
    
    // ============ 数据结构 ============
    
    struct Market {
        address collateralToken;
        address debtToken;
        uint256 totalCollateral;
        uint256 totalDebt;
        uint256 collateralFactor;      // 抵押率 (e.g. 7500 = 75%)
        uint256 liquidationThreshold; // 清算阈值
        uint256 liquidationPenalty;    // 清算惩罚
        uint256 interestRateModel;
        uint256 supplyRate;
        uint256 borrowRate;
        bool isActive;
    }
    
    struct Account {
        address owner;
        uint256 collateralBalance;
        uint256 debtBalance;
        uint256 healthFactor;
        uint256 lastUpdate;
        bytes mlDsaPublicKey;          // 用户的ML-DSA公钥
    }
    
    struct BorrowRequest {
        bytes32 requestId;
        address borrower;
        uint256 collateralAmount;
        uint256 borrowAmount;
        uint256 interestRate;
        uint256 deadline;
        bytes mlDsaSignature;
        bytes aiRiskAssessment;       // AEM的AI风险评估结果
    }
    
    // ============ 状态变量 ============
    
    mapping(bytes32 => Market) public markets;
    mapping(bytes32 => mapping(address => Account)) public accounts;
    mapping(address => bytes32[]) public userMarketPositions;
    
    IAEModule public immutable AEM;
    address public riskOracle;        // AI风险评估预言机
    
    uint256 public constant LIQUIDATION_BONUS = 11000; // 清算收益10%
    uint256 public constant MIN_HEALTH_FACTOR = 1e18;   // 最低健康因子
    
    // ============ 事件 ============
    
    event MarketCreated(bytes32 indexed marketId, address collateral, address debt);
    event CollateralDeposited(bytes32 indexed marketId, address indexed user, uint256 amount);
    event BorrowExecuted(bytes32 indexed marketId, address indexed user, uint256 amount);
    event LiquidationExecuted(
        bytes32 indexed marketId, 
        address indexed liquidator, 
        address indexed borrower,
        uint256 collateralSeized,
        uint256 debtCovered
    );
    event HealthFactorUpdated(address indexed user, uint256 newHealthFactor);
    
    // ============ 核心函数 ============
    
    constructor(address _aemModule) {
        AEM = IAEMModule(_aemModule);
    }
    
    /**
     * @notice 创建借贷市场
     */
    function createMarket(
        address collateralToken,
        address debtToken,
        uint256 collateralFactor,
        uint256 liquidationThreshold,
        uint256 interestRateModel
    ) external returns (bytes32 marketId) {
        require(collateralToken != debtToken, "Same token");
        require(collateralFactor < 10000, "Invalid collateral factor");
        
        marketId = keccak256(abi.encode(collateralToken, debtToken));
        
        markets[marketId] = Market({
            collateralToken: collateralToken,
            debtToken: debtToken,
            totalCollateral: 0,
            totalDebt: 0,
            collateralFactor: collateralFactor,
            liquidationThreshold: liquidationThreshold,
            liquidationPenalty: LIQUIDATION_BONUS,
            interestRateModel: interestRateModel,
            supplyRate: 0,
            borrowRate: 0,
            isActive: true
        });
        
        emit MarketCreated(marketId, collateralToken, debtToken);
    }
    
    /**
     * @notice 存入抵押品
     */
    function depositCollateral(
        bytes32 marketId,
        uint256 amount,
        bytes calldata mlDsaSignature
    ) external {
        Market storage market = markets[marketId];
        require(market.isActive, "Market not active");
        
        Account storage account = accounts[marketId][msg.sender];
        
        // 验证签名（确保存款者身份）
        require(_verifyDepositSignature(
            msg.sender, marketId, amount, mlDsaSignature
        ), "Invalid signature");
        
        // 转账抵押品
        require(IERC20(market.collateralToken).transferFrom(
            msg.sender, address(this), amount
        ), "Transfer failed");
        
        // 更新账户
        account.collateralBalance += amount;
        account.owner = msg.sender;
        
        // 如果是新仓位，注册ML-DSA公钥
        if (account.mlDsaPublicKey.length == 0) {
            account.mlDsaPublicKey = _extractPublicKey(mlDsaSignature);
        }
        
        // 更新健康因子
        _updateHealthFactor(marketId, msg.sender);
        
        // 更新市场总量
        market.totalCollateral += amount;
        
        emit CollateralDeposited(marketId, msg.sender, amount);
    }
    
    /**
     * @notice 借款（集成AI风险评估）
     */
    function borrow(
        bytes32 marketId,
        uint256 collateralAmount,
        uint256 borrowAmount,
        uint256 deadline,
        bytes calldata mlDsaSignature
    ) external returns (uint256 interestRate) {
        Market storage market = markets[marketId];
        Account storage account = accounts[marketId][msg.sender];
        
        require(market.isActive, "Market not active");
        require(deadline >= block.timestamp, "Expired");
        
        // 1. AI风险评估（通过AEM模块）
        bytes memory riskAssessment = _performAIRiskAssessment(
            msg.sender, marketId, collateralAmount, borrowAmount
        );
        
        // 解析AI评估结果
        (uint256 aiRiskScore, bool isApproved, uint256 adjustedRate) = 
            abi.decode(riskAssessment, (uint256, bool, uint256));
        
        require(isApproved, "AI risk assessment rejected");
        
        // 2. 验证签名
        require(_verifyBorrowSignature(
            msg.sender, marketId, collateralAmount, borrowAmount, 
            deadline, mlDsaSignature, riskAssessment
        ), "Invalid signature");
        
        // 3. 转账抵押品
        require(IERC20(market.collateralToken).transferFrom(
            msg.sender, address(this), collateralAmount
        ), "Transfer failed");
        
        // 4. 计算借款金额上限（考虑抵押率）
        uint256 maxBorrow = (collateralAmount * market.collateralFactor) / 10000;
        require(borrowAmount <= maxBorrow, "Exceeds limit");
        
        // 5. 更新账户
        account.collateralBalance += collateralAmount;
        account.debtBalance += borrowAmount;
        
        // 6. 更新健康因子
        _updateHealthFactor(marketId, msg.sender);
        require(accounts[marketId][msg.sender].healthFactor >= MIN_HEALTH_FACTOR,
            "Health factor too low");
        
        // 7. 更新市场
        market.totalCollateral += collateralAmount;
        market.totalDebt += borrowAmount;
        market.borrowRate = adjustedRate;
        
        // 8. 转出借款
        require(IERC20(market.debtToken).transfer(msg.sender, borrowAmount),
            "Transfer failed");
        
        // 记录借款请求
        bytes32 requestId = keccak256(abi.encode(
            msg.sender, marketId, borrowAmount, block.timestamp
        ));
        
        emit BorrowExecuted(marketId, msg.sender, borrowAmount);
        
        return adjustedRate;
    }
    
    /**
     * @notice 执行清算
     */
    function liquidate(
        bytes32 marketId,
        address borrower,
        uint256 repayAmount,
        bytes calldata mlDsaSignature
    ) external {
        Market storage market = markets[marketId];
        Account storage borrowerAccount = accounts[marketId][borrower];
        
        require(borrowerAccount.healthFactor < MIN_HEALTH_FACTOR, 
            "Not liquidatable");
        
        // 验证清算者签名
        require(_verifyLiquidationSignature(
            msg.sender, marketId, borrower, repayAmount, mlDsaSignature
        ), "Invalid signature");
        
        // 计算可获得的抵押品
        uint256 collateralSeized = (repayAmount * market.liquidationPenalty) / 10000;
        
        // 更新借款者账户
        borrowerAccount.collateralBalance -= collateralSeized;
        borrowerAccount.debtBalance -= repayAmount;
        
        // 更新市场
        market.totalCollateral -= collateralSeized;
        market.totalDebt -= repayAmount;
        
        // 转账
        require(IERC20(market.debtToken).transferFrom(
            msg.sender, address(this), repayAmount
        ), "Repay transfer failed");
        require(IERC20(market.collateralToken).transfer(
            msg.sender, collateralSeized
        ), "Collateral transfer failed");
        
        // 更新健康因子
        _updateHealthFactor(marketId, borrower);
        
        emit LiquidationExecuted(marketId, msg.sender, borrower, 
            collateralSeized, repayAmount);
    }
    
    // ============ AI风险评估集成 ============
    
    /**
     * @notice 通过AEM执行AI风险评估
     */
    function _performAIRiskAssessment(
        address user,
        bytes32 marketId,
        uint256 collateralAmount,
        uint256 borrowAmount
    ) internal returns (bytes memory) {
        // 调用AEM模块执行AI推理
        bytes memory inputData = abi.encode(
            user,
            marketId,
            collateralAmount,
            borrowAmount,
            accounts[marketId][user].collateralBalance,
            accounts[marketId][user].debtBalance,
            block.timestamp
        );
        
        // AEM执行风险评估模型
        return AEM.executeAIAgent(
            RISK_ASSESSMENT_AGENT_ID,
            inputData
        );
    }
    
    // ============ 健康因子计算 ============
    
    /**
     * @notice 更新健康因子
     */
    function _updateHealthFactor(bytes32 marketId, address user) internal {
        Account storage account = accounts[marketId][user];
        Market storage market = markets[marketId];
        
        if (account.debtBalance == 0) {
            account.healthFactor = type(uint256).max;
        } else {
            // 健康因子 = (抵押品价值 * 清算阈值) / 债务价值
            uint256 collateralValue = account.collateralBalance;
            uint256 debtValue = account.debtBalance;
            
            account.healthFactor = (collateralValue * market.liquidationThreshold) 
                / debtValue;
        }
        
        account.lastUpdate = block.timestamp;
        
        emit HealthFactorUpdated(user, account.healthFactor);
    }
    
    // ============ 签名验证 ============
    
    function _verifyDepositSignature(
        address user, bytes32 marketId, uint256 amount, bytes calldata sig
    ) internal view returns (bool) {
        bytes32 message = keccak256(abi.encode("DEPOSIT", user, marketId, amount));
        return _verifyMLDSA(sig, accounts[marketId][user].mlDsaPublicKey, message);
    }
    
    function _verifyBorrowSignature(
        address user, bytes32 marketId, uint256 collateral, uint256 borrow,
        uint256 deadline, bytes calldata sig, bytes memory aiResult
    ) internal view returns (bool) {
        bytes32 message = keccak256(abi.encode(
            "BORROW", user, marketId, collateral, borrow, deadline, aiResult
        ));
        return _verifyMLDSA(sig, accounts[marketId][user].mlDsaPublicKey, message);
    }
}
```

#### 合约8：QuantumVault（量子保险库）

```solidity
contract QuantumVault {
    
    // ============ 数据结构 ============
    
    enum VaultType { Standard, Premium, Institutional }
    
    struct Vault {
        bytes32 vaultId;
        address owner;
        VaultType vaultType;
        address[] depositedTokens;
        uint256[] depositedAmounts;
        uint256 totalValue;
        uint256 lockDuration;
        uint256 startTime;
        uint256 endTime;
        uint256 apy;                  // 年化收益率
        uint256 quantumProtectionLevel; // 量子保护等级
        bytes encryptionKeyHash;      // PQC加密密钥的哈希
        bool isSealed;               // 是否已封存
    }
    
    struct VaultConfig {
        VaultType vaultType;
        uint256 minDeposit;
        uint256 maxDeposit;
        uint256 minLockDuration;
        uint256 maxLockDuration;
        uint256 baseAPY;
        uint256 quantumBonus;         // 量子保护加成
    }
    
    // ============ 状态变量 ============
    
    mapping(bytes32 => Vault) public vaults;
    mapping(VaultType => VaultConfig) public vaultConfigs;
    mapping(address => bytes32[]) public userVaults;
    
    uint256 public constant QUANTUM_PROTECTION_FEE = 100; // 1% 量子保护费
    
    // ============ 事件 ============
    
    event VaultCreated(
        bytes32 indexed vaultId, 
        address indexed owner, 
        VaultType vaultType
    );
    event VaultSealed(bytes32 indexed vaultId);
    event VaultUnsealed(bytes32 indexed vaultId, uint256 value);
    event YieldClaimed(bytes32 indexed vaultId, uint256 amount);
    
    // ============ 核心函数 ============
    
    /**
     * @notice 创建量子保险库
     */
    function createVault(
        VaultType vaultType,
        uint256 lockDuration,
        bytes calldata mlDsaSignature
    ) external returns (bytes32 vaultId) {
        VaultConfig memory config = vaultConfigs[vaultType];
        require(lockDuration >= config.minLockDuration, "Lock too short");
        require(lockDuration <= config.maxLockDuration, "Lock too long");
        
        // 验证签名
        require(_verifyVaultCreation(msg.sender, vaultType, lockDuration, mlDsaSignature),
            "Invalid signature");
        
        // 生成PQC加密密钥
        (bytes memory encryptionKey, bytes memory encapsulatedKey) = 
            IPQCModule(pqcModuleAddress).kemEncapsulate(
                bytes32ToBytes(uint256(uint160(msg.sender)))
            );
        
        vaultId = keccak256(abi.encode(
            msg.sender, vaultType, lockDuration, block.timestamp
        ));
        
        vaults[vaultId] = Vault({
            vaultId: vaultId,
            owner: msg.sender,
            vaultType: vaultType,
            depositedTokens: new address[](0),
            depositedAmounts: new uint256[](0),
            totalValue: 0,
            lockDuration: lockDuration,
            startTime: block.timestamp,
            endTime: block.timestamp + lockDuration,
            apy: config.baseAPY + config.quantumBonus,
            quantumProtectionLevel: _getProtectionLevel(vaultType),
            encryptionKeyHash: keccak256(encryptionKey),
            isSealed: false
        });
        
        userVaults[msg.sender].push(vaultId);
        
        emit VaultCreated(vaultId, msg.sender, vaultType);
    }
    
    /**
     * @notice 存入资产（封存后不可存入）
     */
    function deposit(
        bytes32 vaultId,
        address token,
        uint256 amount,
        bytes calldata mlDsaSignature
    ) external {
        Vault storage vault = vaults[vaultId];
        require(vault.owner == msg.sender, "Not owner");
        require(!vault.isSealed, "Vault sealed");
        require(block.timestamp < vault.endTime - 7 days, "Too late to deposit");
        
        // 验证签名
        require(_verifyDeposit(msg.sender, vaultId, token, amount, mlDsaSignature),
            "Invalid signature");
        
        // 转账代币
        require(IERC20(token).transferFrom(msg.sender, address(this), amount),
            "Transfer failed");
        
        // 更新保险库
        vault.depositedTokens.push(token);
        vault.depositedAmounts.push(amount);
        vault.totalValue += amount;
    }
    
    /**
     * @notice 封存保险库（量子保护生效）
     * @dev 封存后，资产进入PQC加密保护状态
     */
    function sealVault(bytes32 vaultId, bytes calldata mlDsaSignature) external {
        Vault storage vault = vaults[vaultId];
        require(vault.owner == msg.sender, "Not owner");
        require(!vault.isSealed, "Already sealed");
        
        // 验证签名
        require(_verifySeal(msg.sender, vaultId, mlDsaSignature),
            "Invalid signature");
        
        vault.isSealed = true;
        
        // PQC密钥现在正式生效
        emit VaultSealed(vaultId);
    }
    
    /**
     * @notice 解封保险库（锁定期结束后）
     */
    function unsealVault(bytes32 vaultId, bytes calldata pqcDecryptionKey) external {
        Vault storage vault = vaults[vaultId];
        require(vault.owner == msg.sender, "Not owner");
        require(vault.isSealed, "Not sealed");
        require(block.timestamp >= vault.endTime, "Lock not expired");
        
        // 验证PQC解密密钥
        require(keccak256(pqcDecryptionKey) == vault.encryptionKeyHash,
            "Invalid decryption key");
        
        vault.isSealed = false;
        
        // 计算并领取收益
        uint256 yield = _calculateYield(vault);
        
        emit VaultUnsealed(vaultId, vault.totalValue + yield);
    }
    
    /**
     * @notice 领取收益（无需解封）
     */
    function claimYield(bytes32 vaultId) external {
        Vault storage vault = vaults[vaultId];
        require(vault.owner == msg.sender, "Not owner");
        
        uint256 yield = _calculateYield(vault);
        require(yield > 0, "No yield");
        
        // 更新状态
        vault.totalValue += yield;
        
        // 转账收益（以第一个代币形式）
        require(IERC20(vault.depositedTokens[0]).transfer(msg.sender, yield),
            "Transfer failed");
        
        emit YieldClaimed(vaultId, yield);
    }
    
    // ============ 收益率计算 ============
    
    /**
     * @notice 计算收益
     */
    function _calculateYield(Vault storage vault) internal view returns (uint256) {
        uint256 timePassed = block.timestamp - vault.startTime;
        uint256 yearsPassed = timePassed / 365 days;
        
        // 复利计算
        // yield = principal * ((1 + apy)^years - 1)
        uint256 multiplier = (10000 + vault.apy) ** yearsPassed;
        uint256 totalWithYield = (vault.totalValue * multiplier) / 10000;
        
        return totalWithYield - vault.totalValue;
    }
    
    /**
     * @notice 获取量子保护等级
     */
    function _getProtectionLevel(VaultType vt) internal pure returns (uint256) {
        if (vt == VaultType.Standard) return 1;    // ML-DSA-44
        if (vt == VaultType.Premium) return 2;     // ML-DSA-65
        return 3;                                   // ML-DSA-87
    }
}
```

### 3.3 与AEM模块的集成

**DeFi + AI的深度整合**：

```solidity
// AI增强的风险管理合约
contract AIRiskManager {
    
    IAEMModule public immutable AEM;
    
    // 预定义的风险评估Agent ID
    bytes32 public constant COLLATERAL_AUCTION_AGENT = 
        bytes32(0x1234...); // 抵押品拍卖Agent
    bytes32 public constant LIQUIDATION_AGENT = 
        bytes32(0x5678...); // 清算Agent
    bytes32 public constant PORTFOLIO_OPTIMIZER = 
        bytes32(0x9abc...); // 组合优化Agent
    
    /**
     * @notice AI驱动的清算触发
     */
    function triggerLiquidation(
        bytes32 marketId,
        address borrower
    ) external returns (bool shouldLiquidate, bytes memory optimalStrategy) {
        // 调用AI Agent分析最佳清算策略
        bytes memory inputData = abi.encode(
            marketId,
            borrower,
            block.timestamp
        );
        
        // AEM执行清算优化Agent
        bytes memory aiOutput = AEM.executeAIAgent(
            LIQUIDATION_AGENT,
            inputData
        );
        
        return abi.decode(aiOutput, (bool, bytes));
    }
    
    /**
     * @notice AI驱动的收益率优化
     */
    function optimizeYield(
        bytes32 vaultId
    ) external returns (uint256 optimalAPY, address[] memory recommendations) {
        Vault memory vault = vaults[vaultId];
        
        bytes memory inputData = abi.encode(
            vault.depositedTokens,
            vault.depositedAmounts,
            vault.totalValue,
            block.timestamp
        );
        
        bytes memory aiOutput = AEM.executeAIAgent(
            PORTFOLIO_OPTIMIZER,
            inputData
        );
        
        return abi.decode(aiOutput, (uint256, address[]));
    }
}
```

### 3.4 代币经济模型

#### $QOR 在DeFi套件中的用途

| 用途 | 比例 | 说明 |
|------|------|------|
| 流动性激励 | 40% | LP挖矿奖励 |
| 借款抵押品 | 20% | $QOR作为借贷抵押品 |
| 治理投票 | 15% | 协议参数治理 |
| 量子保险库优先权 | 10% | Premium/Institutional vault访问权 |
| 协议回购 | 15% | 手续费回购销毁 |

#### 量子保险库收益率加成

| 保险库类型 | 基础APY | 量子保护加成 | 锁定期 |
|-----------|--------|--------------|--------|
| Standard | 5% | +1% | 30天 |
| Premium | 8% | +2% | 90天 |
| Institutional | 12% | +3% | 180天 |

### 3.5 MVP范围（黑客松可交付）

**Phase 1 (HTX Hackathon)**：
- [x] QuantumSwap基础AMM合约
- [x] ML-DSA签名验证
- [x] 基础LP头寸管理
- [x] 前端Demo：AMM交换界面

**Phase 2**：
- [ ] QuantumLend借贷合约
- [ ] AI风险评估集成
- [ ] 清算机制

**Phase 3**：
- [ ] QuantumVault保险库
- [ ] 衍生品交易所
- [ ] 完整DeFi生态

### 3.6 竞品对比

| 维度 | QuantumSwap | Uniswap V3 | SushiSwap | Curve |
|------|-------------|------------|-----------|-------|
| 抗量子签名 | ML-DSA ✅ | 无 | 无 | 无 |
| LP头寸保护 | 是 ✅ | 否 | 否 | 否 |
| AI风险评估 | AEM集成 ✅ | 无 | 无 | 无 |
| 原生质押收益 | 是 ✅ | 否 | 否 | 否 |

| 维度 | QuantumLend | Aave V3 | Compound | MakerDAO |
|------|-------------|---------|----------|----------|
| 抗量子抵押 | ML-DSA ✅ | 无 | 无 | 无 |
| AI风控 | AEM集成 ✅ | 无 | 无 | 无 |
| 即时清算 | 是 ✅ | 否 | 否 | 否 |

**核心差异化**：
1. **签名层安全**：所有用户操作均使用ML-DSA签名，防止量子计算机伪造交易
2. **LP头寸保护**：Uniswap等AMM的LP头寸在量子时代面临被窃取风险，QuantumSwap没有
3. **AI增强风控**：借贷清算不再依赖简单的健康因子，而是AI综合评估

---

## 技术整合：三大应用协同

### 跨应用互操作

```
┌─────────────────────────────────────────────────────────────────┐
│                        Qylith 链                                  │
│                                                                   │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐        │
│  │ QylithAI Hub │   │ QuantumShield│   │ QuantumDeFi  │        │
│  │              │   │   Bridge     │   │              │        │
│  │ - Agent注册   │◄──│ - 跨链Agent  │◄──│ - DeFi收益   │        │
│  │ - 服务调用   │   │   调度       │   │   支付       │        │
│  │ - 声誉系统   │   │ - PQC密钥   │   │ - 保险库     │        │
│  └──────────────┘   │   传递       │   │   保护       │        │
│         │          └──────────────┘   └──────────────┘        │
│         │                 │                    │                │
│         └─────────────────┴────────────────────┘                │
│                           │                                      │
│                    ┌──────────────┐                              │
│                    │  AEM Module   │                              │
│                    │  AI执行引擎   │                              │
│                    │  ML-DSA验证   │                              │
│                    └──────────────┘                              │
│                           │                                      │
│                    ┌──────────────┐                              │
│                    │ ML-KEM-768   │                              │
│                    │ PQC密钥封装   │                              │
│                    └──────────────┘                              │
└─────────────────────────────────────────────────────────────────┘
```

### 价值流转

1. **AI Agent Hub** → 产生AI推理需求 → 消耗$QOR作为Gas
2. **QuantumBridge** → 吸引跨链TVL → 产生手续费收入
3. **QuantumDeFi** → 提供流动性 → 增加链上资产沉淀
4. **三者互相增强**：DeFi收益可以支付Agent服务费，跨链桥连接其他链的Agent和资产

---

## 与Aura项目的协同

**Aura项目定位**：AI Agent + Web3基础设施

**Qylith + Aura协同点**：

| Aura能力 | Qylith能力 | 协同价值 |
|---------|-----------|---------|
| AI Agent开发框架 | AEM原生执行 | Agent开发成本降低80% |
| Agent身份协议 | ML-DSA身份证明 | 链上Agent身份不可伪造 |
| 跨平台Agent | 跨链桥 | Agent可服务全链用户 |
| AI服务市场 | DeFi收益分成 | Agent可持有链上资产并理财 |

**具体集成**：
- Aura的AI Agent可以注册到QylithAI Hub
- Aura的Agent身份协议可以用Qylith的ML-DSA签名强化
- Aura的跨链需求可以通过QuantumBridge安全实现
- Aura用户的闲置资产可以存入QuantumVault获得保护

---

*文档版本：v1.0*
*作者：Qylith生态设计团队*
*最后更新：2024年*
