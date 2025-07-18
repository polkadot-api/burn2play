#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod burn2play {
    use ink::abi::Sol;
    use ink::contract_ref;
    use ink::env::hash;
    use ink::env::DefaultEnvironment;
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

    #[ink(storage)]
    pub struct Burn2play {
        ticket_price: u128,
        burn_perbill: u32,
        closes: BlockNumber,
        claim_fee: u128,
        ticket_to_address: Mapping<u128, H160>,
        address_to_ticket: Mapping<H160, u128>,
        tickets_sold: u128,
    }

    #[ink(event)]
    pub struct ParticipantEntered {
        address: H160,
        tickets: (u128, u128),
    }

    #[ink(event)]
    pub struct RaffleClosed {
        ticket: u128,
        amount: u128,
        address: H160,
    }

    #[ink(event)]
    pub struct PlayCalled {}

    #[ink(event)]
    pub struct BeforeBurn {}

    #[ink(event)]
    pub struct AfterBurn {}

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
            duration: BlockNumber,
            claim_fee: u128,
        ) -> Result<Self, Vec<u8>> {
            let transferred = Self::u256_to_u128(Self::env().transferred_value());

            assert!(
                claim_fee < transferred,
                "Claim fee must be lower than the transferred value"
            );

            Ok(Self {
                ticket_price,
                burn_perbill,
                closes: Self::env().block_number().saturating_add(duration),
                claim_fee,
                ticket_to_address: Mapping::new(),
                address_to_ticket: Mapping::new(),
                tickets_sold: 0,
            })
        }

        fn burn(amount: u128) {
            let precompile_address = fixed_address(11);
            let precompile: contract_ref!(Burn, DefaultEnvironment, Sol) =
                precompile_address.into();
            precompile.burn(amount, true);
        }

        fn u256_to_u128(value: U256) -> u128 {
            // Dunno, empirically I found this
            (value / U256::from(100_000_000)).as_u128()
        }

        #[ink(message, payable)]
        pub fn burn_and_play(&mut self) {
            Self::env().emit_event(PlayCalled {});

            assert!(self.closes > self.env().block_number(), "Raffle finished");

            let caller = self.env().caller();
            let value = Self::u256_to_u128(self.env().transferred_value());

            let amount = get_amount_to_burn(self.burn_perbill, value);

            Self::env().emit_event(BeforeBurn {});
            Self::burn(amount);
            Self::env().emit_event(AfterBurn {});

            let tickets = value / self.ticket_price;
            if tickets > 0 {
                for i in 0..tickets {
                    self.ticket_to_address
                        .insert(self.tickets_sold + i, &caller);
                }

                let current_tickets = self.address_to_ticket.get(caller).unwrap_or(0);
                self.address_to_ticket
                    .insert(caller, &(current_tickets + tickets));

                Self::env().emit_event(ParticipantEntered {
                    address: caller,
                    tickets: (self.tickets_sold, self.tickets_sold + tickets - 1),
                });
                self.tickets_sold += tickets;
            }
        }

        #[ink(message)]
        pub fn close(&mut self) {
            assert!(
                self.closes <= self.env().block_number(),
                "Raffle not finished yet"
            );

            let caller = self.env().caller();

            if self.tickets_sold > 0 {
                // Transfer claim fee
                self.env()
                    .transfer(caller, U256::from(self.claim_fee))
                    .unwrap();

                // Terminate contract awarding the winner
                let winner_idx = self.get_pseudo_random() % self.tickets_sold;
                let winner = self.ticket_to_address.get(winner_idx).unwrap();

                Self::env().emit_event(RaffleClosed {
                    address: winner,
                    amount: Self::u256_to_u128(self.env().balance()),
                    ticket: winner_idx,
                });

                self.env().terminate_contract(winner);
            } else {
                // Burn everything but the claim fee
                Self::burn(Self::u256_to_u128(self.env().balance()) - self.claim_fee);

                // Terminate contract awarding claimer
                self.env().terminate_contract(caller);
            }
        }
    }
}
