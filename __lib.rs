#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod burn2play {
    impl ::ink::env::ContractEnv for Burn2play {
        type Env = ::ink::env::DefaultEnvironment;
    }
    type Environment = <Burn2play as ::ink::env::ContractEnv>::Env;
    type AccountId =
        <<Burn2play as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::AccountId;
    type Balance =
        <<Burn2play as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Balance;
    type Hash = <<Burn2play as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Hash;
    type Timestamp =
        <<Burn2play as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::Timestamp;
    type BlockNumber =
        <<Burn2play as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::BlockNumber;
    type ChainExtension =
        <<Burn2play as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::ChainExtension;
    const MAX_EVENT_TOPICS: usize =
        <<Burn2play as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::MAX_EVENT_TOPICS;
    type EventRecord =
        <<Burn2play as ::ink::env::ContractEnv>::Env as ::ink::env::Environment>::EventRecord;
    type Address = ::ink::primitives::Address;
    type SolBytes<T> = ::ink::primitives::SolBytes<T>;
    const _: () = {
        struct Check {
            salt: (),
            field_0: u128,
            field_1: u32,
            field_2: BlockNumber,
            field_3: Mapping<u32, H160>,
        }
    };
    #[scale_info(crate = ::ink::scale_info)]
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    pub struct Burn2play {
        ticket_price: <u128 as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<1423368495u32, ()>,
        >>::Type,
        burn_perbill: <u32 as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<2724647697u32, ()>,
        >>::Type,
        closes: <BlockNumber as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<1023692690u32, ()>,
        >>::Type,
        claim_fee: <u128 as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<2608397937u32, ()>,
        >>::Type,
        entries: <Mapping<u32, H160> as ::ink::storage::traits::AutoStorableHint<
            ::ink::storage::traits::ManualKey<1181160673u32, ()>,
        >>::Type,
    }
    const _: () = {
        impl<__ink_generic_salt: ::ink::storage::traits::StorageKey>
            ::ink::storage::traits::StorableHint<__ink_generic_salt> for Burn2play
        {
            type Type = Burn2play;
            type PreferredKey = ::ink::storage::traits::AutoKey;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageKey for Burn2play {
            const KEY: ::ink::primitives::Key = <() as ::ink::storage::traits::StorageKey>::KEY;
        }
    };
    const _: () = {
        impl ::ink::storage::traits::Storable for Burn2play {
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn decode<__ink_I: ::ink::scale::Input>(
                __input: &mut __ink_I,
            ) -> ::core::result::Result<Self, ::ink::scale::Error> {
                ::core::result::Result::Ok(Burn2play {
                    ticket_price: <<u128 as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<1423368495u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(
                        __input
                    )?,
                    burn_perbill: <<u32 as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<2724647697u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(
                        __input
                    )?,
                    closes: <<BlockNumber as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<1023692690u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(
                        __input
                    )?,
                    claim_fee: <<u128 as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<2608397937u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(
                        __input
                    )?,
                    entries: <<Mapping<u32, H160> as ::ink::storage::traits::AutoStorableHint<
                        ::ink::storage::traits::ManualKey<1181160673u32, ()>,
                    >>::Type as ::ink::storage::traits::Storable>::decode(
                        __input
                    )?,
                })
            }
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn encode<__ink_O: ::ink::scale::Output + ?::core::marker::Sized>(
                &self,
                __dest: &mut __ink_O,
            ) {
                match self {
                    Burn2play {
                        ticket_price: __binding_0,
                        burn_perbill: __binding_1,
                        closes: __binding_2,
                        claim_fee: __binding_3,
                        entries: __binding_4,
                    } => {
                        {
                            ::ink::storage::traits::Storable::encode(__binding_0, __dest);
                        }
                        {
                            ::ink::storage::traits::Storable::encode(__binding_1, __dest);
                        }
                        {
                            ::ink::storage::traits::Storable::encode(__binding_2, __dest);
                        }
                        {
                            ::ink::storage::traits::Storable::encode(__binding_3, __dest);
                        }
                        {
                            ::ink::storage::traits::Storable::encode(__binding_4, __dest);
                        }
                    }
                }
            }
            #[inline(always)]
            #[allow(non_camel_case_types)]
            fn encoded_size(&self) -> ::core::primitive::usize {
                match self {
                    Burn2play {
                        ticket_price: __binding_0,
                        burn_perbill: __binding_1,
                        closes: __binding_2,
                        claim_fee: __binding_3,
                        entries: __binding_4,
                    } => ::core::primitive::usize::MIN
                        .saturating_add(::ink::storage::traits::Storable::encoded_size(__binding_0))
                        .saturating_add(::ink::storage::traits::Storable::encoded_size(__binding_1))
                        .saturating_add(::ink::storage::traits::Storable::encoded_size(__binding_2))
                        .saturating_add(::ink::storage::traits::Storable::encoded_size(__binding_3))
                        .saturating_add(::ink::storage::traits::Storable::encoded_size(
                            __binding_4,
                        )),
                }
            }
        }
    };
    #[allow(
        non_upper_case_globals,
        deprecated,
        unused_attributes,
        unused_qualifications
    )]
    const _: () = {
        impl ::ink::scale_info::TypeInfo for Burn2play {
            type Identity = Self;
            fn type_info() -> ::ink::scale_info::Type {
                ::ink::scale_info::Type::builder()
                    .path(
                        ::ink::scale_info::Path::new_with_replace(
                            "Burn2play",
                            "burn2play::burn2play",
                            &[],
                        ),
                    )
                    .type_params(::alloc::vec::Vec::new())
                    .docs(
                        &[
                            "Defines the storage of your contract.",
                            "Add new fields to the below struct in order",
                            "to add new static storage fields to your contract.",
                        ],
                    )
                    .composite(
                        ::ink::scale_info::build::Fields::named()
                            .field(|f| {
                                f
                                    .ty::<
                                        <u128 as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<1423368495u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("ticket_price")
                                    .type_name(
                                        "<u128 as::ink::storage::traits::AutoStorableHint<::ink::storage\n::traits::ManualKey<1423368495u32, ()>,>>::Type",
                                    )
                            })
                            .field(|f| {
                                f
                                    .ty::<
                                        <u32 as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<2724647697u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("burn_perbill")
                                    .type_name(
                                        "<u32 as::ink::storage::traits::AutoStorableHint<::ink::storage\n::traits::ManualKey<2724647697u32, ()>,>>::Type",
                                    )
                            })
                            .field(|f| {
                                f
                                    .ty::<
                                        <BlockNumber as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<1023692690u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("closes")
                                    .type_name(
                                        "<BlockNumber as::ink::storage::traits::AutoStorableHint<::ink::\nstorage::traits::ManualKey<1023692690u32, ()>,>>::Type",
                                    )
                            })
                            .field(|f| {
                                f
                                    .ty::<
                                        <u128 as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<2608397937u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("claim_fee")
                                    .type_name(
                                        "<u128 as::ink::storage::traits::AutoStorableHint<::ink::storage\n::traits::ManualKey<2608397937u32, ()>,>>::Type",
                                    )
                            })
                            .field(|f| {
                                f
                                    .ty::<
                                        <Mapping<
                                            u32,
                                            H160,
                                        > as ::ink::storage::traits::AutoStorableHint<
                                            ::ink::storage::traits::ManualKey<1181160673u32, ()>,
                                        >>::Type,
                                    >()
                                    .name("entries")
                                    .type_name(
                                        "<Mapping<u32, H160> as::ink::storage::traits::AutoStorableHint<\n::ink::storage::traits::ManualKey<1181160673u32, ()>,>>::Type",
                                    )
                            }),
                    )
            }
        }
    };
    const _: () = {
        impl ::ink::storage::traits::StorageLayout for Burn2play {
            fn layout(__key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(::ink::metadata::layout::StructLayout::new(
                    "Burn2play",
                    [
                        ::ink::metadata::layout::FieldLayout::new(
                            "ticket_price",
                            <<u128 as ::ink::storage::traits::AutoStorableHint<
                                ::ink::storage::traits::ManualKey<1423368495u32, ()>,
                            >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                __key
                            ),
                        ),
                        ::ink::metadata::layout::FieldLayout::new(
                            "burn_perbill",
                            <<u32 as ::ink::storage::traits::AutoStorableHint<
                                ::ink::storage::traits::ManualKey<2724647697u32, ()>,
                            >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                __key
                            ),
                        ),
                        ::ink::metadata::layout::FieldLayout::new(
                            "closes",
                            <<BlockNumber as ::ink::storage::traits::AutoStorableHint<
                                ::ink::storage::traits::ManualKey<1023692690u32, ()>,
                            >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                __key
                            ),
                        ),
                        ::ink::metadata::layout::FieldLayout::new(
                            "claim_fee",
                            <<u128 as ::ink::storage::traits::AutoStorableHint<
                                ::ink::storage::traits::ManualKey<2608397937u32, ()>,
                            >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                __key
                            ),
                        ),
                        ::ink::metadata::layout::FieldLayout::new(
                            "entries",
                            <<Mapping<u32, H160> as ::ink::storage::traits::AutoStorableHint<
                                ::ink::storage::traits::ManualKey<1181160673u32, ()>,
                            >>::Type as ::ink::storage::traits::StorageLayout>::layout(
                                __key
                            ),
                        ),
                    ],
                ))
            }
        }
    };
    const _: () = {
        impl ::ink::reflect::ContractName for Burn2play {
            const NAME: &'static str = "Burn2play";
        }
        #[allow(non_camel_case_types)]
        trait __ink_StorageMarker {}
        impl __ink_StorageMarker for Burn2play {}
    };
    const _: () = {
        impl<'a> ::ink::codegen::Env for &'a Burn2play {
            type EnvAccess = ::ink::EnvAccess<'a, <Burn2play as ::ink::env::ContractEnv>::Env>;
            fn env(self) -> Self::EnvAccess {
                <<Self as ::ink::codegen::Env>::EnvAccess as ::core::default::Default>::default()
            }
        }
        impl<'a> ::ink::codegen::StaticEnv for Burn2play {
            type EnvAccess = ::ink::EnvAccess<'static, <Burn2play as ::ink::env::ContractEnv>::Env>;
            fn env() -> Self::EnvAccess {
                <<Self as ::ink::codegen::StaticEnv>::EnvAccess as ::core::default::Default>::default()
            }
        }
    };
    const _: () = {
        #[allow(unused_imports)]
        use ::ink::codegen::{Env as _, StaticEnv as _};
    };
    impl ::ink::reflect::DispatchableConstructorInfo<0x9BAE9D5E_u32> for Burn2play {
        type Input = (u128, u32, BlockNumber, u128);
        type Output = Self;
        type Storage = Burn2play;
        type Error =
            <::ink::reflect::ConstructorOutputValue<Self> as ::ink::reflect::ConstructorOutput<
                Burn2play,
            >>::Error;
        const IS_RESULT: ::core::primitive::bool =
            <::ink::reflect::ConstructorOutputValue<Self> as ::ink::reflect::ConstructorOutput<
                Burn2play,
            >>::IS_RESULT;
        const CALLABLE: fn(Self::Input) -> Self::Output =
            |(__ink_binding_0, __ink_binding_1, __ink_binding_2, __ink_binding_3)| {
                Burn2play::new(
                    __ink_binding_0,
                    __ink_binding_1,
                    __ink_binding_2,
                    __ink_binding_3,
                )
            };
        const DECODE: fn(
            &mut &[::core::primitive::u8],
        )
            -> ::core::result::Result<Self::Input, ::ink::env::DispatchError> = |input| {
            <Self::Input as ::ink::scale::Decode>::decode(input)
                .map_err(|_| ::ink::env::DispatchError::InvalidParameters)
        };
        const PAYABLE: ::core::primitive::bool = true;
        const SELECTOR: ::core::option::Option<[::core::primitive::u8; 4usize]> =
            ::core::option::Option::Some([0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8]);
        const LABEL: &'static ::core::primitive::str = "new";
        const ABI: ::ink::abi::Abi = ::ink::abi::Abi::Ink;
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x18A1C342_u32> for Burn2play {
        type Input = ();
        type Output = u128;
        type Storage = Burn2play;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output =
            |storage, _| Burn2play::get_pseudo_random(storage);
        const DECODE: fn(
            &mut &[::core::primitive::u8],
        )
            -> ::core::result::Result<Self::Input, ::ink::env::DispatchError> = |input| {
            <Self::Input as ::ink::scale::Decode>::decode(input)
                .map_err(|_| ::ink::env::DispatchError::InvalidParameters)
        };
        const RETURN: fn(::ink::env::ReturnFlags, Self::Output) -> () = |flags, output| {
            ::ink::env::return_value::<::ink::MessageResult<Self::Output>>(
                flags,
                &::ink::MessageResult::Ok(output),
            )
        };
        const SELECTOR: [::core::primitive::u8; 4usize] = [0x18_u8, 0xA1_u8, 0xC3_u8, 0x42_u8];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "get_pseudo_random";
        const ABI: ::ink::abi::Abi = ::ink::abi::Abi::Ink;
    }
    impl ::ink::reflect::DispatchableMessageInfo<0x9B1A3A45_u32> for Burn2play {
        type Input = ();
        type Output = ();
        type Storage = Burn2play;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output =
            |storage, _| Burn2play::burn_and_play(storage);
        const DECODE: fn(
            &mut &[::core::primitive::u8],
        )
            -> ::core::result::Result<Self::Input, ::ink::env::DispatchError> = |input| {
            <Self::Input as ::ink::scale::Decode>::decode(input)
                .map_err(|_| ::ink::env::DispatchError::InvalidParameters)
        };
        const RETURN: fn(::ink::env::ReturnFlags, Self::Output) -> () = |flags, output| {
            ::ink::env::return_value::<::ink::MessageResult<Self::Output>>(
                flags,
                &::ink::MessageResult::Ok(output),
            )
        };
        const SELECTOR: [::core::primitive::u8; 4usize] = [0x9B_u8, 0x1A_u8, 0x3A_u8, 0x45_u8];
        const PAYABLE: ::core::primitive::bool = true;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "burn_and_play";
        const ABI: ::ink::abi::Abi = ::ink::abi::Abi::Ink;
    }
    impl ::ink::reflect::DispatchableMessageInfo<0xB388803F_u32> for Burn2play {
        type Input = ();
        type Output = ();
        type Storage = Burn2play;
        const CALLABLE: fn(&mut Self::Storage, Self::Input) -> Self::Output =
            |storage, _| Burn2play::claim(storage);
        const DECODE: fn(
            &mut &[::core::primitive::u8],
        )
            -> ::core::result::Result<Self::Input, ::ink::env::DispatchError> = |input| {
            <Self::Input as ::ink::scale::Decode>::decode(input)
                .map_err(|_| ::ink::env::DispatchError::InvalidParameters)
        };
        const RETURN: fn(::ink::env::ReturnFlags, Self::Output) -> () = |flags, output| {
            ::ink::env::return_value::<::ink::MessageResult<Self::Output>>(
                flags,
                &::ink::MessageResult::Ok(output),
            )
        };
        const SELECTOR: [::core::primitive::u8; 4usize] = [0xB3_u8, 0x88_u8, 0x80_u8, 0x3F_u8];
        const PAYABLE: ::core::primitive::bool = false;
        const MUTATES: ::core::primitive::bool = true;
        const LABEL: &'static ::core::primitive::str = "claim";
        const ABI: ::ink::abi::Abi = ::ink::abi::Abi::Ink;
    }
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_ConstructorDecoder {
            Constructor0(
                <Burn2play as ::ink::reflect::DispatchableConstructorInfo<0x9BAE9D5E_u32>>::Input,
            ),
        }
        impl ::ink::env::DecodeDispatch for __ink_ConstructorDecoder {
            fn decode_dispatch(
                input: &mut &[::core::primitive::u8],
            ) -> ::core::result::Result<Self, ::ink::env::DispatchError> {
                const CONSTRUCTOR_0: [::core::primitive::u8; 4usize] = <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                    0x9BAE9D5E_u32,
                >>::SELECTOR
                    .expect("Expected a selector");
                match <[::core::primitive::u8; 4usize] as ::ink::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::env::DispatchError::InvalidSelector)?
                {
                    CONSTRUCTOR_0 => ::core::result::Result::Ok(Self::Constructor0(
                        <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                            0x9BAE9D5E_u32,
                        >>::DECODE(input)?,
                    )),
                    _invalid => {
                        ::core::result::Result::Err(::ink::env::DispatchError::UnknownSelector)
                    }
                }
            }
        }
        impl ::ink::reflect::ExecuteDispatchable for __ink_ConstructorDecoder {
            #[allow(clippy::nonminimal_bool, dead_code)]
            fn execute_dispatchable(self) -> ::core::result::Result<(), ::ink::env::DispatchError> {
                match self {
                    Self::Constructor0(input) => {
                        if {
                            false || {
                                let constructor_0 = false;
                                let constructor_0 =
                                    <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                                        0x9BAE9D5E_u32,
                                    >>::PAYABLE;
                                constructor_0
                            }
                        } && !<Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                            0x9BAE9D5E_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment()?;
                        }
                        let result: <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                            0x9BAE9D5E_u32,
                        >>::Output = <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                            0x9BAE9D5E_u32,
                        >>::CALLABLE(input);
                        let output_value = ::ink::reflect::ConstructorOutputValue::new(result);
                        let output_result = <::ink::reflect::ConstructorOutputValue<
                            <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                                0x9BAE9D5E_u32,
                            >>::Output,
                        > as ::ink::reflect::ConstructorOutput<Burn2play>>::as_result(
                            &output_value,
                        );
                        if let ::core::result::Result::Ok(contract) = output_result.as_ref() {
                            ::ink::env::set_contract_storage::<::ink::primitives::Key, Burn2play>(
                                &<Burn2play as ::ink::storage::traits::StorageKey>::KEY,
                                contract,
                            );
                        }
                        let mut flag = ::ink::env::ReturnFlags::empty();
                        if output_result.is_err() {
                            flag = ::ink::env::ReturnFlags::REVERT;
                        }
                        ::ink::env::return_value::<
                            ::ink::ConstructorResult<
                                ::core::result::Result<
                                    (),
                                    &<::ink::reflect::ConstructorOutputValue<
                                        <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                                            0x9BAE9D5E_u32,
                                        >>::Output,
                                    > as ::ink::reflect::ConstructorOutput<Burn2play>>::Error,
                                >,
                            >,
                        >(
                            flag,
                            &::ink::ConstructorResult::Ok(output_result.map(|_| ())),
                        );
                        return ::core::result::Result::Ok(());
                    }
                }
            }
        }
        impl ::ink::reflect::ContractConstructorDecoder for Burn2play {
            type Type = __ink_ConstructorDecoder;
        }
    };
    const _: () = {
        #[allow(non_camel_case_types)]
        pub enum __ink_MessageDecoder {
            Message0(<Burn2play as ::ink::reflect::DispatchableMessageInfo<0x18A1C342_u32>>::Input),
            Message1(<Burn2play as ::ink::reflect::DispatchableMessageInfo<0x9B1A3A45_u32>>::Input),
            Message2(<Burn2play as ::ink::reflect::DispatchableMessageInfo<0xB388803F_u32>>::Input),
        }
        impl ::ink::env::DecodeDispatch for __ink_MessageDecoder {
            fn decode_dispatch(
                input: &mut &[::core::primitive::u8],
            ) -> ::core::result::Result<Self, ::ink::env::DispatchError> {
                const MESSAGE_0: [::core::primitive::u8; 4usize] = <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                    0x18A1C342_u32,
                >>::SELECTOR;
                const MESSAGE_1: [::core::primitive::u8; 4usize] = <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                    0x9B1A3A45_u32,
                >>::SELECTOR;
                const MESSAGE_2: [::core::primitive::u8; 4usize] = <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                    0xB388803F_u32,
                >>::SELECTOR;
                match <[::core::primitive::u8; 4usize] as ::ink::scale::Decode>::decode(input)
                    .map_err(|_| ::ink::env::DispatchError::InvalidSelector)?
                {
                    MESSAGE_0 => ::core::result::Result::Ok(
                        Self::Message0(<Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x18A1C342_u32,
                        >>::DECODE(input)?),
                    ),
                    MESSAGE_1 => ::core::result::Result::Ok(
                        Self::Message1(<Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x9B1A3A45_u32,
                        >>::DECODE(input)?),
                    ),
                    MESSAGE_2 => ::core::result::Result::Ok(
                        Self::Message2(<Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0xB388803F_u32,
                        >>::DECODE(input)?),
                    ),
                    _invalid => {
                        ::core::result::Result::Err(::ink::env::DispatchError::UnknownSelector)
                    }
                }
            }
        }
        fn push_contract(contract: ::core::mem::ManuallyDrop<Burn2play>, mutates: bool) {
            if mutates {
                ::ink::env::set_contract_storage::<::ink::primitives::Key, Burn2play>(
                    &<Burn2play as ::ink::storage::traits::StorageKey>::KEY,
                    &contract,
                );
            }
        }
        impl ::ink::reflect::ExecuteDispatchable for __ink_MessageDecoder {
            #[allow(clippy::nonminimal_bool, clippy::let_unit_value, dead_code)]
            fn execute_dispatchable(self) -> ::core::result::Result<(), ::ink::env::DispatchError> {
                let key = <Burn2play as ::ink::storage::traits::StorageKey>::KEY;
                let mut contract: ::core::mem::ManuallyDrop<Burn2play> =
                    ::core::mem::ManuallyDrop::new(match ::ink::env::get_contract_storage(&key) {
                        ::core::result::Result::Ok(::core::option::Option::Some(value)) => value,
                        ::core::result::Result::Ok(::core::option::Option::None) => {
                            ::core::panicking::panic_fmt(format_args!("storage entry was empty"));
                        }
                        ::core::result::Result::Err(_) => {
                            ::core::panicking::panic_fmt(format_args!(
                                "could not properly decode storage entry"
                            ));
                        }
                    });
                match self {
                    Self::Message0(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0x18A1C342_u32,
                                        >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0x9B1A3A45_u32,
                                        >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0xB388803F_u32,
                                        >>::PAYABLE;
                                    message_2
                                }
                        } && !<Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x18A1C342_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment()?;
                        }
                        let result: <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x18A1C342_u32,
                        >>::Output = <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x18A1C342_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                    0x18A1C342_u32,
                                >>::Output,
                            >::VALUE
                        } && {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultErrFallback as _;
                            ::ink::result_info::IsResultErr(&result).value()
                        };
                        let mut flag = ::ink::env::ReturnFlags::REVERT;
                        if !is_reverted {
                            flag = ::ink::env::ReturnFlags::empty();
                            push_contract(
                                contract,
                                <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                    0x18A1C342_u32,
                                >>::MUTATES,
                            );
                        }
                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x18A1C342_u32,
                        >>::RETURN(flag, result);
                        return ::core::result::Result::Ok(());
                    }
                    Self::Message1(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0x18A1C342_u32,
                                        >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0x9B1A3A45_u32,
                                        >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0xB388803F_u32,
                                        >>::PAYABLE;
                                    message_2
                                }
                        } && !<Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x9B1A3A45_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment()?;
                        }
                        let result: <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x9B1A3A45_u32,
                        >>::Output = <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x9B1A3A45_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                    0x9B1A3A45_u32,
                                >>::Output,
                            >::VALUE
                        } && {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultErrFallback as _;
                            ::ink::result_info::IsResultErr(&result).value()
                        };
                        let mut flag = ::ink::env::ReturnFlags::REVERT;
                        if !is_reverted {
                            flag = ::ink::env::ReturnFlags::empty();
                            push_contract(
                                contract,
                                <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                    0x9B1A3A45_u32,
                                >>::MUTATES,
                            );
                        }
                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0x9B1A3A45_u32,
                        >>::RETURN(flag, result);
                        return ::core::result::Result::Ok(());
                    }
                    Self::Message2(input) => {
                        if {
                            false
                                || {
                                    let message_0 = false;
                                    let message_0 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0x18A1C342_u32,
                                        >>::PAYABLE;
                                    message_0
                                }
                                || {
                                    let message_1 = false;
                                    let message_1 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0x9B1A3A45_u32,
                                        >>::PAYABLE;
                                    message_1
                                }
                                || {
                                    let message_2 = false;
                                    let message_2 =
                                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                            0xB388803F_u32,
                                        >>::PAYABLE;
                                    message_2
                                }
                        } && !<Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0xB388803F_u32,
                        >>::PAYABLE
                        {
                            ::ink::codegen::deny_payment()?;
                        }
                        let result: <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0xB388803F_u32,
                        >>::Output = <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0xB388803F_u32,
                        >>::CALLABLE(&mut contract, input);
                        let is_reverted = {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultTypeFallback as _;
                            ::ink::result_info::IsResultType::<
                                <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                    0xB388803F_u32,
                                >>::Output,
                            >::VALUE
                        } && {
                            #[allow(unused_imports)]
                            use ::ink::result_info::IsResultErrFallback as _;
                            ::ink::result_info::IsResultErr(&result).value()
                        };
                        let mut flag = ::ink::env::ReturnFlags::REVERT;
                        if !is_reverted {
                            flag = ::ink::env::ReturnFlags::empty();
                            push_contract(
                                contract,
                                <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                                    0xB388803F_u32,
                                >>::MUTATES,
                            );
                        }
                        <Burn2play as ::ink::reflect::DispatchableMessageInfo<
                            0xB388803F_u32,
                        >>::RETURN(flag, result);
                        return ::core::result::Result::Ok(());
                    }
                }
            }
        }
        impl ::ink::reflect::ContractMessageDecoder for Burn2play {
            type Type = __ink_MessageDecoder;
        }
    };
    const _: () = {
        use ::ink::codegen::{Env as _, StaticEnv as _};
        const _: ::ink::codegen::utils::IsSameType<Burn2play> =
            ::ink::codegen::utils::IsSameType::<Burn2play>::new();
        impl Burn2play {
            /// Constructor that initializes the `bool` value to the given `init_value`.
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
                }
            }
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
                    result = new_result;
                }
                result
            }
            pub fn burn_and_play(&mut self) {
                let caller = self.env().caller();
                let value = self.env().transferred_value().as_u128();
                let amount = get_amount_to_burn(self.burn_perbill, value);
                let precompile_address = fixed_address(11);
                let mut precompile: <<::ink::reflect::TraitDefinitionRegistry<
                    DefaultEnvironment,
                > as Burn>::__ink_TraitInfo as ::ink::codegen::TraitCallForwarder>::Forwarder<
                    Sol,
                > = precompile_address.into();
                precompile.burn(amount, true);
            }
            pub fn claim(&mut self) {
                if self.closes < self.env().block_number() {
                    return_value(
                        ink::env::ReturnFlags::REVERT,
                        &("Raffle not closed yet").as_bytes(),
                    );
                }
                let rnd = self.get_pseudo_random();
            }
        }
        const _: () = {
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u128>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u32>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<BlockNumber>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u128>>();
            ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchOutput<u128>>();
        };
    };
    const _: () = {
        #[codec(crate = ::ink::scale)]
        /// The ink! smart contract's call builder.
        ///
        /// Implements the underlying on-chain calling of the ink! smart contract
        /// messages and trait implementations in a type safe way.
        #[repr(transparent)]
        pub struct CallBuilder<Abi> {
            addr: ::ink::Address,
            _marker: core::marker::PhantomData<Abi>,
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl<Abi> ::ink::scale::Decode for CallBuilder<Abi>
            where
                core::marker::PhantomData<Abi>: ::ink::scale::Decode,
                core::marker::PhantomData<Abi>: ::ink::scale::Decode,
            {
                fn decode<__CodecInputEdqy: ::ink::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::ink::scale::Error> {
                    ::core::result::Result::Ok(CallBuilder::<Abi> {
                        addr: {
                            let __codec_res_edqy = <::ink::Address as ::ink::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `CallBuilder::addr`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        _marker: {
                            let __codec_res_edqy =
                                <core::marker::PhantomData<Abi> as ::ink::scale::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(
                                        e.chain("Could not decode `CallBuilder::_marker`"),
                                    );
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    })
                }
                fn decode_into<__CodecInputEdqy: ::ink::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                    dst_: &mut ::core::mem::MaybeUninit<Self>,
                ) -> ::core::result::Result<::ink::scale::DecodeFinished, ::ink::scale::Error>
                {
                    match (
                        &(::core::mem::size_of::<::ink::Address>()
                            + ::core::mem::size_of::<core::marker::PhantomData<Abi>>()),
                        &::core::mem::size_of::<Self>(),
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    if !(if ::core::mem::size_of::<::ink::Address>() > 0 {
                        1
                    } else {
                        0
                    } + if ::core::mem::size_of::<core::marker::PhantomData<Abi>>() > 0 {
                        1
                    } else {
                        0
                    } <= 1)
                    {
                        ::core::panicking::panic(
                            "assertion failed: if ::core::mem::size_of::<::ink::Address>() > 0 { 1 } else { 0 } +\n        if ::core::mem::size_of::<core::marker::PhantomData<Abi>>() > 0 {\n            1\n        } else { 0 } <= 1",
                        )
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<::ink::Address> = unsafe {
                            &mut *dst_
                                .as_mut_ptr()
                                .cast::<::core::mem::MaybeUninit<::ink::Address>>()
                        };
                        <::ink::Address as ::ink::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<core::marker::PhantomData<Abi>> = unsafe {
                            &mut *dst_
                                .as_mut_ptr()
                                .cast::<::core::mem::MaybeUninit<core::marker::PhantomData<Abi>>>()
                        };
                        <core::marker::PhantomData<Abi> as ::ink::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    unsafe {
                        ::core::result::Result::Ok(
                            ::ink::scale::DecodeFinished::assert_decoding_finished(),
                        )
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl<Abi> ::ink::scale::Encode for CallBuilder<Abi>
            where
                core::marker::PhantomData<Abi>: ::ink::scale::Encode,
                core::marker::PhantomData<Abi>: ::ink::scale::Encode,
            {
                fn size_hint(&self) -> usize {
                    0_usize
                        .saturating_add(::ink::scale::Encode::size_hint(&self.addr))
                        .saturating_add(::ink::scale::Encode::size_hint(&self._marker))
                }
                fn encode_to<__CodecOutputEdqy: ::ink::scale::Output + ?::core::marker::Sized>(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    ::ink::scale::Encode::encode_to(&self.addr, __codec_dest_edqy);
                    ::ink::scale::Encode::encode_to(&self._marker, __codec_dest_edqy);
                }
            }
            #[automatically_derived]
            impl<Abi> ::ink::scale::EncodeLike for CallBuilder<Abi>
            where
                core::marker::PhantomData<Abi>: ::ink::scale::Encode,
                core::marker::PhantomData<Abi>: ::ink::scale::Encode,
            {
            }
        };
        #[automatically_derived]
        impl<Abi: ::core::fmt::Debug> ::core::fmt::Debug for CallBuilder<Abi> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CallBuilder",
                    "addr",
                    &self.addr,
                    "_marker",
                    &&self._marker,
                )
            }
        }
        #[automatically_derived]
        impl<Abi: ::core::hash::Hash> ::core::hash::Hash for CallBuilder<Abi> {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.addr, state);
                ::core::hash::Hash::hash(&self._marker, state)
            }
        }
        #[automatically_derived]
        impl<Abi> ::core::marker::StructuralPartialEq for CallBuilder<Abi> {}
        #[automatically_derived]
        impl<Abi: ::core::cmp::PartialEq> ::core::cmp::PartialEq for CallBuilder<Abi> {
            #[inline]
            fn eq(&self, other: &CallBuilder<Abi>) -> bool {
                self.addr == other.addr && self._marker == other._marker
            }
        }
        #[automatically_derived]
        impl<Abi: ::core::cmp::Eq> ::core::cmp::Eq for CallBuilder<Abi> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::ink::Address>;
                let _: ::core::cmp::AssertParamIsEq<core::marker::PhantomData<Abi>>;
            }
        }
        #[automatically_derived]
        impl<Abi: ::core::clone::Clone> ::core::clone::Clone for CallBuilder<Abi> {
            #[inline]
            fn clone(&self) -> CallBuilder<Abi> {
                CallBuilder {
                    addr: ::core::clone::Clone::clone(&self.addr),
                    _marker: ::core::clone::Clone::clone(&self._marker),
                }
            }
        }
        const _: () = {
            impl ::ink::codegen::ContractCallBuilder for Burn2play {
                type Type<Abi> = CallBuilder<Abi>;
            }
            impl<Abi> ::ink::env::ContractEnv for CallBuilder<Abi> {
                type Env = <Burn2play as ::ink::env::ContractEnv>::Env;
            }
            impl<Abi> ::ink::storage::traits::StorageLayout for CallBuilder<Abi> {
                fn layout(__key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                    ::ink::metadata::layout::Layout::Struct(
                        ::ink::metadata::layout::StructLayout::new(
                            "CallBuilder",
                            [::ink::metadata::layout::FieldLayout::new(
                                "addr",
                                <::ink::Address as ::ink::storage::traits::StorageLayout>::layout(
                                    __key,
                                ),
                            )],
                        ),
                    )
                }
            }
            impl<Abi> ::ink::scale_info::TypeInfo for CallBuilder<Abi>
            where
                ::ink::Address: ::ink::scale_info::TypeInfo + 'static,
            {
                type Identity = ::ink::Address;
                fn type_info() -> ::ink::scale_info::Type {
                    <::ink::Address as ::ink::scale_info::TypeInfo>::type_info()
                }
            }
        };
        impl<Abi> ::ink::env::call::FromAddr for CallBuilder<Abi> {
            #[inline]
            fn from_addr(addr: ::ink::Address) -> Self {
                Self {
                    addr,
                    _marker: ::core::default::Default::default(),
                }
            }
        }
        impl<Abi> ::ink::ToAddr for CallBuilder<Abi> {
            #[inline]
            fn to_addr(&self) -> ::ink::Address {
                <::ink::Address as ::core::clone::Clone>::clone(&self.addr)
            }
        }
        impl<Abi> ::core::convert::AsRef<::ink::Address> for CallBuilder<Abi> {
            fn as_ref(&self) -> &::ink::Address {
                &self.addr
            }
        }
        impl<Abi> ::core::convert::AsMut<::ink::Address> for CallBuilder<Abi> {
            fn as_mut(&mut self) -> &mut ::ink::Address {
                &mut self.addr
            }
        }
        impl CallBuilder<::ink::abi::Ink> {
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn get_pseudo_random(
                &mut self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList<::ink::abi::Ink>,
                        ::ink::abi::Ink,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<u128>>,
            > {
                ::ink::env::call::build_call_abi::<Environment, ::ink::abi::Ink>()
                    .call(::ink::ToAddr::to_addr(self))
                    .exec_input(::ink::env::call::ExecutionInput::new(
                        ::ink::env::call::Selector::new([0x18_u8, 0xA1_u8, 0xC3_u8, 0x42_u8]),
                    ))
                    .returns::<u128>()
            }
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn burn_and_play(
                &mut self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList<::ink::abi::Ink>,
                        ::ink::abi::Ink,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<()>>,
            > {
                ::ink::env::call::build_call_abi::<Environment, ::ink::abi::Ink>()
                    .call(::ink::ToAddr::to_addr(self))
                    .exec_input(::ink::env::call::ExecutionInput::new(
                        ::ink::env::call::Selector::new([0x9B_u8, 0x1A_u8, 0x3A_u8, 0x45_u8]),
                    ))
                    .returns::<()>()
            }
            #[allow(clippy::type_complexity)]
            #[inline]
            pub fn claim(
                &mut self,
            ) -> ::ink::env::call::CallBuilder<
                Environment,
                ::ink::env::call::utils::Set<::ink::env::call::Call>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::EmptyArgumentList<::ink::abi::Ink>,
                        ::ink::abi::Ink,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<()>>,
            > {
                ::ink::env::call::build_call_abi::<Environment, ::ink::abi::Ink>()
                    .call(::ink::ToAddr::to_addr(self))
                    .exec_input(::ink::env::call::ExecutionInput::new(
                        ::ink::env::call::Selector::new([0xB3_u8, 0x88_u8, 0x80_u8, 0x3F_u8]),
                    ))
                    .returns::<()>()
            }
        }
    };
    #[codec(crate = ::ink::scale)]
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    pub struct Burn2playRef<Abi = ::ink::abi::Ink> {
        inner: <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>,
        _marker: core::marker::PhantomData<Abi>,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<Abi> ::ink::scale::Decode for Burn2playRef<Abi>
        where
            <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>: ::ink::scale::Decode,
            <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>: ::ink::scale::Decode,
            core::marker::PhantomData<Abi>: ::ink::scale::Decode,
            core::marker::PhantomData<Abi>: ::ink::scale::Decode,
        {
            fn decode<__CodecInputEdqy: ::ink::scale::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::ink::scale::Error> {
                ::core::result::Result::Ok(Burn2playRef::<Abi> {
                    inner: {
                        let __codec_res_edqy = <<Burn2play as ::ink::codegen::ContractCallBuilder>::Type<
                            Abi,
                        > as ::ink::scale::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Burn2playRef::inner`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    _marker: {
                        let __codec_res_edqy =
                            <core::marker::PhantomData<Abi> as ::ink::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Burn2playRef::_marker`"),
                                );
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<Abi> ::ink::scale::Encode for Burn2playRef<Abi>
        where
            <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>: ::ink::scale::Encode,
            <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>: ::ink::scale::Encode,
            core::marker::PhantomData<Abi>: ::ink::scale::Encode,
            core::marker::PhantomData<Abi>: ::ink::scale::Encode,
        {
            fn size_hint(&self) -> usize {
                0_usize
                    .saturating_add(::ink::scale::Encode::size_hint(&self.inner))
                    .saturating_add(::ink::scale::Encode::size_hint(&self._marker))
            }
            fn encode_to<__CodecOutputEdqy: ::ink::scale::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::ink::scale::Encode::encode_to(&self.inner, __codec_dest_edqy);
                ::ink::scale::Encode::encode_to(&self._marker, __codec_dest_edqy);
            }
        }
        #[automatically_derived]
        impl<Abi> ::ink::scale::EncodeLike for Burn2playRef<Abi>
        where
            <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>: ::ink::scale::Encode,
            <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>: ::ink::scale::Encode,
            core::marker::PhantomData<Abi>: ::ink::scale::Encode,
            core::marker::PhantomData<Abi>: ::ink::scale::Encode,
        {
        }
    };
    #[automatically_derived]
    impl<Abi: ::core::fmt::Debug> ::core::fmt::Debug for Burn2playRef<Abi> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Burn2playRef",
                "inner",
                &self.inner,
                "_marker",
                &&self._marker,
            )
        }
    }
    #[automatically_derived]
    impl<Abi: ::core::hash::Hash> ::core::hash::Hash for Burn2playRef<Abi> {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.inner, state);
            ::core::hash::Hash::hash(&self._marker, state)
        }
    }
    #[automatically_derived]
    impl<Abi> ::core::marker::StructuralPartialEq for Burn2playRef<Abi> {}
    #[automatically_derived]
    impl<Abi: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Burn2playRef<Abi> {
        #[inline]
        fn eq(&self, other: &Burn2playRef<Abi>) -> bool {
            self.inner == other.inner && self._marker == other._marker
        }
    }
    #[automatically_derived]
    impl<Abi: ::core::cmp::Eq> ::core::cmp::Eq for Burn2playRef<Abi> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<
                <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>,
            >;
            let _: ::core::cmp::AssertParamIsEq<core::marker::PhantomData<Abi>>;
        }
    }
    #[automatically_derived]
    impl<Abi: ::core::clone::Clone> ::core::clone::Clone for Burn2playRef<Abi> {
        #[inline]
        fn clone(&self) -> Burn2playRef<Abi> {
            Burn2playRef {
                inner: ::core::clone::Clone::clone(&self.inner),
                _marker: ::core::clone::Clone::clone(&self._marker),
            }
        }
    }
    const _: () = {
        impl ::ink::env::ContractReference for Burn2play {
            type Type = Burn2playRef;
        }
        impl<Abi> ::ink::env::ContractReverseReference for Burn2playRef<Abi> {
            type Type = Burn2play;
        }
        impl ::ink::env::call::ConstructorReturnType<Burn2playRef> for Burn2play {
            type Output = Burn2playRef;
            type Error = ();
            fn ok(value: Burn2playRef) -> Self::Output {
                value
            }
        }
        impl<E> ::ink::env::call::ConstructorReturnType<Burn2playRef>
            for ::core::result::Result<Burn2play, E>
        where
            E: ::ink::scale::Decode,
        {
            const IS_RESULT: bool = true;
            type Output = ::core::result::Result<Burn2playRef, E>;
            type Error = E;
            fn ok(value: Burn2playRef) -> Self::Output {
                ::core::result::Result::Ok(value)
            }
            fn err(err: Self::Error) -> ::core::option::Option<Self::Output> {
                ::core::option::Option::Some(::core::result::Result::Err(err))
            }
        }
        impl<Abi> ::ink::env::ContractEnv for Burn2playRef<Abi> {
            type Env = <Burn2play as ::ink::env::ContractEnv>::Env;
        }
        impl<Abi> ::ink::storage::traits::StorageLayout for Burn2playRef<Abi> {
            fn layout(__key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(
                    ::ink::metadata::layout::StructLayout::new(
                        "Burn2playRef",
                        [
                            ::ink::metadata::layout::FieldLayout::new(
                                "inner",
                                <<Burn2play as ::ink::codegen::ContractCallBuilder>::Type<
                                    Abi,
                                > as ::ink::storage::traits::StorageLayout>::layout(__key),
                            ),
                        ],
                    ),
                )
            }
        }
        impl<Abi> ::ink::scale_info::TypeInfo for Burn2playRef<Abi>
        where
            ::ink::Address: ::ink::scale_info::TypeInfo + 'static,
        {
            type Identity = <<Burn2play as ::ink::codegen::ContractCallBuilder>::Type<
                Abi,
            > as ::ink::scale_info::TypeInfo>::Identity;
            fn type_info() -> ::ink::scale_info::Type {
                <<Burn2play as ::ink::codegen::ContractCallBuilder>::Type<
                    Abi,
                > as ::ink::scale_info::TypeInfo>::type_info()
            }
        }
    };
    impl Burn2playRef<::ink::abi::Ink> {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[inline]
        #[allow(clippy::type_complexity)]
        pub fn new(
            __ink_binding_0: u128,
            __ink_binding_1: u32,
            __ink_binding_2: BlockNumber,
            __ink_binding_3: u128,
        ) -> ::ink::env::call::CreateBuilder<
            Environment,
            Self,
            ::ink::env::call::utils::Set<::ink::env::call::LimitParamsV2>,
            ::ink::env::call::utils::Set<
                ::ink::env::call::ExecutionInput<
                    ::ink::env::call::utils::ArgumentList<
                        ::ink::env::call::utils::Argument<u128>,
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<BlockNumber>,
                            ::ink::env::call::utils::ArgumentList<
                                ::ink::env::call::utils::Argument<u32>,
                                ::ink::env::call::utils::ArgumentList<
                                    ::ink::env::call::utils::Argument<u128>,
                                    ::ink::env::call::utils::EmptyArgumentList<::ink::abi::Ink>,
                                    ::ink::abi::Ink,
                                >,
                                ::ink::abi::Ink,
                            >,
                            ::ink::abi::Ink,
                        >,
                        ::ink::abi::Ink,
                    >,
                    ::ink::abi::Ink,
                >,
            >,
            ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<Self>>,
        > {
            ::ink::env::call::build_create_abi::<Self, ::ink::abi::Ink>()
                .exec_input(
                    ::ink::env::call::ExecutionInput::new(::ink::env::call::Selector::new([
                        0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8,
                    ]))
                    .push_arg(__ink_binding_0)
                    .push_arg(__ink_binding_1)
                    .push_arg(__ink_binding_2)
                    .push_arg(__ink_binding_3),
                )
                .returns::<Self>()
        }
        #[inline]
        pub fn get_pseudo_random(&mut self) -> u128 {
            self.try_get_pseudo_random().unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "encountered error while calling {0}::{1}: {2:?}",
                    "Burn2play", "get_pseudo_random", error,
                ));
            })
        }
        #[inline]
        pub fn try_get_pseudo_random(&mut self) -> ::ink::MessageResult<u128> {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .get_pseudo_random()
                .try_invoke()
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Burn2play", "get_pseudo_random", error,
                    ));
                })
        }
        #[inline]
        pub fn burn_and_play(&mut self) {
            self.try_burn_and_play().unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "encountered error while calling {0}::{1}: {2:?}",
                    "Burn2play", "burn_and_play", error,
                ));
            })
        }
        #[inline]
        pub fn try_burn_and_play(&mut self) -> ::ink::MessageResult<()> {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .burn_and_play()
                .try_invoke()
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Burn2play", "burn_and_play", error,
                    ));
                })
        }
        #[inline]
        pub fn claim(&mut self) {
            self.try_claim().unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "encountered error while calling {0}::{1}: {2:?}",
                    "Burn2play", "claim", error,
                ));
            })
        }
        #[inline]
        pub fn try_claim(&mut self) -> ::ink::MessageResult<()> {
            <Self as ::ink::codegen::TraitCallBuilder>::call_mut(self)
                .claim()
                .try_invoke()
                .unwrap_or_else(|error| {
                    ::core::panicking::panic_fmt(format_args!(
                        "encountered error while calling {0}::{1}: {2:?}",
                        "Burn2play", "claim", error,
                    ));
                })
        }
    }
    const _: () = {
        impl<Abi> ::ink::codegen::TraitCallBuilder for Burn2playRef<Abi> {
            type Builder = <Burn2play as ::ink::codegen::ContractCallBuilder>::Type<Abi>;
            #[inline]
            fn call(&self) -> &Self::Builder {
                &self.inner
            }
            #[inline]
            fn call_mut(&mut self) -> &mut Self::Builder {
                &mut self.inner
            }
        }
    };
    impl<Abi> ::ink::env::call::FromAddr for Burn2playRef<Abi> {
        #[inline]
        fn from_addr(addr: ::ink::Address) -> Self {
            Self {
                inner: <<Burn2play as ::ink::codegen::ContractCallBuilder>::Type<
                    Abi,
                > as ::ink::env::call::FromAddr>::from_addr(addr),
                _marker: ::core::default::Default::default(),
            }
        }
    }
    impl<Abi> ::ink::ToAddr for Burn2playRef<Abi> {
        #[inline]
        fn to_addr(&self) -> ::ink::Address {
            <<Burn2play as ::ink::codegen::ContractCallBuilder>::Type<
                Abi,
            > as ::ink::ToAddr>::to_addr(&self.inner)
        }
    }
    impl<Abi> ::core::convert::AsRef<::ink::Address> for Burn2playRef<Abi> {
        fn as_ref(&self) -> &::ink::Address {
            <_ as ::core::convert::AsRef<::ink::Address>>::as_ref(&self.inner)
        }
    }
    impl<Abi> ::core::convert::AsMut<::ink::Address> for Burn2playRef<Abi> {
        fn as_mut(&mut self) -> &mut ::ink::Address {
            <_ as ::core::convert::AsMut<::ink::Address>>::as_mut(&mut self.inner)
        }
    }
    const _: () = {
        #[no_mangle]
        pub fn __ink_generate_metadata() -> ::ink::metadata::InkProject {
            let layout =
                ::ink::metadata::layout::Layout::Root(::ink::metadata::layout::RootLayout::new(
                    <::ink::metadata::layout::LayoutKey as ::core::convert::From<
                        ::ink::primitives::Key,
                    >>::from(
                        <Burn2play as ::ink::storage::traits::StorageKey>::KEY
                    ),
                    <Burn2play as ::ink::storage::traits::StorageLayout>::layout(
                        &<Burn2play as ::ink::storage::traits::StorageKey>::KEY,
                    ),
                    ::ink::scale_info::meta_type::<Burn2play>(),
                ));
            ::ink::metadata::layout::ValidateLayout::validate(&layout).unwrap_or_else(|error| {
                ::core::panicking::panic_fmt(format_args!(
                    "metadata ink! generation failed: {0}",
                    error
                ));
            });
            ::ink::metadata::InkProject::new(
                layout,
                ::ink::metadata::ContractSpec::new()
                    .constructors([
                        ::ink::metadata::ConstructorSpec::from_label("new")
                            .selector([0x9B_u8, 0xAE_u8, 0x9D_u8, 0x5E_u8])
                            .args([
                                ::ink::metadata::MessageParamSpec::new("ticket_price")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u128,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u128"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                                ::ink::metadata::MessageParamSpec::new("burn_perbill")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u32,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u32"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                                ::ink::metadata::MessageParamSpec::new("closes")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            BlockNumber,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["BlockNumber"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                                ::ink::metadata::MessageParamSpec::new("claim_fee")
                                    .of_type(
                                        ::ink::metadata::TypeSpec::with_name_segs::<
                                            u128,
                                            _,
                                        >(
                                            ::core::iter::Iterator::map(
                                                ::core::iter::IntoIterator::into_iter(["u128"]),
                                                ::core::convert::AsRef::as_ref,
                                            ),
                                        ),
                                    )
                                    .done(),
                            ])
                            .payable(true)
                            .default(false)
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    if <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                                        2611912030u32,
                                    >>::IS_RESULT {
                                        ::ink::metadata::TypeSpec::with_name_str::<
                                            ::ink::ConstructorResult<
                                                ::core::result::Result<
                                                    (),
                                                    <Burn2play as ::ink::reflect::DispatchableConstructorInfo<
                                                        2611912030u32,
                                                    >>::Error,
                                                >,
                                            >,
                                        >("ink_primitives::ConstructorResult")
                                    } else {
                                        ::ink::metadata::TypeSpec::with_name_str::<
                                            ::ink::ConstructorResult<()>,
                                        >("ink_primitives::ConstructorResult")
                                    },
                                ),
                            )
                            .docs([
                                " Constructor that initializes the `bool` value to the given `init_value`.",
                            ])
                            .done(),
                    ])
                    .messages([
                        ::ink::metadata::MessageSpec::from_label("get_pseudo_random")
                            .selector([0x18_u8, 0xA1_u8, 0xC3_u8, 0x42_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<u128>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(true)
                            .payable(false)
                            .default(false)
                            .docs([])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("burn_and_play")
                            .selector([0x9B_u8, 0x1A_u8, 0x3A_u8, 0x45_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<()>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(true)
                            .payable(true)
                            .default(false)
                            .docs([])
                            .done(),
                        ::ink::metadata::MessageSpec::from_label("claim")
                            .selector([0xB3_u8, 0x88_u8, 0x80_u8, 0x3F_u8])
                            .args([])
                            .returns(
                                ::ink::metadata::ReturnTypeSpec::new(
                                    ::ink::metadata::TypeSpec::with_name_segs::<
                                        ::ink::MessageResult<()>,
                                        _,
                                    >(
                                        ::core::iter::Iterator::map(
                                            ::core::iter::IntoIterator::into_iter([
                                                "ink",
                                                "MessageResult",
                                            ]),
                                            ::core::convert::AsRef::as_ref,
                                        ),
                                    ),
                                ),
                            )
                            .mutates(true)
                            .payable(false)
                            .default(false)
                            .docs([])
                            .done(),
                    ])
                    .events(::ink::collect_events())
                    .docs([])
                    .lang_error(
                        ::ink::metadata::TypeSpec::with_name_segs::<
                            ::ink::LangError,
                            _,
                        >(
                            ::core::iter::Iterator::map(
                                ::core::iter::IntoIterator::into_iter(["ink", "LangError"]),
                                ::core::convert::AsRef::as_ref,
                            ),
                        ),
                    )
                    .environment(
                        ::ink::metadata::EnvironmentSpec::new()
                            .account_id(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    AccountId,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["AccountId"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .balance(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Balance,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Balance"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .hash(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Hash,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Hash"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .timestamp(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    Timestamp,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["Timestamp"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .block_number(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    BlockNumber,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["BlockNumber"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .chain_extension(
                                ::ink::metadata::TypeSpec::with_name_segs::<
                                    ChainExtension,
                                    _,
                                >(
                                    ::core::iter::Iterator::map(
                                        ::core::iter::IntoIterator::into_iter(["ChainExtension"]),
                                        ::core::convert::AsRef::as_ref,
                                    ),
                                ),
                            )
                            .max_event_topics(MAX_EVENT_TOPICS)
                            .static_buffer_size(::ink::env::BUFFER_SIZE)
                            .done(),
                    )
                    .done(),
            )
        }
    };
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
    pub trait Burn: ::ink::env::ContractEnv {
        /// Holds general and global information about the trait.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        type __ink_TraitInfo: ::ink::codegen::TraitCallForwarder;
        /// Output type of the respective trait message.
        type burnOutput: ::ink::codegen::ImpliesReturn<u32>;
        #[allow(non_snake_case)]
        fn burn(&self, value: u128, keep_alive: bool) -> Self::burnOutput;
    }
    const _: () = {
        impl<E> Burn for ::ink::reflect::TraitDefinitionRegistry<E>
        where
            E: ::ink::env::Environment,
        {
            /// Holds general and global information about the trait.
            #[allow(non_camel_case_types)]
            type __ink_TraitInfo = __ink_TraitInfoBurn<E>;
            type burnOutput = u32;
            #[allow(non_snake_case)]
            #[cold]
            fn burn(&self, __ink_binding_0: u128, __ink_binding_1: bool) -> Self::burnOutput {
                ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<u128>>();
                ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchInput<bool>>();
                ::ink::codegen::utils::consume_type::<::ink::codegen::DispatchOutput<u32>>();
                /// We enforce linking errors in case this is ever actually called.
                /// These linker errors are properly resolved by the cargo-contract tool.
                extern "C" {
                    fn __ink_enforce_error_0x01104275726e106275726e8f22105e00() -> !;
                }
                unsafe { __ink_enforce_error_0x01104275726e106275726e8f22105e00() }
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct __ink_TraitInfoBurn<E> {
            marker: ::core::marker::PhantomData<fn() -> E>,
        }
        impl<E> ::ink::reflect::TraitMessageInfo<2985279867u32> for __ink_TraitInfoBurn<E> {
            const PAYABLE: ::core::primitive::bool = false;
            const SELECTOR: [::core::primitive::u8; 4usize] = [0x8F_u8, 0x22_u8, 0x10_u8, 0x5E_u8];
        }
        impl<E> ::ink::reflect::TraitInfo for __ink_TraitInfoBurn<E>
        where
            E: ::ink::env::Environment,
        {
            const ID: u32 = 2401374302;
            const PATH: &'static ::core::primitive::str = "burn2play::burn2play";
            const NAME: &'static ::core::primitive::str = "Burn";
        }
        impl<E> ::ink::codegen::TraitCallForwarder for __ink_TraitInfoBurn<E>
        where
            E: ::ink::env::Environment,
        {
            type Forwarder<Abi> = __ink_TraitCallForwarderBurn<E, Abi>;
        }
        impl<E> ::ink::codegen::TraitMessageBuilder for __ink_TraitInfoBurn<E>
        where
            E: ::ink::env::Environment,
        {
            type MessageBuilder<Abi> = __ink_TraitMessageBuilderBurn<E, Abi>;
        }
        #[codec(crate = ::ink::scale)]
        #[scale_info(crate = ::ink::scale_info)]
        /// The global call builder type for all trait implementers.
        ///
        /// All calls to types (contracts) implementing the trait will be built by this type.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct __ink_TraitMessageBuilderBurn<E, Abi> {
            _marker: ::core::marker::PhantomData<fn() -> (E, Abi)>,
        }
        #[allow(
            non_upper_case_globals,
            deprecated,
            unused_attributes,
            unused_qualifications
        )]
        const _: () = {
            impl<E, Abi> ::ink::scale_info::TypeInfo for __ink_TraitMessageBuilderBurn<E, Abi>
            where
                ::core::marker::PhantomData<fn() -> (E, Abi)>:
                    ::ink::scale_info::TypeInfo + 'static,
                E: ::ink::scale_info::TypeInfo + 'static,
                Abi: ::ink::scale_info::TypeInfo + 'static,
            {
                type Identity = Self;
                fn type_info() -> ::ink::scale_info::Type {
                    ::ink::scale_info::Type::builder()
                        .path(
                            ::ink::scale_info::Path::new_with_replace(
                                "__ink_TraitMessageBuilderBurn",
                                "burn2play::burn2play",
                                &[],
                            ),
                        )
                        .type_params(
                            <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    ::ink::scale_info::TypeParameter::new(
                                        "E",
                                        ::core::option::Option::Some(
                                            ::ink::scale_info::meta_type::<E>(),
                                        ),
                                    ),
                                    ::ink::scale_info::TypeParameter::new(
                                        "Abi",
                                        ::core::option::Option::Some(
                                            ::ink::scale_info::meta_type::<Abi>(),
                                        ),
                                    ),
                                ]),
                            ),
                        )
                        .docs(
                            &[
                                "The global call builder type for all trait implementers.",
                                "",
                                "All calls to types (contracts) implementing the trait will be built by this type.",
                            ],
                        )
                        .composite(
                            ::ink::scale_info::build::Fields::named()
                                .field(|f| {
                                    f
                                        .ty::<::core::marker::PhantomData<fn() -> (E, Abi)>>()
                                        .name("_marker")
                                        .type_name("::core::marker::PhantomData<fn() ->(E, Abi)>")
                                }),
                        )
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::Decode for __ink_TraitMessageBuilderBurn<E, Abi>
            where
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Decode,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Decode,
            {
                fn decode<__CodecInputEdqy: ::ink::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::ink::scale::Error> {
                    ::core::result::Result::Ok(__ink_TraitMessageBuilderBurn::<E, Abi> {
                        _marker: {
                            let __codec_res_edqy = <::core::marker::PhantomData<
                                fn() -> (E, Abi),
                            > as ::ink::scale::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `__ink_TraitMessageBuilderBurn::_marker`",
                                    ));
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    })
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::Encode for __ink_TraitMessageBuilderBurn<E, Abi>
            where
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Encode,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Encode,
            {
                fn size_hint(&self) -> usize {
                    ::ink::scale::Encode::size_hint(&&self._marker)
                }
                fn encode_to<__CodecOutputEdqy: ::ink::scale::Output + ?::core::marker::Sized>(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    ::ink::scale::Encode::encode_to(&&self._marker, __codec_dest_edqy)
                }
                fn encode(&self) -> ::ink::scale::alloc::vec::Vec<::core::primitive::u8> {
                    ::ink::scale::Encode::encode(&&self._marker)
                }
                fn using_encoded<
                    __CodecOutputReturn,
                    __CodecUsingEncodedCallback: ::core::ops::FnOnce(&[::core::primitive::u8]) -> __CodecOutputReturn,
                >(
                    &self,
                    f: __CodecUsingEncodedCallback,
                ) -> __CodecOutputReturn {
                    ::ink::scale::Encode::using_encoded(&&self._marker, f)
                }
            }
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::EncodeLike for __ink_TraitMessageBuilderBurn<E, Abi>
            where
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Encode,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Encode,
            {
            }
        };
        impl<E, Abi> ::core::default::Default for __ink_TraitMessageBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            fn default() -> Self {
                Self {
                    _marker: ::core::default::Default::default(),
                }
            }
        }
        impl<E, Abi> ::ink::env::ContractEnv for __ink_TraitMessageBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            type Env = E;
        }
        impl<E> Burn for __ink_TraitMessageBuilderBurn<E, ::ink::abi::Ink>
        where
            E: ::ink::env::Environment,
        {
            #[allow(non_camel_case_types)]
            type __ink_TraitInfo = __ink_TraitInfoBurn<E>;
            type burnOutput = ::ink::env::call::Execution<
                ::ink::env::call::utils::ArgumentList<
                    ::ink::env::call::utils::Argument<bool>,
                    ::ink::env::call::utils::ArgumentList<
                        ::ink::env::call::utils::Argument<u128>,
                        ::ink::env::call::utils::EmptyArgumentList<::ink::abi::Ink>,
                        ::ink::abi::Ink,
                    >,
                    ::ink::abi::Ink,
                >,
                u32,
                ::ink::abi::Ink,
            >;
            #[allow(non_snake_case)]
            #[inline]
            fn burn(&self, __ink_binding_0: u128, __ink_binding_1: bool) -> Self::burnOutput {
                ::ink::env::call::Execution::new(
                    ::ink::env::call::ExecutionInput::new(::ink::env::call::Selector::new([
                        0x8F_u8, 0x22_u8, 0x10_u8, 0x5E_u8,
                    ]))
                    .push_arg(__ink_binding_0)
                    .push_arg(__ink_binding_1),
                )
            }
        }
        #[codec(crate = ::ink::scale)]
        /// The global call builder type for all trait implementers.
        ///
        /// All calls to types (contracts) implementing the trait will be built by this type.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            addr: ::ink::Address,
            _marker: ::core::marker::PhantomData<fn() -> (E, Abi)>,
        }
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::Decode for __ink_TraitCallBuilderBurn<E, Abi>
            where
                E: ::ink::env::Environment,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Decode,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Decode,
            {
                fn decode<__CodecInputEdqy: ::ink::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::ink::scale::Error> {
                    ::core::result::Result::Ok(__ink_TraitCallBuilderBurn::<E, Abi> {
                        addr: {
                            let __codec_res_edqy = <::ink::Address as ::ink::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `__ink_TraitCallBuilderBurn::addr`",
                                    ));
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        _marker: {
                            let __codec_res_edqy = <::core::marker::PhantomData<
                                fn() -> (E, Abi),
                            > as ::ink::scale::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `__ink_TraitCallBuilderBurn::_marker`",
                                    ));
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    })
                }
                fn decode_into<__CodecInputEdqy: ::ink::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                    dst_: &mut ::core::mem::MaybeUninit<Self>,
                ) -> ::core::result::Result<::ink::scale::DecodeFinished, ::ink::scale::Error>
                {
                    match (
                        &(::core::mem::size_of::<::ink::Address>()
                            + ::core::mem::size_of::<::core::marker::PhantomData<fn() -> (E, Abi)>>(
                            )),
                        &::core::mem::size_of::<Self>(),
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    if !(if ::core::mem::size_of::<::ink::Address>() > 0 {
                        1
                    } else {
                        0
                    } + if ::core::mem::size_of::<::core::marker::PhantomData<fn() -> (E, Abi)>>(
                    ) > 0
                    {
                        1
                    } else {
                        0
                    } <= 1)
                    {
                        ::core::panicking::panic(
                            "assertion failed: if ::core::mem::size_of::<::ink::Address>() > 0 { 1 } else { 0 } +\n        if ::core::mem::size_of::<::core::marker::PhantomData<fn()\n                            -> (E, Abi)>>() > 0 {\n            1\n        } else { 0 } <= 1",
                        )
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<::ink::Address> = unsafe {
                            &mut *dst_
                                .as_mut_ptr()
                                .cast::<::core::mem::MaybeUninit<::ink::Address>>()
                        };
                        <::ink::Address as ::ink::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<
                            ::core::marker::PhantomData<fn() -> (E, Abi)>,
                        > = unsafe {
                            &mut *dst_.as_mut_ptr().cast::<::core::mem::MaybeUninit<
                                ::core::marker::PhantomData<fn() -> (E, Abi)>,
                            >>()
                        };
                        <::core::marker::PhantomData<
                            fn() -> (E, Abi),
                        > as ::ink::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    unsafe {
                        ::core::result::Result::Ok(
                            ::ink::scale::DecodeFinished::assert_decoding_finished(),
                        )
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::Encode for __ink_TraitCallBuilderBurn<E, Abi>
            where
                E: ::ink::env::Environment,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Encode,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Encode,
            {
                fn size_hint(&self) -> usize {
                    0_usize
                        .saturating_add(::ink::scale::Encode::size_hint(&self.addr))
                        .saturating_add(::ink::scale::Encode::size_hint(&self._marker))
                }
                fn encode_to<__CodecOutputEdqy: ::ink::scale::Output + ?::core::marker::Sized>(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    ::ink::scale::Encode::encode_to(&self.addr, __codec_dest_edqy);
                    ::ink::scale::Encode::encode_to(&self._marker, __codec_dest_edqy);
                }
            }
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::EncodeLike for __ink_TraitCallBuilderBurn<E, Abi>
            where
                E: ::ink::env::Environment,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Encode,
                ::core::marker::PhantomData<fn() -> (E, Abi)>: ::ink::scale::Encode,
            {
            }
        };
        impl<E, Abi> ::ink::storage::traits::StorageLayout for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::ink::storage::traits::StorageLayout,
        {
            fn layout(__key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                ::ink::metadata::layout::Layout::Struct(::ink::metadata::layout::StructLayout::new(
                    "__ink_TraitCallBuilderBurn",
                    [::ink::metadata::layout::FieldLayout::new(
                        "addr",
                        <::ink::Address as ::ink::storage::traits::StorageLayout>::layout(__key),
                    )],
                ))
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E, Abi> ::core::clone::Clone for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::core::clone::Clone,
        {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    addr: ::core::clone::Clone::clone(&self.addr),
                    _marker: self._marker,
                }
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E, Abi> ::core::fmt::Debug for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::core::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct("__ink_TraitCallBuilderBurn")
                    .field("addr", &self.addr)
                    .finish()
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E, Abi> ::ink::scale_info::TypeInfo for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::ink::scale_info::TypeInfo + 'static,
        {
            type Identity = ::ink::Address;
            fn type_info() -> ::ink::scale_info::Type {
                <::ink::Address as ::ink::scale_info::TypeInfo>::type_info()
            }
        }
        impl<E, Abi> ::ink::env::call::FromAddr for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            #[inline]
            fn from_addr(addr: ::ink::Address) -> Self {
                Self {
                    addr,
                    _marker: ::core::default::Default::default(),
                }
            }
        }
        impl<E, Abi> ::core::convert::From<::ink::Address> for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::ink::env::AccountIdGuard,
        {
            fn from(value: ::ink::Address) -> Self {
                <Self as ::ink::env::call::FromAddr>::from_addr(value)
            }
        }
        impl<E, Abi> ::ink::ToAddr for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            #[inline]
            fn to_addr(&self) -> ::ink::Address {
                <::ink::Address as ::core::clone::Clone>::clone(&self.addr)
            }
        }
        impl<E, Abi> ::core::convert::AsRef<::ink::Address> for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            fn as_ref(&self) -> &::ink::Address {
                &self.addr
            }
        }
        impl<E, Abi> ::core::convert::AsMut<::ink::Address> for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            fn as_mut(&mut self) -> &mut ::ink::Address {
                &mut self.addr
            }
        }
        impl<E, Abi> ::ink::env::ContractEnv for __ink_TraitCallBuilderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            type Env = E;
        }
        /// This trait allows to bridge from the call builder to message builder.
        impl<E, TypeAbi> ::ink::codegen::TraitMessageBuilder for __ink_TraitCallBuilderBurn<E, TypeAbi>
        where
            E: ::ink::env::Environment,
        {
            type MessageBuilder<TraitAbi> = __ink_TraitMessageBuilderBurn<E, TypeAbi>;
        }
        impl<E> Burn for __ink_TraitCallBuilderBurn<E, ::ink::abi::Ink>
        where
            E: ::ink::env::Environment,
        {
            #[allow(non_camel_case_types)]
            type __ink_TraitInfo = __ink_TraitInfoBurn<E>;
            #[allow(clippy::type_complexity)]
            type burnOutput = ::ink::env::call::CallBuilder<
                Self::Env,
                ::ink::env::call::utils::Set<::ink::env::call::Call>,
                ::ink::env::call::utils::Set<
                    ::ink::env::call::ExecutionInput<
                        ::ink::env::call::utils::ArgumentList<
                            ::ink::env::call::utils::Argument<bool>,
                            ::ink::env::call::utils::ArgumentList<
                                ::ink::env::call::utils::Argument<u128>,
                                ::ink::env::call::utils::EmptyArgumentList<::ink::abi::Ink>,
                                ::ink::abi::Ink,
                            >,
                            ::ink::abi::Ink,
                        >,
                        ::ink::abi::Ink,
                    >,
                >,
                ::ink::env::call::utils::Set<::ink::env::call::utils::ReturnType<u32>>,
            >;
            #[allow(non_snake_case)]
            #[inline]
            fn burn(&self, __ink_binding_0: u128, __ink_binding_1: bool) -> Self::burnOutput {
                <::ink::env::call::CallBuilder<
                    Self::Env,
                    ::ink::env::call::utils::Unset<::ink::env::call::Call>,
                    ::ink::env::call::utils::Set<
                        ::ink::env::call::ExecutionInput<
                            ::ink::env::call::utils::ArgumentList<
                                ::ink::env::call::utils::Argument<bool>,
                                ::ink::env::call::utils::ArgumentList<
                                    ::ink::env::call::utils::Argument<u128>,
                                    ::ink::env::call::utils::EmptyArgumentList<::ink::abi::Ink>,
                                    ::ink::abi::Ink,
                                >,
                                ::ink::abi::Ink,
                            >,
                            ::ink::abi::Ink,
                        >,
                    >,
                    ::ink::env::call::utils::Set<
                        ::ink::env::call::utils::ReturnType<u32>,
                    >,
                > as ::core::convert::From<
                    _,
                >>::from(
                        <<Self as ::ink::codegen::TraitMessageBuilder>::MessageBuilder<
                            ::ink::abi::Ink,
                        > as Burn>::burn(
                            &<<Self as ::ink::codegen::TraitMessageBuilder>::MessageBuilder<
                                ::ink::abi::Ink,
                            > as ::core::default::Default>::default(),
                            __ink_binding_0,
                            __ink_binding_1,
                        ),
                    )
                    .call(::ink::ToAddr::to_addr(self))
            }
        }
        #[codec(crate = ::ink::scale)]
        /// The global call forwarder for the ink! trait definition.
        ///
        /// All cross-contract calls to contracts implementing the associated ink! trait
        /// will be handled by this type.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[repr(transparent)]
        pub struct __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            builder: <Self as ::ink::codegen::TraitCallBuilder>::Builder,
            _marker: ::core::marker::PhantomData<fn() -> Abi>,
        }
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::Decode for __ink_TraitCallForwarderBurn<E, Abi>
            where
                E: ::ink::env::Environment,
                ::core::marker::PhantomData<fn() -> Abi>: ::ink::scale::Decode,
                ::core::marker::PhantomData<fn() -> Abi>: ::ink::scale::Decode,
            {
                fn decode<__CodecInputEdqy: ::ink::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::ink::scale::Error> {
                    ::core::result::Result::Ok(__ink_TraitCallForwarderBurn::<E, Abi> {
                        builder: {
                            let __codec_res_edqy = <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::scale::Decode>::decode(
                                __codec_input_edqy,
                            );
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `__ink_TraitCallForwarderBurn::builder`",
                                    ));
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        _marker: {
                            let __codec_res_edqy = <::core::marker::PhantomData<
                                fn() -> Abi,
                            > as ::ink::scale::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                ::core::result::Result::Err(e) => {
                                    return ::core::result::Result::Err(e.chain(
                                        "Could not decode `__ink_TraitCallForwarderBurn::_marker`",
                                    ));
                                }
                                ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    })
                }
                fn decode_into<__CodecInputEdqy: ::ink::scale::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                    dst_: &mut ::core::mem::MaybeUninit<Self>,
                ) -> ::core::result::Result<::ink::scale::DecodeFinished, ::ink::scale::Error>
                {
                    match (
                        &(::core::mem::size_of::<
                            <Self as ::ink::codegen::TraitCallBuilder>::Builder,
                        >() + ::core::mem::size_of::<::core::marker::PhantomData<fn() -> Abi>>()),
                        &::core::mem::size_of::<Self>(),
                    ) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                    if !(if ::core::mem::size_of::<
                        <Self as ::ink::codegen::TraitCallBuilder>::Builder,
                    >() > 0
                    {
                        1
                    } else {
                        0
                    } + if ::core::mem::size_of::<::core::marker::PhantomData<fn() -> Abi>>()
                        > 0
                    {
                        1
                    } else {
                        0
                    } <= 1)
                    {
                        ::core::panicking::panic(
                            "assertion failed: if ::core::mem::size_of::<<Self as\n                        ::ink::codegen::TraitCallBuilder>::Builder>() > 0 {\n            1\n        } else { 0 } +\n        if ::core::mem::size_of::<::core::marker::PhantomData<fn() -> Abi>>()\n                > 0 {\n            1\n        } else { 0 } <= 1",
                        )
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<
                            <Self as ::ink::codegen::TraitCallBuilder>::Builder,
                        > = unsafe {
                            &mut *dst_.as_mut_ptr().cast::<::core::mem::MaybeUninit<
                                <Self as ::ink::codegen::TraitCallBuilder>::Builder,
                            >>()
                        };
                        <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    {
                        let dst_: &mut ::core::mem::MaybeUninit<Self> = dst_;
                        let dst_: &mut ::core::mem::MaybeUninit<
                            ::core::marker::PhantomData<fn() -> Abi>,
                        > =
                            unsafe {
                                &mut *dst_.as_mut_ptr().cast::<::core::mem::MaybeUninit<
                                    ::core::marker::PhantomData<fn() -> Abi>,
                                >>()
                            };
                        <::core::marker::PhantomData<
                            fn() -> Abi,
                        > as ::ink::scale::Decode>::decode_into(
                            __codec_input_edqy,
                            dst_,
                        )?;
                    }
                    unsafe {
                        ::core::result::Result::Ok(
                            ::ink::scale::DecodeFinished::assert_decoding_finished(),
                        )
                    }
                }
            }
        };
        #[allow(deprecated)]
        const _: () = {
            #[allow(non_camel_case_types)]
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::Encode for __ink_TraitCallForwarderBurn<E, Abi>
            where
                E: ::ink::env::Environment,
                ::core::marker::PhantomData<fn() -> Abi>: ::ink::scale::Encode,
                ::core::marker::PhantomData<fn() -> Abi>: ::ink::scale::Encode,
            {
                fn size_hint(&self) -> usize {
                    0_usize
                        .saturating_add(::ink::scale::Encode::size_hint(&self.builder))
                        .saturating_add(::ink::scale::Encode::size_hint(&self._marker))
                }
                fn encode_to<__CodecOutputEdqy: ::ink::scale::Output + ?::core::marker::Sized>(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    ::ink::scale::Encode::encode_to(&self.builder, __codec_dest_edqy);
                    ::ink::scale::Encode::encode_to(&self._marker, __codec_dest_edqy);
                }
            }
            #[automatically_derived]
            impl<E, Abi> ::ink::scale::EncodeLike for __ink_TraitCallForwarderBurn<E, Abi>
            where
                E: ::ink::env::Environment,
                ::core::marker::PhantomData<fn() -> Abi>: ::ink::scale::Encode,
                ::core::marker::PhantomData<fn() -> Abi>: ::ink::scale::Encode,
            {
            }
        };
        impl<E, Abi> ::ink::storage::traits::StorageLayout for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::ink::storage::traits::StorageLayout,
        {
            fn layout(__key: &::ink::primitives::Key) -> ::ink::metadata::layout::Layout {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::storage::traits::StorageLayout>::layout(
                    __key,
                )
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E, Abi> ::core::clone::Clone for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::core::clone::Clone,
        {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    builder: <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::core::clone::Clone>::clone(
                        &self.builder,
                    ),
                    _marker: self._marker,
                }
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E, Abi> ::core::fmt::Debug for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::core::fmt::Debug,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct("__ink_TraitCallForwarderBurn")
                    .field("addr", &self.builder.addr)
                    .finish()
            }
        }
        /// We require this manual implementation since the derive produces incorrect trait bounds.
        impl<E, Abi> ::ink::scale_info::TypeInfo for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
            ::ink::Address: ::ink::scale_info::TypeInfo + 'static,
        {
            type Identity = <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::scale_info::TypeInfo>::Identity;
            fn type_info() -> ::ink::scale_info::Type {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::scale_info::TypeInfo>::type_info()
            }
        }
        impl<E, Abi> ::ink::env::call::FromAddr for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            #[inline]
            fn from_addr(addr: ::ink::Address) -> Self {
                Self {
                    builder: <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::env::call::FromAddr>::from_addr(
                        addr,
                    ),
                    _marker: ::core::default::Default::default(),
                }
            }
        }
        impl<E, Abi> ::core::convert::From<::ink::Address> for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            fn from(addr: ::ink::Address) -> Self {
                <Self as ::ink::env::call::FromAddr>::from_addr(addr)
            }
        }
        impl<E, Abi> ::ink::ToAddr for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            #[inline]
            fn to_addr(&self) -> ::ink::Address {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as ::ink::ToAddr>::to_addr(
                    &self.builder,
                )
            }
        }
        impl<E, Abi> ::core::convert::AsRef<::ink::Address> for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            fn as_ref(&self) -> &::ink::Address {
                <_ as ::core::convert::AsRef<::ink::Address>>::as_ref(&self.builder)
            }
        }
        impl<E, Abi> ::core::convert::AsMut<::ink::Address> for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            fn as_mut(&mut self) -> &mut ::ink::Address {
                <_ as ::core::convert::AsMut<::ink::Address>>::as_mut(&mut self.builder)
            }
        }
        impl<E, Abi> ::ink::env::ContractEnv for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            type Env = E;
        }
        /// This trait allows to bridge from call forwarder to call builder.
        ///
        /// Also this explains why we designed the generated code so that we have
        /// both types and why the forwarder is a thin-wrapper around the builder
        /// as this allows to perform this operation safely.
        impl<E, Abi> ::ink::codegen::TraitCallBuilder for __ink_TraitCallForwarderBurn<E, Abi>
        where
            E: ::ink::env::Environment,
        {
            type Builder = __ink_TraitCallBuilderBurn<E, Abi>;
            #[inline]
            fn call(&self) -> &<Self as ::ink::codegen::TraitCallBuilder>::Builder {
                &self.builder
            }
            #[inline]
            fn call_mut(&mut self) -> &mut <Self as ::ink::codegen::TraitCallBuilder>::Builder {
                &mut self.builder
            }
        }
        impl<E> Burn for __ink_TraitCallForwarderBurn<E, ::ink::abi::Ink>
        where
            E: ::ink::env::Environment,
        {
            #[allow(non_camel_case_types)]
            type __ink_TraitInfo = __ink_TraitInfoBurn<E>;
            type burnOutput = u32;
            #[allow(non_snake_case)]
            #[inline]
            fn burn(&self, value: u128, keep_alive: bool) -> Self::burnOutput {
                <<Self as ::ink::codegen::TraitCallBuilder>::Builder as Burn>::burn(
                    <Self as ::ink::codegen::TraitCallBuilder>::call(self),
                    value,
                    keep_alive,
                )
                .try_invoke()
                .unwrap()
                .unwrap_or_else()
            }
        }
    };
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
    fn get_amount_to_burn(burn_perbill: u32, value: u128) -> u128 {
        value * u128::from(burn_perbill) / 1_000_000_000
    }
}
