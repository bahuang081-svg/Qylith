import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import * as dotenv from "dotenv";

// 加载环境变量
dotenv.config();

const SEPOLIA_RPC_URL = process.env.SEPOLIA_RPC_URL || "https://sepolia.infura.io/v3/YOUR_INFURA_KEY";
const PRIVATE_KEY = process.env.PRIVATE_KEY || "0x0000000000000000000000000000000000000000000000000000000000000000";
const ETHERSCAN_API_KEY = process.env.ETHERSCAN_API_KEY || "";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  
  networks: {
    // 本地开发网络
    hardhat: {
      chainId: 31337,
      accounts: {
        mnemonic: "quantum shield bridge hackathon test mnemonic phrase"
      }
    },
    
    // Sepolia测试网（用于演示）
    sepolia: {
      url: SEPOLIA_RPC_URL,
      accounts: [PRIVATE_KEY],
      chainId: 11155111
    },
    
    // 主网
    mainnet: {
      url: process.env.MAINNET_RPC_URL || "",
      accounts: [PRIVATE_KEY],
      chainId: 1
    }
  },
  
  // etherscan验证配置
  etherscan: {
    apiKey: {
      sepolia: ETHERSCAN_API_KEY,
      mainnet: ETHERSCAN_API_KEY
    }
  },
  
  // 气体报告配置
  gasReporter: {
    enabled: process.env.REPORT_GAS === "true",
    currency: "USD",
    coinmarketcap: process.env.COINMARKETCAP_API_KEY
  },
  
  // 路径配置
  paths: {
    sources: "./contracts",
    tests: "./test",
    cache: "./cache",
    artifacts: "./artifacts"
  },
  
  // TypeScript配置
  namedAccounts: {
    deployer: {
      default: 0,
      sepolia: 0
    },
    relayer: {
      default: 1,
      sepolia: 1
    }
  }
};

export default config;
