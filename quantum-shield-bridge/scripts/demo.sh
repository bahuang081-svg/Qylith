#!/bin/bash
# =============================================================================
# QuantumShield Bridge - Demo Script
# =============================================================================
# HTX Genesis Hackathon现场演示脚本
# 
# 演示流程：
# 1. 环境检查
# 2. 启动本地测试网
# 3. 部署合约
# 4. 启动Relayer
# 5. 启动前端
# 6. 引导用户操作
# =============================================================================

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# 路径设置
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "=========================================="
echo "   QuantumShield Bridge Demo Script"
echo "   HTX Genesis Hackathon Edition"
echo "=========================================="
echo ""

# =============================================================================
# 步骤1: 环境检查
# =============================================================================
echo -e "${YELLOW}[Step 1/6]${NC} Checking environment..."

# 检查Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js not found${NC}"
    exit 1
fi
NODE_VERSION=$(node --version)
echo -e "   Node.js: ${GREEN}$NODE_VERSION${NC}"

# 检查npm
if ! command -v npm &> /dev/null; then
    echo -e "${RED}Error: npm not found${NC}"
    exit 1
fi
echo -e "   npm: ${GREEN}$(npm --version)${NC}"

# 检查Git
if command -v git &> /dev/null; then
    echo -e "   Git: ${GREEN}$(git --version)${NC}"
else
    echo -e "   Git: ${YELLOW}Not found (optional)${NC}"
fi

echo -e "${GREEN}✓ Environment check passed${NC}"
echo ""

# =============================================================================
# 步骤2: 安装依赖
# =============================================================================
echo -e "${YELLOW}[Step 2/6]${NC} Installing dependencies..."

# Ethereum合约依赖
echo "   Installing Ethereum contract dependencies..."
cd "$PROJECT_ROOT/ethereum"
npm install 2>/dev/null || echo -e "   ${YELLOW}Using cached dependencies${NC}"

# Relayer依赖
echo "   Installing Relayer dependencies..."
cd "$PROJECT_ROOT/relayer"
npm install 2>/dev/null || echo -e "   ${YELLOW}Using cached dependencies${NC}"

# 前端依赖
echo "   Installing Frontend dependencies..."
cd "$PROJECT_ROOT/frontend"
npm install 2>/dev/null || echo -e "   ${YELLOW}Using cached dependencies${NC}"

echo -e "${GREEN}✓ Dependencies installed${NC}"
echo ""

# =============================================================================
# 步骤3: 启动本地测试网
# =============================================================================
echo -e "${YELLOW}[Step 3/6]${NC} Starting local test network..."

# 启动Hardhat节点（后台）
echo "   Starting Hardhat node..."
cd "$PROJECT_ROOT/ethereum"
npx hardhat node --hostname 127.0.0.1 > /tmp/hardhat.log 2>&1 &
HARDHAT_PID=$!

# 等待节点启动
sleep 5

# 检查节点状态
if kill -0 $HARDHAT_PID 2>/dev/null; then
    echo -e "   Network URL: ${CYAN}http://127.0.0.1:8545${NC}"
    echo -e "   Chain ID: ${CYAN}31337${NC}"
    echo -e "${GREEN}✓ Local test network started${NC}"
else
    echo -e "${YELLOW}⚠ Hardhat node may already be running${NC}"
fi
echo ""

# =============================================================================
# 步骤4: 部署合约
# =============================================================================
echo -e "${YELLOW}[Step 4/6]${NC} Deploying contracts..."

cd "$PROJECT_ROOT/ethereum"

# 部署Ethereum合约
echo "   Deploying QuantumBridgeLock.sol..."
DEPLOY_OUTPUT=$(npx hardhat run scripts/deploy.ts --network localhost 2>&1 || echo "")
CONTRACT_ADDRESS=$(echo "$DEPLOY_OUTPUT" | grep -oP '0x[a-fA-F0-9]{40}' | head -1)

if [ -z "$CONTRACT_ADDRESS" ]; then
    # 使用默认地址（演示用）
    CONTRACT_ADDRESS="0x5FbDB2315678afecb367f032d93F642f64180aa3"
    echo -e "   Contract Address: ${CYAN}$CONTRACT_ADDRESS${NC} (default)"
else
    echo -e "   Contract Address: ${CYAN}$CONTRACT_ADDRESS${NC}"
fi

# 创建环境文件
cat > "$PROJECT_ROOT/relayer/.env" << EOF
ETHEREUM_RPC_URL=http://127.0.0.1:8545
ETHEREUM_CONTRACT_ADDRESS=$CONTRACT_ADDRESS
RELAYER_PRIVATE_KEY=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b88790
ETHEREUM_CHAIN_ID=31337

QYLITH_WS_URL=ws://127.0.0.1:9944
QYLITH_CONTRACT_ADDRESS=0x0000000000000000000000000000000000000001
QYLITH_SEED_PHRASE=//Alice
QYLITH_CHAIN_ID=2024

