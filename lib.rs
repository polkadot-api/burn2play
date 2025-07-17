#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod burn2play {
    use ink::env::hash;
    use ink::env::return_value;
    use ink::env::FromLittleEndian;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink::H160;
    use ink::U256;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Burn2play {
        burn_pct: u32,
        claim_fee: u128,
        // While we don't have a burn precompile, this is the best we can do
        value: u128,
        closes: BlockNumber,
        // TODO PseudoRandom move to a separate contract for composability
        salt: u64,
        entries: Mapping<H160, u128>,
    }

    // #[ink(event)]
    // pub struct Awarded {
    //     winning_threshold: U256,
    //     lucky_number: U256,
    // }

    fn get_amount_to_keep(burn_pct: u32, value: u128) -> u128 {
        value - value * u128::from(burn_pct) / 1_000_000_000
    }

    impl Burn2play {
        // TODO PseudoRandom move to a separate contract for composability
        #[ink(message)]
        pub fn get_pseudo_random(&mut self) -> u128 {
            let seed = self.env().block_timestamp();
            let mut input: Vec<u8> = Vec::from(&seed.to_be_bytes());
            input.extend_from_slice(&self.salt.to_be_bytes());
            let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<hash::Keccak256>(&input, &mut output);
            self.salt += 1;

            let mut result: u128 = 0;
            let mut tmp = [0u8; size_of::<u128>()];
            for i in (0..output.len()).step_by(size_of::<u128>()) {
                tmp.clone_from_slice(&output[i..i + size_of::<u128>()]);

                let (new_result, _) = result.overflowing_add(u128::from_le_bytes(tmp));
                result = new_result
            }

            result
        }

        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor, payable)]
        pub fn new(
            initial_burn: u128,
            claim_fee: u128,
            closes: BlockNumber,
            ticket_price: u128,
            // In PERBILL
            burn_pct: u32,
        ) -> Self {
            let transferred = Self::env().transferred_value().as_u128();
            if transferred < initial_burn {
                return_value(
                    ink::env::ReturnFlags::REVERT,
                    &("Can't burn more than transferred value").as_bytes(),
                );
            }

            let value = transferred.saturating_sub(initial_burn);
            if value < claim_fee {
                return_value(
                    ink::env::ReturnFlags::REVERT,
                    &("Claim fee must be lower than the non-burned value").as_bytes(),
                );
            }

            Self::env().block_number();

            Self {
                salt: 0,
                value,
                claim_fee,
                closes,
                burn_pct,
                entries: Mapping::new(),
            }
        }

        #[ink(message, payable)]
        pub fn burn_and_play(&mut self) {
            let caller = self.env().caller();
            let value = self.env().transferred_value().as_u128();

            let current_participation = self.entries.get(caller).unwrap_or(0);
            self.entries
                .insert(caller, &current_participation.saturating_add(value));

            self.value = self
                .value
                .saturating_add(get_amount_to_keep(self.burn_pct, value))
        }

        #[ink(message)]
        pub fn claim(&mut self) {
            if self.closes < self.env().block_number() {
                return_value(
                    ink::env::ReturnFlags::REVERT,
                    &("Raffle not closed yet").as_bytes(),
                );
            }

            let rnd = self.get_pseudo_random();

            // self.entries.it
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            // let burn2play = Burn2play::default();
            // assert_eq!(burn2play.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            // let mut burn2play = Burn2play::new();

            // burn2play.play_2_burn()
        }
    }

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        // /// We test that we can upload and instantiate the contract using its default constructor.
        // #[ink_e2e::test]
        // async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        //     // Given
        //     let mut constructor = Burn2playRef::default();

        //     // When
        //     let contract = client
        //         .instantiate("burn2play", &ink_e2e::alice(), &mut constructor)
        //         .submit()
        //         .await
        //         .expect("instantiate failed");
        //     let call_builder = contract.call_builder::<Burn2play>();

        //     // Then
        //     let get = call_builder.get();
        //     let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
        //     assert!(matches!(get_result.return_value(), false));

        //     Ok(())
        // }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            // let mut constructor = Burn2playRef::new();
            // let contract = client
            //     .instantiate("burn2play", &ink_e2e::bob(), &mut constructor)
            //     .submit()
            //     .await
            //     .expect("instantiate failed");
            // let mut call_builder = contract.call_builder::<Burn2play>();

            // let get = call_builder.play_2_burn();
            // let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            // let result = U256::zero();
            // assert!(matches!(get_result.return_value(), result));

            // // When
            // let flip = call_builder.flip();
            // let _flip_result = client
            //     .call(&ink_e2e::bob(), &flip)
            //     .submit()
            //     .await
            //     .expect("flip failed");

            // // Then
            // let get = call_builder.get();
            // let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            // assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
