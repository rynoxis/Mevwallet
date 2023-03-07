pub use mwb::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod mwb {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"at\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"etch\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static MWB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        128,
        96,
        64,
        82,
        52,
        97,
        0,
        22,
        87,
        97,
        3,
        124,
        144,
        129,
        97,
        0,
        28,
        130,
        57,
        243,
        91,
        96,
        0,
        128,
        253,
        254,
        96,
        128,
        96,
        64,
        144,
        128,
        130,
        82,
        96,
        4,
        145,
        130,
        54,
        16,
        21,
        97,
        0,
        23,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        146,
        131,
        53,
        96,
        224,
        28,
        99,
        119,
        194,
        67,
        235,
        20,
        97,
        0,
        46,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        97,
        1,
        227,
        87,
        131,
        96,
        32,
        128,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        2,
        216,
        87,
        130,
        53,
        144,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        22,
        128,
        146,
        3,
        97,
        2,
        212,
        87,
        127,
        77,
        123,
        175,
        6,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        134,
        82,
        128,
        132,
        135,
        1,
        82,
        96,
        3,
        96,
        36,
        135,
        1,
        82,
        127,
        77,
        87,
        66,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        68,
        135,
        1,
        82,
        115,
        113,
        9,
        112,
        158,
        207,
        169,
        26,
        128,
        98,
        111,
        243,
        152,
        157,
        104,
        246,
        127,
        91,
        29,
        209,
        45,
        144,
        131,
        135,
        96,
        100,
        129,
        133,
        90,
        250,
        150,
        135,
        21,
        97,
        2,
        202,
        87,
        132,
        151,
        97,
        1,
        231,
        87,
        91,
        80,
        129,
        59,
        21,
        97,
        1,
        227,
        87,
        131,
        96,
        100,
        97,
        1,
        126,
        148,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        96,
        31,
        138,
        81,
        155,
        140,
        152,
        137,
        151,
        136,
        149,
        127,
        180,
        214,
        199,
        130,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        135,
        82,
        141,
        135,
        1,
        82,
        141,
        96,
        36,
        135,
        1,
        82,
        130,
        81,
        146,
        131,
        145,
        130,
        96,
        68,
        137,
        1,
        82,
        136,
        136,
        1,
        145,
        1,
        97,
        3,
        76,
        86,
        91,
        1,
        22,
        129,
        1,
        3,
        1,
        146,
        90,
        241,
        128,
        21,
        97,
        1,
        217,
        87,
        97,
        1,
        149,
        87,
        131,
        128,
        243,
        91,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        131,
        17,
        97,
        1,
        173,
        87,
        80,
        82,
        56,
        128,
        128,
        131,
        128,
        243,
        91,
        131,
        96,
        65,
        96,
        36,
        146,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        82,
        253,
        91,
        130,
        81,
        61,
        134,
        130,
        62,
        61,
        144,
        253,
        91,
        131,
        128,
        253,
        91,
        145,
        146,
        147,
        80,
        149,
        80,
        61,
        128,
        136,
        131,
        62,
        97,
        1,
        252,
        129,
        131,
        97,
        2,
        220,
        86,
        91,
        129,
        1,
        144,
        134,
        129,
        131,
        3,
        18,
        97,
        2,
        198,
        87,
        128,
        81,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        145,
        130,
        130,
        17,
        97,
        2,
        194,
        87,
        1,
        144,
        130,
        96,
        31,
        131,
        1,
        18,
        21,
        97,
        2,
        190,
        87,
        129,
        81,
        144,
        129,
        17,
        97,
        2,
        146,
        87,
        144,
        136,
        148,
        147,
        146,
        145,
        135,
        81,
        146,
        97,
        2,
        107,
        138,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        96,
        31,
        133,
        1,
        22,
        1,
        133,
        97,
        2,
        220,
        86,
        91,
        129,
        132,
        82,
        137,
        130,
        132,
        1,
        1,
        17,
        97,
        2,
        142,
        87,
        97,
        2,
        135,
        145,
        137,
        128,
        133,
        1,
        145,
        1,
        97,
        3,
        76,
        86,
        91,
        149,
        56,
        97,
        1,
        1,
        86,
        91,
        133,
        128,
        253,
        91,
        96,
        36,
        137,
        96,
        65,
        136,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        82,
        253,
        91,
        136,
        128,
        253,
        91,
        137,
        128,
        253,
        91,
        135,
        128,
        253,
        91,
        134,
        81,
        61,
        134,
        130,
        62,
        61,
        144,
        253,
        91,
        130,
        128,
        253,
        91,
        80,
        128,
        253,
        91,
        144,
        96,
        31,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        145,
        1,
        22,
        129,
        1,
        144,
        129,
        16,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        17,
        23,
        97,
        3,
        29,
        87,
        96,
        64,
        82,
        86,
        91,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        0,
        82,
        96,
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        0,
        91,
        131,
        129,
        16,
        97,
        3,
        95,
        87,
        80,
        80,
        96,
        0,
        145,
        1,
        82,
        86,
        91,
        129,
        129,
        1,
        81,
        131,
        130,
        1,
        82,
        96,
        32,
        1,
        97,
        3,
        79,
        86,
        254,
        161,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        10,
    ];
    ///The bytecode of the contract.
    pub static MWB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        144,
        128,
        130,
        82,
        96,
        4,
        145,
        130,
        54,
        16,
        21,
        97,
        0,
        23,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        146,
        131,
        53,
        96,
        224,
        28,
        99,
        119,
        194,
        67,
        235,
        20,
        97,
        0,
        46,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        97,
        1,
        227,
        87,
        131,
        96,
        32,
        128,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        2,
        216,
        87,
        130,
        53,
        144,
        115,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        22,
        128,
        146,
        3,
        97,
        2,
        212,
        87,
        127,
        77,
        123,
        175,
        6,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        134,
        82,
        128,
        132,
        135,
        1,
        82,
        96,
        3,
        96,
        36,
        135,
        1,
        82,
        127,
        77,
        87,
        66,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        68,
        135,
        1,
        82,
        115,
        113,
        9,
        112,
        158,
        207,
        169,
        26,
        128,
        98,
        111,
        243,
        152,
        157,
        104,
        246,
        127,
        91,
        29,
        209,
        45,
        144,
        131,
        135,
        96,
        100,
        129,
        133,
        90,
        250,
        150,
        135,
        21,
        97,
        2,
        202,
        87,
        132,
        151,
        97,
        1,
        231,
        87,
        91,
        80,
        129,
        59,
        21,
        97,
        1,
        227,
        87,
        131,
        96,
        100,
        97,
        1,
        126,
        148,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        96,
        31,
        138,
        81,
        155,
        140,
        152,
        137,
        151,
        136,
        149,
        127,
        180,
        214,
        199,
        130,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        135,
        82,
        141,
        135,
        1,
        82,
        141,
        96,
        36,
        135,
        1,
        82,
        130,
        81,
        146,
        131,
        145,
        130,
        96,
        68,
        137,
        1,
        82,
        136,
        136,
        1,
        145,
        1,
        97,
        3,
        76,
        86,
        91,
        1,
        22,
        129,
        1,
        3,
        1,
        146,
        90,
        241,
        128,
        21,
        97,
        1,
        217,
        87,
        97,
        1,
        149,
        87,
        131,
        128,
        243,
        91,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        131,
        17,
        97,
        1,
        173,
        87,
        80,
        82,
        56,
        128,
        128,
        131,
        128,
        243,
        91,
        131,
        96,
        65,
        96,
        36,
        146,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        82,
        253,
        91,
        130,
        81,
        61,
        134,
        130,
        62,
        61,
        144,
        253,
        91,
        131,
        128,
        253,
        91,
        145,
        146,
        147,
        80,
        149,
        80,
        61,
        128,
        136,
        131,
        62,
        97,
        1,
        252,
        129,
        131,
        97,
        2,
        220,
        86,
        91,
        129,
        1,
        144,
        134,
        129,
        131,
        3,
        18,
        97,
        2,
        198,
        87,
        128,
        81,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        145,
        130,
        130,
        17,
        97,
        2,
        194,
        87,
        1,
        144,
        130,
        96,
        31,
        131,
        1,
        18,
        21,
        97,
        2,
        190,
        87,
        129,
        81,
        144,
        129,
        17,
        97,
        2,
        146,
        87,
        144,
        136,
        148,
        147,
        146,
        145,
        135,
        81,
        146,
        97,
        2,
        107,
        138,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        96,
        31,
        133,
        1,
        22,
        1,
        133,
        97,
        2,
        220,
        86,
        91,
        129,
        132,
        82,
        137,
        130,
        132,
        1,
        1,
        17,
        97,
        2,
        142,
        87,
        97,
        2,
        135,
        145,
        137,
        128,
        133,
        1,
        145,
        1,
        97,
        3,
        76,
        86,
        91,
        149,
        56,
        97,
        1,
        1,
        86,
        91,
        133,
        128,
        253,
        91,
        96,
        36,
        137,
        96,
        65,
        136,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        131,
        82,
        82,
        253,
        91,
        136,
        128,
        253,
        91,
        137,
        128,
        253,
        91,
        135,
        128,
        253,
        91,
        134,
        81,
        61,
        134,
        130,
        62,
        61,
        144,
        253,
        91,
        130,
        128,
        253,
        91,
        80,
        128,
        253,
        91,
        144,
        96,
        31,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        224,
        145,
        1,
        22,
        129,
        1,
        144,
        129,
        16,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        130,
        17,
        23,
        97,
        3,
        29,
        87,
        96,
        64,
        82,
        86,
        91,
        127,
        78,
        72,
        123,
        113,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        0,
        82,
        96,
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        0,
        91,
        131,
        129,
        16,
        97,
        3,
        95,
        87,
        80,
        80,
        96,
        0,
        145,
        1,
        82,
        86,
        91,
        129,
        129,
        1,
        81,
        131,
        130,
        1,
        82,
        96,
        32,
        1,
        97,
        3,
        79,
        86,
        254,
        161,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        10,
    ];
    ///The deployed bytecode of the contract.
    pub static MWB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MWB<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MWB<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MWB<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MWB<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MWB<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(MWB)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MWB<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MWB_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MWB_ABI.clone(),
                MWB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `etch` (0x77c243eb) function
        pub fn etch(
            &self,
            at: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 194, 67, 235], at)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MWB<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `etch` function with signature `etch(address)` and selector `0x77c243eb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "etch", abi = "etch(address)")]
    pub struct EtchCall {
        pub at: ::ethers::core::types::Address,
    }
}