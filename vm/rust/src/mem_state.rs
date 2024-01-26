use blockifier::execution::contract_class::ContractClass;
use blockifier::state::cached_state::CommitmentStateDiff;
use blockifier::state::errors::StateError;
use blockifier::state::state_api::{State, StateReader, StateResult};
use cached::{Cached, SizedCache};
use once_cell::sync::Lazy;
use starknet_api::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce};
use starknet_api::hash::StarkFelt;
use starknet_api::state::StorageKey;
use std::sync::Mutex;

struct CachedContractClass {
    pub definition: ContractClass,
    pub cached_on_height: u64,
}

static CLASS_CACHE: Lazy<Mutex<SizedCache<ClassHash, CachedContractClass>>> =
    Lazy::new(|| Mutex::new(SizedCache::with_size(128)));
static STORAGE_CACHE: Lazy<Mutex<SizedCache<(ContractAddress, StorageKey), StarkFelt>>> =
    Lazy::new(|| Mutex::new(SizedCache::with_size(128)));
static CLASS_HASH_CACHE: Lazy<Mutex<SizedCache<ContractAddress, ClassHash>>> =
    Lazy::new(|| Mutex::new(SizedCache::with_size(128)));
static NONCE_CACHE: Lazy<Mutex<SizedCache<ContractAddress, Nonce>>> =
    Lazy::new(|| Mutex::new(SizedCache::with_size(128)));
static COMPILED_CLASS_HASH: Lazy<Mutex<SizedCache<ClassHash, CompiledClassHash>>> =
    Lazy::new(|| Mutex::new(SizedCache::with_size(128)));

pub struct MemState {
    height: u64,
}

impl MemState {
    pub fn new(height: u64) -> Self {
        Self { height }
    }
}

impl StateReader for MemState {
    fn get_storage_at(
        &mut self,
        contract_address: ContractAddress,
        key: StorageKey,
    ) -> StateResult<StarkFelt> {
        if let Some(value) = STORAGE_CACHE
            .lock()
            .unwrap()
            .cache_get(&(contract_address, key))
        {
            return Ok(value.clone());
        }
        return Err(StateError::StateReadError(format!(
            "failed to read location {} at address {}",
            key.0.key(),
            contract_address.0.key()
        )));
    }

    fn get_nonce_at(&mut self, contract_address: ContractAddress) -> StateResult<Nonce> {
        if let Some(nonce) = NONCE_CACHE.lock().unwrap().cache_get(&contract_address) {
            return Ok(nonce.clone());
        }
        return Err(StateError::StateReadError(format!(
            "failed to read nonce of address {}",
            contract_address.0.key()
        )));
    }

    fn get_class_hash_at(&mut self, contract_address: ContractAddress) -> StateResult<ClassHash> {
        if let Some(class_hash) = CLASS_HASH_CACHE
            .lock()
            .unwrap()
            .cache_get(&contract_address)
        {
            return Ok(class_hash.clone());
        }
        return Err(StateError::StateReadError(format!(
            "failed to read class hash of address {}",
            contract_address.0.key()
        )));
    }

    fn get_compiled_contract_class(
        &mut self,
        class_hash: &ClassHash,
    ) -> StateResult<ContractClass> {
        if let Some(cached_class) = CLASS_CACHE.lock().unwrap().cache_get(class_hash) {
            if cached_class.cached_on_height < self.height {
                return Ok(cached_class.definition.clone());
            }
        }
        return Err(StateError::UndeclaredClassHash(*class_hash));
    }

    fn get_compiled_class_hash(&mut self, class_hash: ClassHash) -> StateResult<CompiledClassHash> {
        if let Some(compiled_class_hash) =
            COMPILED_CLASS_HASH.lock().unwrap().cache_get(&class_hash)
        {
            return Ok(compiled_class_hash.clone());
        }
        return Err(StateError::UndeclaredClassHash(class_hash.clone()));
    }
}

impl State for MemState {
    fn set_storage_at(
        &mut self,
        contract_address: ContractAddress,
        key: StorageKey,
        value: StarkFelt,
    ) {
        let _ = STORAGE_CACHE
            .lock()
            .unwrap()
            .cache_set((contract_address, key), value);
    }

    fn increment_nonce(&mut self, contract_address: ContractAddress) -> StateResult<()> {
        let current_nonce = self.get_nonce_at(contract_address)?;
        let current_nonce_as_u64 = usize::try_from(current_nonce.0)? as u64;
        let next_nonce_val = 1_u64 + current_nonce_as_u64;
        let next_nonce = Nonce(StarkFelt::from(next_nonce_val));
        let _ = NONCE_CACHE
            .lock()
            .unwrap()
            .cache_set(contract_address, next_nonce);
        Ok(())
    }

    fn set_class_hash_at(
        &mut self,
        contract_address: ContractAddress,
        class_hash: ClassHash,
    ) -> StateResult<()> {
        let _ = CLASS_HASH_CACHE
            .lock()
            .unwrap()
            .cache_set(contract_address, class_hash);
        Ok(())
    }

    fn set_contract_class(
        &mut self,
        class_hash: &ClassHash,
        contract_class: ContractClass,
    ) -> StateResult<()> {
        let _ = CLASS_CACHE.lock().unwrap().cache_set(
            *class_hash,
            CachedContractClass {
                definition: contract_class,
                cached_on_height: 0,
            },
        );
        Ok(())
    }

    fn set_compiled_class_hash(
        &mut self,
        class_hash: ClassHash,
        compiled_class_hash: CompiledClassHash,
    ) -> StateResult<()> {
        let _ = COMPILED_CLASS_HASH
            .lock()
            .unwrap()
            .cache_set(class_hash, compiled_class_hash);
        Ok(())
    }

    fn to_state_diff(&mut self) -> CommitmentStateDiff {
        todo!()
    }
}
