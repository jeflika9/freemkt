#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use ink_lang as ink;
// use ::ink_env::{
//     DefaultEnvironment,
//     call::{build_create, Selector, ExecutionInput, FromAccountId}
// };

mod mod2 {
    impl ::ink_lang::ContractEnv for Mod2 {
        type Env = ::ink_env::DefaultEnvironment;
    }
    type Environment = <Mod2 as ::ink_lang::ContractEnv>::Env;
    type AccountId = <<Mod2 as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::AccountId;
    type Balance = <<Mod2 as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Balance;
    type Hash = <<Mod2 as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Hash;
    type Timestamp = <<Mod2 as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::Timestamp;
    type BlockNumber =
        <<Mod2 as ::ink_lang::ContractEnv>::Env as ::ink_env::Environment>::BlockNumber;
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        impl<'a> ::ink_lang::Env for &'a Mod2 {
            type EnvAccess = ::ink_lang::EnvAccess<'a, <Mod2 as ::ink_lang::ContractEnv>::Env>;
            fn env(self) -> Self::EnvAccess {
                Default::default()
            }
        }
        impl<'a> ::ink_lang::StaticEnv for Mod2 {
            type EnvAccess = ::ink_lang::EnvAccess<'static, <Mod2 as ::ink_lang::ContractEnv>::Env>;
            fn env() -> Self::EnvAccess {
                Default::default()
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    pub struct Mod2 {
        acct_id: AccountId,
    }
    const _: () = {
        impl ::ink_storage::traits::SpreadLayout for Mod2 {
            #[allow(unused_comparisons)]
            const FOOTPRINT: u64 = [
                (0u64 + <AccountId as ::ink_storage::traits::SpreadLayout>::FOOTPRINT),
                0u64,
            ][((0u64 + <AccountId as ::ink_storage::traits::SpreadLayout>::FOOTPRINT)
                < 0u64) as usize];
            const REQUIRES_DEEP_CLEAN_UP: bool = (false
                || (false
                    || <AccountId as ::ink_storage::traits::SpreadLayout>::REQUIRES_DEEP_CLEAN_UP));
            fn pull_spread(__key_ptr: &mut ::ink_storage::traits::KeyPtr) -> Self {
                Mod2 {
                    acct_id: <AccountId as ::ink_storage::traits::SpreadLayout>::pull_spread(
                        __key_ptr,
                    ),
                }
            }
            fn push_spread(&self, __key_ptr: &mut ::ink_storage::traits::KeyPtr) {
                match self {
                    Mod2 {
                        acct_id: __binding_0,
                    } => {
                        ::ink_storage::traits::SpreadLayout::push_spread(__binding_0, __key_ptr);
                    }
                }
            }
            fn clear_spread(&self, __key_ptr: &mut ::ink_storage::traits::KeyPtr) {
                match self {
                    Mod2 {
                        acct_id: __binding_0,
                    } => {
                        ::ink_storage::traits::SpreadLayout::clear_spread(__binding_0, __key_ptr);
                    }
                }
            }
        }
    };
    const _: () = {
        impl ::ink_storage::traits::StorageLayout for Mod2 {
            fn layout(
                __key_ptr: &mut ::ink_storage::traits::KeyPtr,
            ) -> ::ink_metadata::layout::Layout {
                ::ink_metadata::layout::Layout::Struct(::ink_metadata::layout::StructLayout::new(
                    <[_]>::into_vec(box [::ink_metadata::layout::FieldLayout::new(
                        Some("acct_id"),
                        <AccountId as ::ink_storage::traits::StorageLayout>::layout(__key_ptr),
                    )]),
                ))
            }
        }
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[allow(unused_imports)]
        use ink_lang::{Env as _, StaticEnv as _};
    };
    #[cfg(not(test))]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[cfg(not(test))]
        #[no_mangle]
        fn deploy() -> u32 {
            ::ink_lang::DispatchRetCode::from(
                <Mod2 as ::ink_lang::DispatchUsingMode>::dispatch_using_mode(
                    ::ink_lang::DispatchMode::Instantiate,
                ),
            )
            .to_u32()
        }
        #[cfg(not(test))]
        #[no_mangle]
        fn call() -> u32 {
            if true {
                ::ink_lang::deny_payment::<<Mod2 as ::ink_lang::ContractEnv>::Env>()
                    .expect("caller transferred value even though all ink! message deny payments")
            }
            ::ink_lang::DispatchRetCode::from(
                <Mod2 as ::ink_lang::DispatchUsingMode>::dispatch_using_mode(
                    ::ink_lang::DispatchMode::Call,
                ),
            )
            .to_u32()
        }
        impl ::ink_lang::DispatchUsingMode for Mod2 {
            #[allow(unused_parens)]
            fn dispatch_using_mode(
                mode: ::ink_lang::DispatchMode,
            ) -> core::result::Result<(), ::ink_lang::DispatchError> {
                match mode {                                                                                                                                                                 ::ink_lang::DispatchMode::Instantiate => {                                                                                                                                   <<Mod2 as ::ink_lang::ConstructorDispatcher>::Type                                                                                                                           as                                                                                                                                                                       ::ink_lang::Execute>::execute(::ink_env::decode_input::<<Mod2                                                                                                                                                                    as                                                                                                                                                                       ::ink_lang::ConstructorDispatcher>::Type>().map_err(|_|                                                                                                                                                                          ::ink_lang::DispatchError::CouldNotReadInput)?)                                                                                                                                                                           }                                                                                                                                                                        ::ink_lang::DispatchMode::Call => {                                                                                                                                          <<Mod2 as ::ink_lang::MessageDispatcher>::Type as                                                                                                                            ::ink_lang::Execute>::execute(::ink_env::decode_input::<<Mod2                                                                                                                                                                    as                                                                                                                                                                       ::ink_lang::MessageDispatcher>::Type>().map_err(|_|                                                                                                                                                                          ::ink_lang::DispatchError::CouldNotReadInput)?)                                                                                                                                                                               }                                                                                                                                                                    }
            }
        }
        #[doc(hidden)]
        pub struct __ink_Msg<S> {
            marker: core::marker::PhantomData<fn() -> S>,
        }
        #[doc(hidden)]
        pub struct __ink_Constr<S> {
            marker: core::marker::PhantomData<fn() -> S>,
        }
        impl ::ink_lang::FnInput for __ink_Msg<[(); 2297276310usize]> {
            type Input = ();
        }
        impl ::ink_lang::FnSelector for __ink_Msg<[(); 2297276310usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([150u8, 167u8, 237u8, 136u8]);
        }
        impl ::ink_lang::FnState for __ink_Msg<[(); 2297276310usize]> {
            type State = Mod2;
        }
        impl ::ink_lang::FnOutput for __ink_Msg<[(); 2297276310usize]> {
            #[allow(unused_parens)]
            type Output = u32;
        }
        impl ::ink_lang::MessageRef for __ink_Msg<[(); 2297276310usize]> {
            const CALLABLE: fn(
                &<Self as ::ink_lang::FnState>::State,
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnOutput>::Output = |state, _| <Mod2>::msg(state);
        }
        impl ::ink_lang::FnInput for __ink_Constr<[(); 1587392155usize]> {
            type Input = ();
        }
        impl ::ink_lang::FnSelector for __ink_Constr<[(); 1587392155usize]> {
            const SELECTOR: ::ink_env::call::Selector =
                ::ink_env::call::Selector::new([155u8, 174u8, 157u8, 94u8]);
        }
        impl ::ink_lang::FnState for __ink_Constr<[(); 1587392155usize]> {
            type State = Mod2;
        }
        impl ::ink_lang::Constructor for __ink_Constr<[(); 1587392155usize]> {
            const CALLABLE: fn(
                <Self as ::ink_lang::FnInput>::Input,
            ) -> <Self as ::ink_lang::FnState>::State = |_| <Mod2>::new();
        }
        const _: () = {
            #[doc(hidden)]  
            pub enum __ink_MessageDispatchEnum {
                __ink_Message_0x96a7ed88(),
            }
            impl ::ink_lang::MessageDispatcher for Mod2 {
                type Type = __ink_MessageDispatchEnum;
            }
            impl ::scale::Decode for __ink_MessageDispatchEnum {
                fn decode<I: ::scale::Input>(
                    input: &mut I,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    match <[u8; 4] as ::scale::Decode>::decode(input)? {
                        [150u8, 167u8, 237u8, 136u8] => Ok(Self::__ink_Message_0x96a7ed88()),
                        _invalid => Err(::scale::Error::from(
                            "encountered unknown ink! message selector",
                        )),
                    }
                }
            }
            impl ::ink_lang::Execute for __ink_MessageDispatchEnum {
                fn execute(self) -> ::core::result::Result<(), ::ink_lang::DispatchError> {
                    match self {
                        Self::__ink_Message_0x96a7ed88() => ::ink_lang::execute_message::<
                            <Mod2 as ::ink_lang::ContractEnv>::Env,
                            __ink_Msg<[(); 2297276310usize]>,
                            _,
                        >(
                            ::ink_lang::AcceptsPayments(true),
                            ::ink_lang::EnablesDynamicStorageAllocator(false),
                            move |state: &Mod2| {
                                <__ink_Msg<[(); 2297276310usize]>                                                                                                                                            as                                                                                                                                                                       ::ink_lang::MessageRef>::CALLABLE(state,                                                                                                                                                                   ())
                            },
                        ),
                    }
                }
            }
        };
        const _: () = {
            #[doc(hidden)]
            pub enum __ink_ConstructorDispatchEnum {
                __ink_Constructor_0x9bae9d5e(),
            }
            impl ::ink_lang::ConstructorDispatcher for Mod2 {
                type Type = __ink_ConstructorDispatchEnum;
            }
            impl ::scale::Decode for __ink_ConstructorDispatchEnum {
                fn decode<I: ::scale::Input>(
                    input: &mut I,
                ) -> ::core::result::Result<Self, ::scale::Error> {
                    match <[u8; 4] as ::scale::Decode>::decode(input)? {
                        [155u8, 174u8, 157u8, 94u8] => Ok(Self::__ink_Constructor_0x9bae9d5e()),
                        _invalid => Err(::scale::Error::from(
                            "encountered unknown ink! constructor selector",
                        )),
                    }
                }
            }
            impl ::ink_lang::Execute for __ink_ConstructorDispatchEnum {
                fn execute(self) -> ::core::result::Result<(), ::ink_lang::DispatchError> {
                    match self {
                        Self::__ink_Constructor_0x9bae9d5e() => {
                            ::ink_lang::execute_constructor::<__ink_Constr<[(); 1587392155usize]>, _>(
                                ::ink_lang::EnablesDynamicStorageAllocator(false),
                                move || {
                                    <__ink_Constr<[(); 1587392155usize]>                                                                                                                                         as                                                                                                                                                                       ::ink_lang::Constructor>::CALLABLE(())
                                },
                            )
                        }
                    }
                }
            }
        };
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        use ink_lang::{EmitEvent, Env, StaticEnv};
        const _: fn() = || {
            trait TypeEq {
                type This: ?Sized;
            }
            impl<T: ?Sized> TypeEq for T {
                type This = Self;
            }
            fn assert_type_eq_all<T, U>()
            where
                T: ?Sized + TypeEq<This = U>,
                U: ?Sized,
            {
            }
            assert_type_eq_all::<Mod2, Mod2>();
        };
        impl Mod2 {
            #[doc = " Instantiate a delegator with the given sub-contract codes."]
            pub fn new() -> Self {
                Self {
                    acct_id: AccountId::from([0x1; 32]),
                }
            }
            pub fn msg(&self) -> u32 {
                2
            }
        }
    };
    #[cfg(feature = "std")]
    #[cfg(not(feature = "ink-as-dependency"))]
    const _: () = {
        #[no_mangle]
        pub fn __ink_generate_metadata() -> ::ink_metadata::InkProject {
            let contract: ::ink_metadata::ContractSpec = {
                ::ink_metadata::ContractSpec::new()
                    .constructors(<[_]>::into_vec(box [
                        ::ink_metadata::ConstructorSpec::from_name("new")
                            .selector([155u8, 174u8, 157u8, 94u8])
                            .args(::alloc::vec::Vec::new())
                            .docs(<[_]>::into_vec(box [
                                " Instantiate a delegator with the given sub-contract codes.",
                            ]))
                            .done(),
                    ]))
                    .messages(<[_]>::into_vec(box [
                        ::ink_metadata::MessageSpec::from_name("msg")
                            .selector([150u8, 167u8, 237u8, 136u8])
                            .args(::alloc::vec::Vec::new())
                            .returns(::ink_metadata::ReturnTypeSpec::new(
                                ::ink_metadata::TypeSpec::with_name_segs::<u32, _>(
                                    <[_]>::into_vec(box ["u32"]).into_iter().map(AsRef::as_ref),
                                ),
                            ))
                            .mutates(false)
                            .payable(false)
                            .docs(::alloc::vec::Vec::new())
                            .done(),
                    ]))
                    .events(::alloc::vec::Vec::new())
                    .docs(::alloc::vec::Vec::new())
                    .done()
            };
            let layout: ::ink_metadata::layout::Layout = {
                <Mod2 as ::ink_storage::traits::StorageLayout>::layout(
                    &mut ::ink_primitives::KeyPtr::from(::ink_primitives::Key::from([0x00; 32])),
                )
            };
            ::ink_metadata::InkProject::new(layout, contract)
        }
    };
}
