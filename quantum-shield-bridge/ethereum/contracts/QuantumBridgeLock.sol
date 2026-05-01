// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title QuantumBridgeLock
 * @dev Ethereum端HTLC锁定合约，支持ETH和ERC20代币
 * @notice 这是QuantumShield Bridge的核心合约之一
 * 
 * 核心功能：
 * 1. 锁定用户资产（ETH/ERC20）
 * 2. 生成HTLC（哈希时间锁合约）
 * 3. 支持跨链证明验证
 * 4. 超时自动退款
 * 
 * 安全机制：
 * - 重放攻击防护：使用chainId + nonce
 * - 时间锁：防止恶意锁定
 * - 事件追踪：完整的审计日志
 * 
 * @author QuantumShield Team
 * @notice For HTX Genesis Hackathon Demo
 */
contract QuantumBridgeLock {
    // ==================== 类型定义 ====================
    
    /// @notice 锁定资产的结构
    struct Lock {
        address sender;           // 发送者地址
        address recipient;        // Qylith目标地址（编码格式）
        address token;            // 代币地址，0x0表示ETH
        uint256 amount;           // 锁定金额
        bytes32 hashlock;         // 哈希锁（防止提前释放）
        uint256 timelock;         // 时间锁（超时时间戳）
        bool claimed;              // 是否已被领取
        bool refunded;            // 是否已退款
        bytes32 secret;           // 密钥（领取后可见）
        uint256 chainId;          // 链ID（防重放）
        uint256 nonce;            // 随机数（防重放）
    }
    
    /// @notice 支持的代币类型
    enum TokenType {
        ETH,
        ERC20
    }
    
    // ==================== 状态变量 ====================
    
    /// @notice 锁定记录映射 (lockId => Lock)
    mapping(bytes32 => Lock) public locks;
    
    /// @notice 地址对应的nonce计数（防重放）
    mapping(address => uint256) public nonces;
    
    /// @notice 合约所有者（用于管理）
    address public owner;
    
    /// @notice Relayer地址（有权提交证明）
    address public relayer;
    
    /// @notice 锁定超时时间（秒），默认24小时
    uint256 public lockTimeout = 24 hours;
    
    /// @notice 最小锁定金额
    uint256 public minLockAmount = 0.001 ether;
    
    /// @notice 锁定ID列表（用于遍历）
    bytes32[] public lockIds;
    
    // ==================== 事件 ====================
    
    /**
     * @dev 资产锁定事件 - Relayer会监听此事件
     * @param lockId 锁定ID（由合约生成）
     * @param sender 发送者地址
     * @param recipient Qylith目标地址
     * @param token 代币地址（0x0为ETH）
     * @param amount 锁定金额
     * @param hashlock 哈希锁
     * @param timelock 超时时间
     * @param chainId 链ID
     * @param nonce 随机数
     */
    event AssetLocked(
        bytes32 indexed lockId,
        address indexed sender,
        bytes indexed recipient,
        address token,
        uint256 amount,
        bytes32 hashlock,
        uint256 timelock,
        uint256 chainId,
        uint256 nonce
    );
    
    /**
     * @dev 资产领取事件 - 跨链完成标志
     * @param lockId 锁定ID
     * @param recipient 领取者
     * @param secret 使用的密钥
     * @param falconsig FALCON签名证明（Qylith侧验证用）
     */
    event AssetClaimed(
        bytes32 indexed lockId,
        address indexed recipient,
        bytes32 secret,
        bytes falconsig
    );
    
    /**
     * @dev 资产退款事件 - 超时或错误时触发
     * @param lockId 锁定ID
     * @param recipient 退款接收者
     * @param reason 退款原因
     */
    event AssetRefunded(
        bytes32 indexed lockId,
        address indexed recipient,
        string reason
    );
    
    /**
     * @dev Relayer地址变更事件
     */
    event RelayerUpdated(address oldRelayer, address newRelayer);
    
    /**
     * @dev 锁定超时时间更新事件
     */
    event TimeoutUpdated(uint256 oldTimeout, uint256 newTimeout);
    
    // ==================== 错误定义 ====================
    
    error InsufficientBalance();
    error AmountTooSmall();
    error InvalidTimelock();
    error LockNotFound();
    error LockAlreadyClaimed();
    error LockAlreadyRefunded();
    error LockNotExpired();
    error InvalidSecret();
    error InvalidHashlock();
    error OnlyRelayer();
    error TransferFailed();
    
    // ==================== 修饰符 ====================
    
    modifier onlyRelayer() {
        if (msg.sender != relayer) revert OnlyRelayer();
        _;
    }
    
    modifier onlyActiveLock(bytes32 lockId) {
        if (locks[lockId].sender == address(0)) revert LockNotFound();
        if (locks[lockId].claimed) revert LockAlreadyClaimed();
        if (locks[lockId].refunded) revert LockAlreadyRefunded();
        _;
    }
    
    // ==================== 构造函数 ====================
    
    constructor(address _relayer) {
        owner = msg.sender;
        relayer = _relayer;
    }
    
    // ==================== 核心函数：锁定ETH ====================
    
    /**
     * @dev 锁定ETH并创建HTLC
     * @param recipient Qylith目标地址（编码后的bytes）
     * @param hashlock 哈希锁（由接收方提供）
     * @param timelock 超时时间戳
     * 
     * 调用流程：
     * 1. 用户调用lockETH，传入Qylith地址和哈希锁
     * 2. 合约验证参数，创建Lock记录
     * 3. 合约接收ETH，触发AssetLocked事件
     * 4. Relayer监听事件，验证并提交到Qylith
     * 5. Qylith验证后，用户可在Qylith领取对应资产
     * 6. Relayer调用claim，携带FALCON签名证明
     */
    function lockETH(
        bytes calldata recipient,
        bytes32 hashlock,
        uint256 timelock
    ) external payable returns (bytes32 lockId) {
        // 参数验证
        if (msg.value < minLockAmount) revert AmountTooSmall();
        if (timelock <= block.timestamp) revert InvalidTimelock();
        if (hashlock == bytes32(0)) revert InvalidHashlock();
        
        // 生成唯一的lockId
        uint256 currentNonce = nonces[msg.sender]++;
        lockId = _generateLockId(
            msg.sender,
            recipient,
            address(0),
            msg.value,
            hashlock,
            block.timestamp + lockTimeout,
            block.chainid,
            currentNonce
        );
        
        // 创建锁定记录
        locks[lockId] = Lock({
            sender: msg.sender,
            recipient: recipient,
            token: address(0),
            amount: msg.value,
            hashlock: hashlock,
            timelock: timelock,
            claimed: false,
            refunded: false,
            secret: bytes32(0),
            chainId: block.chainid,
            nonce: currentNonce
        });
        
        lockIds.push(lockId);
        
        // 触发锁定事件 - Relayer会监听此事件
        emit AssetLocked(
            lockId,
            msg.sender,
            recipient,
            address(0),
            msg.value,
            hashlock,
            timelock,
            block.chainid,
            currentNonce
        );
    }
    
    // ==================== 核心函数：锁定ERC20 ====================
    
    /**
     * @dev 锁定ERC20代币
     * @param token ERC20代币地址
     * @param amount 锁定数量
     * @param recipient Qylith目标地址
     * @param hashlock 哈希锁
     * @param timelock 超时时间
     */
    function lockERC20(
        address token,
        uint256 amount,
        bytes calldata recipient,
        bytes32 hashlock,
        uint256 timelock
    ) external returns (bytes32 lockId) {
        // 参数验证
        if (amount < minLockAmount) revert AmountTooSmall();
        if (timelock <= block.timestamp) revert InvalidTimelock();
        if (hashlock == bytes32(0)) revert InvalidHashlock();
        if (token == address(0)) revert InvalidHashlock(); // ERC20不能用0地址
        
        // 从用户转当代币到合约
        IERC20 erc20 = IERC20(token);
        if (erc20.balanceOf(msg.sender) < amount) revert InsufficientBalance();
        if (erc20.allowance(msg.sender, address(this)) < amount) {
            revert InsufficientBalance();
        }
        
        // 使用safeTransferFrom确保安全
        bool success = _safeTransferFrom(token, msg.sender, address(this), amount);
        if (!success) revert TransferFailed();
        
        // 生成lockId
        uint256 currentNonce = nonces[msg.sender]++;
        lockId = _generateLockId(
            msg.sender,
            recipient,
            token,
            amount,
            hashlock,
            block.timestamp + lockTimeout,
            block.chainid,
            currentNonce
        );
        
        // 创建锁定记录
        locks[lockId] = Lock({
            sender: msg.sender,
            recipient: recipient,
            token: token,
            amount: amount,
            hashlock: hashlock,
            timelock: timelock,
            claimed: false,
            refunded: false,
            secret: bytes32(0),
            chainId: block.chainid,
            nonce: currentNonce
        });
        
        lockIds.push(lockId);
        
        // 触发事件
        emit AssetLocked(
            lockId,
            msg.sender,
            recipient,
            token,
            amount,
            hashlock,
            timelock,
            block.chainid,
            currentNonce
        );
    }
    
    // ==================== 核心函数：领取资产 ====================
    
    /**
     * @dev 领取资产（由Relayer调用）
     * @notice 只有Relayer可以调用，需要提供FALCON签名证明
     * @param lockId 锁定ID
     * @param secret 密钥（用于验证哈希锁）
     * @param falconsig FALCON签名（Qylith侧验证的证明）
     * 
     * 安全说明：
     * - Relayer必须验证Qylith链上已成功铸造包装资产
     * - FALCON签名证明了跨链操作的有效性
     * - 密钥验证确保只有知道secret的人能领取
     */
    function claim(
        bytes32 lockId,
        bytes32 secret,
        bytes calldata falconsig
    ) external onlyRelayer onlyActiveLock(lockId) {
        Lock storage lockInfo = locks[lockId];
        
        // 验证密钥（哈希锁验证）
        if (keccak256(abi.encodePacked(secret)) != lockInfo.hashlock) {
            revert InvalidSecret();
        }
        
        // 标记为已领取
        lockInfo.claimed = true;
        lockInfo.secret = secret;
        
        // 转账给发送者
        if (lockInfo.token == address(0)) {
            // 转ETH
            (bool success, ) = lockInfo.sender.call{value: lockInfo.amount}("");
            if (!success) revert TransferFailed();
        } else {
            // 转ERC20
            bool success = IERC20(lockInfo.token).transfer(
                lockInfo.sender,
                lockInfo.amount
            );
            if (!success) revert TransferFailed();
        }
        
        emit AssetClaimed(lockId, lockInfo.sender, secret, falconsig);
    }
    
    // ==================== 核心函数：退款 ====================
    
    /**
     * @dev 超时退款
     * @notice 任何人都可以触发超时退款（防止用户忘记）
     * @param lockId 锁定ID
     */
    function refund(bytes32 lockId) external onlyActiveLock(lockId) {
        Lock storage lockInfo = locks[lockId];
        
        // 验证是否超时
        if (block.timestamp < lockInfo.timelock) {
            revert LockNotExpired();
        }
        
        // 标记为已退款
        lockInfo.refunded = true;
        
        // 转账给发送者
        if (lockInfo.token == address(0)) {
            (bool success, ) = lockInfo.sender.call{value: lockInfo.amount}("");
            if (!success) revert TransferFailed();
        } else {
            bool success = IERC20(lockInfo.token).transfer(
                lockInfo.sender,
                lockInfo.amount
            );
            if (!success) revert TransferFailed();
        }
        
        emit AssetRefunded(lockId, lockInfo.sender, "TIMEOUT");
    }
    
    // ==================== 管理函数 ====================
    
    /**
     * @dev 更新Relayer地址
     */
    function updateRelayer(address newRelayer) external {
        require(msg.sender == owner, "Only owner");
        address oldRelayer = relayer;
        relayer = newRelayer;
        emit RelayerUpdated(oldRelayer, newRelayer);
    }
    
    /**
     * @dev 更新锁定超时时间
     */
    function updateLockTimeout(uint256 newTimeout) external {
        require(msg.sender == owner, "Only owner");
        uint256 oldTimeout = lockTimeout;
        lockTimeout = newTimeout;
        emit TimeoutUpdated(oldTimeout, newTimeout);
    }
    
    /**
     * @dev 查看锁定详情
     */
    function getLock(bytes32 lockId) external view returns (Lock memory) {
        return locks[lockId];
    }
    
    /**
     * @dev 获取锁定数量
     */
    function getLockCount() external view returns (uint256) {
        return lockIds.length;
    }
    
    // ==================== 内部函数 ====================
    
    /**
     * @dev 生成唯一的LockId
     * @notice 使用多个参数确保唯一性，防止碰撞攻击
     */
    function _generateLockId(
        address sender,
        bytes memory recipient,
        address token,
        uint256 amount,
        bytes32 hashlock,
        uint256 timelock,
        uint256 chainId,
        uint256 nonce
    ) internal view returns (bytes32) {
        return keccak256(abi.encodePacked(
            sender,
            recipient,
            token,
            amount,
            hashlock,
            timelock,
            chainId,
            nonce,
            address(this),
            block.number
        ));
    }
    
    /**
     * @dev 安全转账ERC20（防止代币陷阱）
     */
    function _safeTransferFrom(
        address token,
        address from,
        address to,
        uint256 amount
    ) internal returns (bool) {
        (bool success, bytes memory data) = token.call(
            abi.encodeWithSelector(
                IERC20.transferFrom.selector,
                from,
                to,
                amount
            )
        );
        return success && (data.length == 0 || abi.decode(data, (bool)));
    }
    
    // ==================== 接受ETH ====================
    
    receive() external payable {}
    
    /**
     * @dev 允许合约接收ETH（用于处理意外的ETH转账）
     */
    function depositETH() external payable {}
}

/**
 * @dev ERC20接口定义
 * @notice 只包含我们需要的函数，避免完整接口的复杂性
 */
interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);
    function transferFrom(address from, address to, uint256 amount) external returns (bool);
    function balanceOf(address account) external returns (uint256);
    function allowance(address owner, address spender) external returns (uint256);
}

/**
 * @title IFALCONVerifier
 * @dev FALCON签名验证接口（未来集成用）
 * @notice 目前由Relayer在链下验证，主网时可集成到Qylith链上验证
 */
interface IFALCONVerifier {
    /**
     * @dev 验证FALCON签名
     * @param message 签名的消息
     * @param signature FALCON签名
     * @param publickey FALCON公钥
     * @return 是否验证通过
     */
    function verify(
        bytes32 message,
        bytes calldata signature,
        bytes calldata publickey
    ) external view returns (bool);
}
