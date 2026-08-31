// In pallet/src/lib.rs
use frame_support::{pallet_prelude::*, traits::Get};
use sp_std::vec::Vec;

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug)]
pub enum ConfigValue {
    Bool(bool),
    U32(u32),
    U64(u64),
    Bytes(Vec<u8>),
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn config)]
    pub type ConfigStorage<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, ConfigValue, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn get_config_value(origin: OriginFor<T>, key: Vec<u8>) -> DispatchResult {
            ensure_none(origin)?;
            let _ = <ConfigStorage<T>>::get(&key); // Just to satisfy compiler; actual read is done off-chain
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn get_config_value_raw(key: &[u8]) -> Option<ConfigValue> {
            <ConfigStorage<T>>::get(key)
        }
    }
}