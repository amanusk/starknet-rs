use async_trait::async_trait;
use starknet_core::types::{
    AddTransactionResult, Block, BlockId, CallContractResult, ContractAddresses, ContractCode,
    InvokeFunction, TransactionId, TransactionReceipt, TransactionRequest, TransactionStatus,
    TransactionWithStatus, UnsignedFieldElement,
};
use std::error::Error;

#[async_trait]
pub trait Provider {
    type Error: Error + Send;

    async fn add_transaction(
        &self,
        tx: TransactionRequest,
        token: Option<String>,
    ) -> Result<AddTransactionResult, Self::Error>;

    async fn get_contract_addresses(&self) -> Result<ContractAddresses, Self::Error>;

    async fn call_contract(
        &self,
        invoke_tx: InvokeFunction,
        block_identifier: Option<BlockId>,
    ) -> Result<CallContractResult, Self::Error>;

    async fn get_block(&self, block_identifier: Option<BlockId>) -> Result<Block, Self::Error>;

    async fn get_code(
        &self,
        contract_address: UnsignedFieldElement,
        block_identifier: Option<BlockId>,
    ) -> Result<ContractCode, Self::Error>;

    async fn get_storage_at(
        &self,
        contract_address: UnsignedFieldElement,
        key: UnsignedFieldElement,
        block_identifier: Option<BlockId>,
    ) -> Result<UnsignedFieldElement, Self::Error>;

    async fn get_transaction_status(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionStatus, Self::Error>;

    async fn get_transaction(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionWithStatus, Self::Error>;

    async fn get_transaction_receipt(
        &self,
        transaction_hash_or_number: TransactionId,
    ) -> Result<TransactionReceipt, Self::Error>;

    async fn get_block_hash_by_id(
        &self,
        block_number: u64,
    ) -> Result<UnsignedFieldElement, Self::Error>;

    async fn get_block_id_by_hash(
        &self,
        block_hash: UnsignedFieldElement,
    ) -> Result<u64, Self::Error>;

    async fn get_transaction_hash_by_id(
        &self,
        transaction_number: u64,
    ) -> Result<UnsignedFieldElement, Self::Error>;

    async fn get_transaction_id_by_hash(
        &self,
        transaction_hash: UnsignedFieldElement,
    ) -> Result<u64, Self::Error>;

    async fn get_last_batch_id(&self) -> Result<u64, Self::Error>;
}
