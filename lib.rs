#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod burn2play {
    use super::*;
    use ink::abi::Sol;
    use ink::contract_ref;
    use ink::env::hash;
    use ink::env::return_value;
    use ink::env::DefaultEnvironment;
    use ink::env::FromLittleEndian;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use ink::H160;
    use ink::U256;

    #[ink::trait_definition]
    pub trait Burn {
        #[ink(message)]
        #[allow(non_snake_case)]
        fn burn(&self, value: u128, keep_alive: bool);
    }

    /// Calculates the address of a precompile at index `n` and with some additional prefix.
    #[inline]
    pub fn fixed_address(n: u16) -> Address {
        let shifted = (n as u32) << 16;

        let suffix = shifted.to_be_bytes();
        let mut address = [0u8; 20];
        let mut i = 16;
        while i < address.len() {
            address[i] = suffix[i - 16];
            i = i + 1;
        }
        Address::from(address)
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Burn2play {
        ticket_price: u128,
        burn_perbill: u32,
        closes: BlockNumber,
        claim_fee: u128,
        entries: Mapping<u128, H160>,
        tickets_sold: u128,
    }

    // #[ink(event)]
    // pub struct Awarded {
    //     winning_threshold: U256,
    //     lucky_number: U256,
    // }

    fn get_amount_to_burn(burn_perbill: u32, value: u128) -> u128 {
        value * u128::from(burn_perbill) / 1_000_000_000
    }

    impl Burn2play {
        // TODO PseudoRandom move to a separate contract for composability
        #[ink(message)]
        pub fn get_pseudo_random(&mut self) -> u128 {
            let seed = self.env().block_timestamp();
            let input: Vec<u8> = Vec::from(&seed.to_be_bytes());
            let mut output = <hash::Keccak256 as hash::HashOutput>::Type::default();
            ink::env::hash_bytes::<hash::Keccak256>(&input, &mut output);

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
            ticket_price: u128,
            burn_perbill: u32,
            closes: BlockNumber,
            claim_fee: u128,
        ) -> Self {
            let transferred = Self::env().transferred_value().as_u128();

            if transferred < claim_fee {
                return_value(
                    ink::env::ReturnFlags::REVERT,
                    &("Claim fee must be lower than the transferred value").as_bytes(),
                );
            }

            Self::env().block_number();

            Self {
                ticket_price,
                burn_perbill,
                closes,
                claim_fee,
                entries: Mapping::new(),
                tickets_sold: 0,
            }
        }

        fn burn(amount: u128) {
            let precompile_address = fixed_address(11);
            let mut precompile: contract_ref!(Burn, DefaultEnvironment, Sol) =
                precompile_address.into();
            precompile.burn(amount, true);
        }

        #[ink(message, payable)]
        pub fn burn_and_play(&mut self) {
            if self.closes >= self.env().block_number() {
                return_value(
                    ink::env::ReturnFlags::REVERT,
                    &("Raffle finished").as_bytes(),
                );
            }

            let caller = self.env().caller();
            let value = self.env().transferred_value().as_u128();

            let amount = get_amount_to_burn(self.burn_perbill, value);
            Self::burn(amount);

            let tickets = value / self.ticket_price;
            for i in 0..tickets {
                self.entries.insert(self.tickets_sold + i, &caller);
            }
            self.tickets_sold += tickets;
        }

        #[ink(message)]
        pub fn claim(&mut self) {
            if self.closes < self.env().block_number() {
                return_value(
                    ink::env::ReturnFlags::REVERT,
                    &("Raffle not closed yet").as_bytes(),
                );
            }

            let caller = self.env().caller();

            if self.tickets_sold > 0 {
                // Transfer claim fee
                self.env()
                    .transfer(caller, U256::from(self.claim_fee))
                    .unwrap();

                // Terminate contract awarding the winner
                let winner_idx = self.get_pseudo_random() % self.tickets_sold;
                let winner = self.entries.get(winner_idx).unwrap();

                self.env().terminate_contract(winner);
            } else {
                // Burn everything but the claim fee
                Self::burn(self.env().balance().as_u128() - self.claim_fee);

                // Terminate contract awarding claimer
                self.env().terminate_contract(caller);
            }
        }
    }
}