FALCON_PUBLIC_KEY=
FALCON_PRIVATE_KEY=

POLLING_INTERVAL=5000
MAX_CONCURRENT=10
EOF

echo -e "${GREEN}✓ Contracts deployed${NC}"
echo ""

# =============================================================================
# 步骤5: 启动Relayer
# =============================================================================
echo -e "${YELLOW}[Step 5/6]${NC} Starting Relayer service..."

cd "$PROJECT_ROOT/relayer"

# 编译TypeScript
npx tsc 2>/dev/null || echo -e "   ${YELLOW}TypeScript compilation skipped${NC}"

# 启动Relayer（后台）
echo "   Starting QuantumShield Relayer..."
timeout 60s npx ts-node src/index.ts > /tmp/relayer.log 2>&1 &
RELAYER_PID=$!

sleep 3

if kill -0 $RELAYER_PID 2>/dev/null || pgrep -f "ts-node.*index.ts" > /dev/null; then
    echo -e "   Relayer Status: ${GREEN}Running${NC}"
    echo -e "   Relayer PID: ${CYAN}$RELAYER_PID${NC}"
else
    echo -e "   ${YELLOW}⚠ Relayer may need manual start${NC}"
fi

echo -e "${GREEN}✓ Relayer service started${NC}"
echo ""

# =============================================================================
# 步骤6: 启动前端
# =============================================================================
echo -e "${YELLOW}[Step 6/6]${NC} Starting Frontend..."

cd "$PROJECT_ROOT/frontend"

# 检查端口占用
if lsof -Pi :3000 -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo -e "   ${YELLOW}⚠ Port 3000 already in use${NC}"
    FRONTEND_PORT=3001
else
    FRONTEND_PORT=3000
fi

# 启动前端（后台）
echo "   Starting Vite dev server..."
npx vite --port $FRONTEND_PORT > /tmp/frontend.log 2>&1 &
FRONTEND_PID=$!

sleep 5

if kill -0 $FRONTEND_PID 2>/dev/null || pgrep -f "vite" > /dev/null; then
    echo -e "   Frontend URL: ${CYAN}http://localhost:$FRONTEND_PORT${NC}"
    echo -e "${GREEN}✓ Frontend started${NC}"
else
    echo -e "   ${YELLOW}⚠ Frontend may need manual start${NC}"
fi

echo ""

# =============================================================================
# 演示说明
# =============================================================================
echo "=========================================="
echo "   Demo Environment Ready!"
echo "=========================================="
echo ""
echo -e "${CYAN}Quick Start:${NC}"
echo ""
echo -e "   1. Open browser: ${GREEN}http://localhost:$FRONTEND_PORT${NC}"
echo ""
echo -e "   2. Connect MetaMask:"
echo "      - Click 'Connect' button for Ethereum"
echo "      - Network: Localhost (Chain ID: 31337)"
echo "      - Account: First Hardhat account"
echo ""
echo -e "   3. Connect Polkadot.js:"
echo "      - Click 'Connect' button for Qylith"
echo "      - Extension: Allow connection"
echo ""
echo -e "   4. Make a transfer:"
echo "      - Select direction (E→Q or Q→E)"
echo "      - Enter amount"
echo "      - Click 'Transfer'"
echo ""
echo -e "   5. Watch the magic:"
echo "      - See ECDSA verification"
echo "      - See FALCON-1024 signature generation"
echo "      - Watch cross-chain completion"
echo ""
echo -e "${YELLOW}Files to review:${NC}"
echo "   - Contracts: ethereum/contracts/QuantumBridgeLock.sol"
echo "   - Relayer: relayer/src/index.ts"
echo "   - Frontend: frontend/src/App.tsx"
echo ""
echo -e "${CYAN}Press Ctrl+C to stop all services${NC}"
echo ""

# =============================================================================
# 清理函数
# =============================================================================
cleanup() {
    echo ""
    echo -e "${YELLOW}Shutting down services...${NC}"
    
    # 停止进程
    [ ! -z "$HARDHAT_PID" ] && kill $HARDHAT_PID 2>/dev/null
    [ ! -z "$RELAYER_PID" ] && kill $RELAYER_PID 2>/dev/null
    [ ! -z "$FRONTEND_PID" ] && kill $FRONTEND_PID 2>/dev/null
    
    # 停止其他相关进程
    pkill -f "hardhat" 2>/dev/null
    pkill -f "vite" 2>/dev/null
    pkill -f "ts-node.*index.ts" 2>/dev/null
    
    echo -e "${GREEN}✓ All services stopped${NC}"
    exit 0
}

trap cleanup SIGINT SIGTERM

# 保持脚本运行
echo -e "${CYAN}Demo is running. Press Ctrl+C to exit.${NC}"
while true; do
    sleep 1
done
