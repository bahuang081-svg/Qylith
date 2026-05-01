/**
 * QuantumBridgeLock Contract TypeScript Interface
 * 
 * Auto-generated type definitions for the deployed contract
 */

import { ethers } from 'ethers';

// Event types
export interface AssetLockedEvent {
  lockId: string;
  sender: string;
  recipient: string;
  token: string;
  amount: ethers.BigNumber;
  hashlock: string;
  timelock: ethers.BigNumber;
  chainId: ethers.BigNumber;
  nonce: ethers.BigNumber;
}

export interface AssetClaimedEvent {
  lockId: string;
  recipient: string;
  secret: string;
  falconsig: string;
}

export interface AssetRefundedEvent {
  lockId: string;
  recipient: string;
  reason: string;
}

// Lock structure
export interface Lock {
  sender: string;
  recipient: string;
  token: string;
  amount: ethers.BigNumber;
  hashlock: string;
  timelock: ethers.BigNumber;
  claimed: boolean;
  refunded: boolean;
  secret: string;
  chainId: ethers.BigNumber;
  nonce: ethers.BigNumber;
}

// Contract interface
export interface QuantumBridgeLock extends ethers.BaseContract {
  // Read functions
  owner(): Promise<string>;
  relayer(): Promise<string>;
  lockTimeout(): Promise<ethers.BigNumber>;
  minLockAmount(): Promise<ethers.BigNumber>;
  
  getLock(lockId: string): Promise<Lock>;
  getLockCount(): Promise<ethers.BigNumber>;
  nonces(address: string): Promise<ethers.BigNumber>;
  
  // Write functions
  lockETH(
    recipient: string | ethers.BytesLike,
    hashlock: string | ethers.BytesLike,
    timelock: ethers.BigNumberish,
    overrides?: ethers.PayableOverrides & { from?: PromiseOrValue<string> }
  ): Promise<ethers.ContractTransaction>;
  
  lockERC20(
    token: string,
    amount: ethers.BigNumberish,
    recipient: string | ethers.BytesLike,
    hashlock: string | ethers.BytesLike,
    timelock: ethers.BigNumberish,
    overrides?: ethers.Overrides & { from?: PromiseOrValue<string> }
  ): Promise<ethers.ContractTransaction>;
  
  claim(
    lockId: string | ethers.BytesLike,
    secret: string | ethers.BytesLike,
    falconsig: string | ethers.BytesLike,
    overrides?: ethers.Overrides & { from?: PromiseOrValue<string> }
  ): Promise<ethers.ContractTransaction>;
  
  refund(
    lockId: string | ethers.BytesLike,
    overrides?: ethers.Overrides & { from?: PromiseOrValue<string> }
  ): Promise<ethers.ContractTransaction>;
  
  updateRelayer(
    newRelayer: string,
    overrides?: ethers.Overrides & { from?: PromiseOrValue<string> }
  ): Promise<ethers.ContractTransaction>;
  
  updateLockTimeout(
    newTimeout: ethers.BigNumberish,
    overrides?: ethers.Overrides & { from?: PromiseOrValue<string> }
  ): Promise<ethers.ContractTransaction>;
  
  // Event filters
  filters: {
    AssetLocked(
      sender?: PromiseOrValue<string> | null,
      recipient?: PromiseOrValue<string> | null
    ): ethers.Filter;
    AssetClaimed(
      lockId?: PromiseOrValue<string> | null,
      recipient?: PromiseOrValue<string> | null
    ): ethers.Filter;
    AssetRefunded(
      lockId?: PromiseOrValue<string> | null,
      recipient?: PromiseOrValue<string> | null
    ): ethers.Filter;
    RelayerUpdated(): ethers.Filter;
    TimeoutUpdated(): ethers.Filter;
  };
  
  // Event listeners
  on(
    event: 'AssetLocked' | ethers.EventName,
    listener: (lockId: string, sender: string, recipient: string, token: string, amount: ethers.BigNumber, hashlock: string, timelock: ethers.BigNumber, chainId: ethers.BigNumber, nonce: ethers.BigNumber, event: ethers.Event) => void
  ): this;
  
  on(
    event: 'AssetClaimed' | ethers.EventName,
    listener: (lockId: string, recipient: string, secret: string, falconsig: string, event: ethers.Event) => void
  ): this;
  
  on(
    event: 'AssetRefunded' | ethers.EventName,
    listener: (lockId: string, recipient: string, reason: string, event: ethers.Event) => void
  ): this;
}

// Type guard
export function isQuantumBridgeLock(contract: ethers.BaseContract): contract is QuantumBridgeLock {
  return 'lockETH' in contract && 'claim' in contract && 'refund' in contract;
}
