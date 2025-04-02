///Module containing a contract's types and functions.
/**

```solidity
library EntropyStructs {
    struct ProviderInfo { uint128 feeInWei; uint128 accruedFeesInWei; bytes32 originalCommitment; uint64 originalCommitmentSequenceNumber; bytes commitmentMetadata; bytes uri; uint64 endSequenceNumber; uint64 sequenceNumber; bytes32 currentCommitment; uint64 currentCommitmentSequenceNumber; address feeManager; }
    struct Request { address provider; uint64 sequenceNumber; uint32 numHashes; bytes32 commitment; uint64 blockNumber; address requester; bool useBlockhash; bool isRequestWithCallback; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod EntropyStructs {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct ProviderInfo { uint128 feeInWei; uint128 accruedFeesInWei; bytes32 originalCommitment; uint64 originalCommitmentSequenceNumber; bytes commitmentMetadata; bytes uri; uint64 endSequenceNumber; uint64 sequenceNumber; bytes32 currentCommitment; uint64 currentCommitmentSequenceNumber; address feeManager; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProviderInfo {
        #[allow(missing_docs)]
        pub feeInWei: u128,
        #[allow(missing_docs)]
        pub accruedFeesInWei: u128,
        #[allow(missing_docs)]
        pub originalCommitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub originalCommitmentSequenceNumber: u64,
        #[allow(missing_docs)]
        pub commitmentMetadata: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub uri: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub endSequenceNumber: u64,
        #[allow(missing_docs)]
        pub sequenceNumber: u64,
        #[allow(missing_docs)]
        pub currentCommitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub currentCommitmentSequenceNumber: u64,
        #[allow(missing_docs)]
        pub feeManager: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u128,
            u128,
            alloy::sol_types::private::FixedBytes<32>,
            u64,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            u64,
            u64,
            alloy::sol_types::private::FixedBytes<32>,
            u64,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProviderInfo> for UnderlyingRustTuple<'_> {
            fn from(value: ProviderInfo) -> Self {
                (
                    value.feeInWei,
                    value.accruedFeesInWei,
                    value.originalCommitment,
                    value.originalCommitmentSequenceNumber,
                    value.commitmentMetadata,
                    value.uri,
                    value.endSequenceNumber,
                    value.sequenceNumber,
                    value.currentCommitment,
                    value.currentCommitmentSequenceNumber,
                    value.feeManager,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ProviderInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    feeInWei: tuple.0,
                    accruedFeesInWei: tuple.1,
                    originalCommitment: tuple.2,
                    originalCommitmentSequenceNumber: tuple.3,
                    commitmentMetadata: tuple.4,
                    uri: tuple.5,
                    endSequenceNumber: tuple.6,
                    sequenceNumber: tuple.7,
                    currentCommitment: tuple.8,
                    currentCommitmentSequenceNumber: tuple.9,
                    feeManager: tuple.10,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ProviderInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ProviderInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.feeInWei),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.accruedFeesInWei),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.originalCommitment),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.originalCommitmentSequenceNumber,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.commitmentMetadata,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.uri,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.endSequenceNumber),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentCommitment),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.currentCommitmentSequenceNumber,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.feeManager,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ProviderInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ProviderInfo {
            const NAME: &'static str = "ProviderInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ProviderInfo(uint128 feeInWei,uint128 accruedFeesInWei,bytes32 originalCommitment,uint64 originalCommitmentSequenceNumber,bytes commitmentMetadata,bytes uri,uint64 endSequenceNumber,uint64 sequenceNumber,bytes32 currentCommitment,uint64 currentCommitmentSequenceNumber,address feeManager)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.feeInWei)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.accruedFeesInWei,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.originalCommitment,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.originalCommitmentSequenceNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.commitmentMetadata,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.uri,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.endSequenceNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sequenceNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currentCommitment,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currentCommitmentSequenceNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.feeManager,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ProviderInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feeInWei,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.accruedFeesInWei,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.originalCommitment,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.originalCommitmentSequenceNumber,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.commitmentMetadata,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.uri,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.endSequenceNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sequenceNumber,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currentCommitment,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currentCommitmentSequenceNumber,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.feeManager,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feeInWei,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.accruedFeesInWei,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.originalCommitment,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.originalCommitmentSequenceNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.commitmentMetadata,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.uri,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.endSequenceNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sequenceNumber,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currentCommitment,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currentCommitmentSequenceNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.feeManager,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct Request { address provider; uint64 sequenceNumber; uint32 numHashes; bytes32 commitment; uint64 blockNumber; address requester; bool useBlockhash; bool isRequestWithCallback; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Request {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequenceNumber: u64,
        #[allow(missing_docs)]
        pub numHashes: u32,
        #[allow(missing_docs)]
        pub commitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockNumber: u64,
        #[allow(missing_docs)]
        pub requester: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub useBlockhash: bool,
        #[allow(missing_docs)]
        pub isRequestWithCallback: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u64,
            u32,
            alloy::sol_types::private::FixedBytes<32>,
            u64,
            alloy::sol_types::private::Address,
            bool,
            bool,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Request> for UnderlyingRustTuple<'_> {
            fn from(value: Request) -> Self {
                (
                    value.provider,
                    value.sequenceNumber,
                    value.numHashes,
                    value.commitment,
                    value.blockNumber,
                    value.requester,
                    value.useBlockhash,
                    value.isRequestWithCallback,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Request {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    provider: tuple.0,
                    sequenceNumber: tuple.1,
                    numHashes: tuple.2,
                    commitment: tuple.3,
                    blockNumber: tuple.4,
                    requester: tuple.5,
                    useBlockhash: tuple.6,
                    isRequestWithCallback: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Request {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Request {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.numHashes),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitment),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.requester,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.useBlockhash,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.isRequestWithCallback,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Request {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Request {
            const NAME: &'static str = "Request";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Request(address provider,uint64 sequenceNumber,uint32 numHashes,bytes32 commitment,uint64 blockNumber,address requester,bool useBlockhash,bool isRequestWithCallback)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.provider,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sequenceNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.numHashes)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.commitment)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.blockNumber)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requester,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.useBlockhash,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.isRequestWithCallback,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Request {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.provider,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sequenceNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numHashes,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.commitment,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blockNumber,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requester,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.useBlockhash,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.isRequestWithCallback,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.provider,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sequenceNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numHashes,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.commitment,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requester,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.useBlockhash,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.isRequestWithCallback,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EntropyStructs`](self) contract instance.

See the [wrapper's documentation](`EntropyStructsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EntropyStructsInstance<T, P, N> {
        EntropyStructsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`EntropyStructs`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EntropyStructs`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EntropyStructsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EntropyStructsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EntropyStructsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EntropyStructsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EntropyStructs`](self) contract instance.

See the [wrapper's documentation](`EntropyStructsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> EntropyStructsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EntropyStructsInstance<T, P, N> {
            EntropyStructsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EntropyStructsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EntropyStructsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library EntropyStructs {
    struct ProviderInfo {
        uint128 feeInWei;
        uint128 accruedFeesInWei;
        bytes32 originalCommitment;
        uint64 originalCommitmentSequenceNumber;
        bytes commitmentMetadata;
        bytes uri;
        uint64 endSequenceNumber;
        uint64 sequenceNumber;
        bytes32 currentCommitment;
        uint64 currentCommitmentSequenceNumber;
        address feeManager;
    }
    struct Request {
        address provider;
        uint64 sequenceNumber;
        uint32 numHashes;
        bytes32 commitment;
        uint64 blockNumber;
        address requester;
        bool useBlockhash;
        bool isRequestWithCallback;
    }
}

interface EntropyEvents {
    event ProviderFeeManagerUpdated(address provider, address oldFeeManager, address newFeeManager);
    event ProviderFeeUpdated(address provider, uint128 oldFee, uint128 newFee);
    event ProviderUriUpdated(address provider, bytes oldUri, bytes newUri);
    event Registered(EntropyStructs.ProviderInfo provider);
    event Requested(EntropyStructs.Request request);
    event RequestedWithCallback(address indexed provider, address indexed requestor, uint64 indexed sequenceNumber, bytes32 userRandomNumber, EntropyStructs.Request request);
    event Revealed(EntropyStructs.Request request, bytes32 userRevelation, bytes32 providerRevelation, bytes32 blockHash, bytes32 randomNumber);
    event RevealedWithCallback(EntropyStructs.Request request, bytes32 userRandomNumber, bytes32 providerRevelation, bytes32 randomNumber);
    event Withdrawal(address provider, address recipient, uint128 withdrawnAmount);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "event",
    "name": "ProviderFeeManagerUpdated",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "oldFeeManager",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newFeeManager",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProviderFeeUpdated",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "oldFee",
        "type": "uint128",
        "indexed": false,
        "internalType": "uint128"
      },
      {
        "name": "newFee",
        "type": "uint128",
        "indexed": false,
        "internalType": "uint128"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProviderUriUpdated",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "oldUri",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "newUri",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Registered",
    "inputs": [
      {
        "name": "provider",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct EntropyStructs.ProviderInfo",
        "components": [
          {
            "name": "feeInWei",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "accruedFeesInWei",
            "type": "uint128",
            "internalType": "uint128"
          },
          {
            "name": "originalCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "originalCommitmentSequenceNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "commitmentMetadata",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "uri",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "endSequenceNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "sequenceNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "currentCommitment",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "currentCommitmentSequenceNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "feeManager",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Requested",
    "inputs": [
      {
        "name": "request",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct EntropyStructs.Request",
        "components": [
          {
            "name": "provider",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "sequenceNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "numHashes",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "commitment",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "blockNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "useBlockhash",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "isRequestWithCallback",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RequestedWithCallback",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "requestor",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sequenceNumber",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "userRandomNumber",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "request",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct EntropyStructs.Request",
        "components": [
          {
            "name": "provider",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "sequenceNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "numHashes",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "commitment",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "blockNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "useBlockhash",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "isRequestWithCallback",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Revealed",
    "inputs": [
      {
        "name": "request",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct EntropyStructs.Request",
        "components": [
          {
            "name": "provider",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "sequenceNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "numHashes",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "commitment",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "blockNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "useBlockhash",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "isRequestWithCallback",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      },
      {
        "name": "userRevelation",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "providerRevelation",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "blockHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "randomNumber",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RevealedWithCallback",
    "inputs": [
      {
        "name": "request",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct EntropyStructs.Request",
        "components": [
          {
            "name": "provider",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "sequenceNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "numHashes",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "commitment",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "blockNumber",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "requester",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "useBlockhash",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "isRequestWithCallback",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      },
      {
        "name": "userRandomNumber",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "providerRevelation",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "randomNumber",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Withdrawal",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "recipient",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "withdrawnAmount",
        "type": "uint128",
        "indexed": false,
        "internalType": "uint128"
      }
    ],
    "anonymous": false
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod EntropyEvents {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /**Event with signature `ProviderFeeManagerUpdated(address,address,address)` and selector `0x2c0fa560a1e6d11854f3f965d262e756c1b6d23d2bfe8f0e54b7807dd79b946b`.
```solidity
event ProviderFeeManagerUpdated(address provider, address oldFeeManager, address newFeeManager);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProviderFeeManagerUpdated {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldFeeManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newFeeManager: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProviderFeeManagerUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProviderFeeManagerUpdated(address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8,
                15u8,
                165u8,
                96u8,
                161u8,
                230u8,
                209u8,
                24u8,
                84u8,
                243u8,
                249u8,
                101u8,
                210u8,
                98u8,
                231u8,
                86u8,
                193u8,
                182u8,
                210u8,
                61u8,
                43u8,
                254u8,
                143u8,
                14u8,
                84u8,
                183u8,
                128u8,
                125u8,
                215u8,
                155u8,
                148u8,
                107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    provider: data.0,
                    oldFeeManager: data.1,
                    newFeeManager: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.oldFeeManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newFeeManager,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProviderFeeManagerUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProviderFeeManagerUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ProviderFeeManagerUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProviderFeeUpdated(address,uint128,uint128)` and selector `0x40873158a9e1446599b5dee14bfd652e53a6f48605dab5aaac3b8a12a56c7fce`.
```solidity
event ProviderFeeUpdated(address provider, uint128 oldFee, uint128 newFee);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProviderFeeUpdated {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldFee: u128,
        #[allow(missing_docs)]
        pub newFee: u128,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProviderFeeUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Uint<128>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProviderFeeUpdated(address,uint128,uint128)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                135u8,
                49u8,
                88u8,
                169u8,
                225u8,
                68u8,
                101u8,
                153u8,
                181u8,
                222u8,
                225u8,
                75u8,
                253u8,
                101u8,
                46u8,
                83u8,
                166u8,
                244u8,
                134u8,
                5u8,
                218u8,
                181u8,
                170u8,
                172u8,
                59u8,
                138u8,
                18u8,
                165u8,
                108u8,
                127u8,
                206u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    provider: data.0,
                    oldFee: data.1,
                    newFee: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldFee),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.newFee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProviderFeeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProviderFeeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProviderFeeUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProviderUriUpdated(address,bytes,bytes)` and selector `0x1efad1d69168ff2e29c45661eed77d2de2b8c95f412cd22a65b15a38e24f7088`.
```solidity
event ProviderUriUpdated(address provider, bytes oldUri, bytes newUri);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProviderUriUpdated {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldUri: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub newUri: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProviderUriUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProviderUriUpdated(address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8,
                250u8,
                209u8,
                214u8,
                145u8,
                104u8,
                255u8,
                46u8,
                41u8,
                196u8,
                86u8,
                97u8,
                238u8,
                215u8,
                125u8,
                45u8,
                226u8,
                184u8,
                201u8,
                95u8,
                65u8,
                44u8,
                210u8,
                42u8,
                101u8,
                177u8,
                90u8,
                56u8,
                226u8,
                79u8,
                112u8,
                136u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    provider: data.0,
                    oldUri: data.1,
                    newUri: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.oldUri,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.newUri,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProviderUriUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProviderUriUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProviderUriUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Registered((uint128,uint128,bytes32,uint64,bytes,bytes,uint64,uint64,bytes32,uint64,address))` and selector `0x3ab0d7a1916fbcf1e3ec532e6c9b3a1dbcb827a3038d7cffa10eadc39fb62685`.
```solidity
event Registered(EntropyStructs.ProviderInfo provider);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Registered {
        #[allow(missing_docs)]
        pub provider: <EntropyStructs::ProviderInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Registered {
            type DataTuple<'a> = (EntropyStructs::ProviderInfo,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Registered((uint128,uint128,bytes32,uint64,bytes,bytes,uint64,uint64,bytes32,uint64,address))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                58u8,
                176u8,
                215u8,
                161u8,
                145u8,
                111u8,
                188u8,
                241u8,
                227u8,
                236u8,
                83u8,
                46u8,
                108u8,
                155u8,
                58u8,
                29u8,
                188u8,
                184u8,
                39u8,
                163u8,
                3u8,
                141u8,
                124u8,
                255u8,
                161u8,
                14u8,
                173u8,
                195u8,
                159u8,
                182u8,
                38u8,
                133u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { provider: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <EntropyStructs::ProviderInfo as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Registered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Registered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Registered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Requested((address,uint64,uint32,bytes32,uint64,address,bool,bool))` and selector `0x20e2c2fc72b2cb9fbae9d7d8fd4bdf5bdcc4579043e1e9854e2baf045b6a31d3`.
```solidity
event Requested(EntropyStructs.Request request);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Requested {
        #[allow(missing_docs)]
        pub request: <EntropyStructs::Request as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Requested {
            type DataTuple<'a> = (EntropyStructs::Request,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Requested((address,uint64,uint32,bytes32,uint64,address,bool,bool))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                32u8,
                226u8,
                194u8,
                252u8,
                114u8,
                178u8,
                203u8,
                159u8,
                186u8,
                233u8,
                215u8,
                216u8,
                253u8,
                75u8,
                223u8,
                91u8,
                220u8,
                196u8,
                87u8,
                144u8,
                67u8,
                225u8,
                233u8,
                133u8,
                78u8,
                43u8,
                175u8,
                4u8,
                91u8,
                106u8,
                49u8,
                211u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { request: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <EntropyStructs::Request as alloy_sol_types::SolType>::tokenize(
                        &self.request,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Requested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Requested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Requested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RequestedWithCallback(address,address,uint64,bytes32,(address,uint64,uint32,bytes32,uint64,address,bool,bool))` and selector `0xa4c85ab66677ced5caabbbba151714887944b9e0fee05f320e42a1b13a01fbc6`.
```solidity
event RequestedWithCallback(address indexed provider, address indexed requestor, uint64 indexed sequenceNumber, bytes32 userRandomNumber, EntropyStructs.Request request);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RequestedWithCallback {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub requestor: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequenceNumber: u64,
        #[allow(missing_docs)]
        pub userRandomNumber: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub request: <EntropyStructs::Request as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RequestedWithCallback {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                EntropyStructs::Request,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "RequestedWithCallback(address,address,uint64,bytes32,(address,uint64,uint32,bytes32,uint64,address,bool,bool))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8,
                200u8,
                90u8,
                182u8,
                102u8,
                119u8,
                206u8,
                213u8,
                202u8,
                171u8,
                187u8,
                186u8,
                21u8,
                23u8,
                20u8,
                136u8,
                121u8,
                68u8,
                185u8,
                224u8,
                254u8,
                224u8,
                95u8,
                50u8,
                14u8,
                66u8,
                161u8,
                177u8,
                58u8,
                1u8,
                251u8,
                198u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    provider: topics.1,
                    requestor: topics.2,
                    sequenceNumber: topics.3,
                    userRandomNumber: data.0,
                    request: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userRandomNumber),
                    <EntropyStructs::Request as alloy_sol_types::SolType>::tokenize(
                        &self.request,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.provider.clone(),
                    self.requestor.clone(),
                    self.sequenceNumber.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.provider,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.requestor,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.sequenceNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RequestedWithCallback {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RequestedWithCallback> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RequestedWithCallback) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Revealed((address,uint64,uint32,bytes32,uint64,address,bool,bool),bytes32,bytes32,bytes32,bytes32)` and selector `0x39c729f66b0c8aa543d92bc83fb7e0914c9701326b96365b593f28ba706976e4`.
```solidity
event Revealed(EntropyStructs.Request request, bytes32 userRevelation, bytes32 providerRevelation, bytes32 blockHash, bytes32 randomNumber);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Revealed {
        #[allow(missing_docs)]
        pub request: <EntropyStructs::Request as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub userRevelation: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub providerRevelation: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub randomNumber: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Revealed {
            type DataTuple<'a> = (
                EntropyStructs::Request,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Revealed((address,uint64,uint32,bytes32,uint64,address,bool,bool),bytes32,bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                57u8,
                199u8,
                41u8,
                246u8,
                107u8,
                12u8,
                138u8,
                165u8,
                67u8,
                217u8,
                43u8,
                200u8,
                63u8,
                183u8,
                224u8,
                145u8,
                76u8,
                151u8,
                1u8,
                50u8,
                107u8,
                150u8,
                54u8,
                91u8,
                89u8,
                63u8,
                40u8,
                186u8,
                112u8,
                105u8,
                118u8,
                228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    request: data.0,
                    userRevelation: data.1,
                    providerRevelation: data.2,
                    blockHash: data.3,
                    randomNumber: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <EntropyStructs::Request as alloy_sol_types::SolType>::tokenize(
                        &self.request,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userRevelation),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.providerRevelation),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockHash),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.randomNumber),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Revealed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Revealed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Revealed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RevealedWithCallback((address,uint64,uint32,bytes32,uint64,address,bool,bool),bytes32,bytes32,bytes32)` and selector `0x40be225f151772416d8785647e5641a0b53507623d0ee3fb88802b7d6bdbf728`.
```solidity
event RevealedWithCallback(EntropyStructs.Request request, bytes32 userRandomNumber, bytes32 providerRevelation, bytes32 randomNumber);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RevealedWithCallback {
        #[allow(missing_docs)]
        pub request: <EntropyStructs::Request as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub userRandomNumber: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub providerRevelation: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub randomNumber: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RevealedWithCallback {
            type DataTuple<'a> = (
                EntropyStructs::Request,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RevealedWithCallback((address,uint64,uint32,bytes32,uint64,address,bool,bool),bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                190u8,
                34u8,
                95u8,
                21u8,
                23u8,
                114u8,
                65u8,
                109u8,
                135u8,
                133u8,
                100u8,
                126u8,
                86u8,
                65u8,
                160u8,
                181u8,
                53u8,
                7u8,
                98u8,
                61u8,
                14u8,
                227u8,
                251u8,
                136u8,
                128u8,
                43u8,
                125u8,
                107u8,
                219u8,
                247u8,
                40u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    request: data.0,
                    userRandomNumber: data.1,
                    providerRevelation: data.2,
                    randomNumber: data.3,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <EntropyStructs::Request as alloy_sol_types::SolType>::tokenize(
                        &self.request,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userRandomNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.providerRevelation),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.randomNumber),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RevealedWithCallback {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RevealedWithCallback> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RevealedWithCallback) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Withdrawal(address,address,uint128)` and selector `0x02128911bc7070fd6c100b116c2dd9a3bb6bf132d5259a65ca8d0c86ccd78f49`.
```solidity
event Withdrawal(address provider, address recipient, uint128 withdrawnAmount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Withdrawal {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawnAmount: u128,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Withdrawal {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Withdrawal(address,address,uint128)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                2u8,
                18u8,
                137u8,
                17u8,
                188u8,
                112u8,
                112u8,
                253u8,
                108u8,
                16u8,
                11u8,
                17u8,
                108u8,
                45u8,
                217u8,
                163u8,
                187u8,
                107u8,
                241u8,
                50u8,
                213u8,
                37u8,
                154u8,
                101u8,
                202u8,
                141u8,
                12u8,
                134u8,
                204u8,
                215u8,
                143u8,
                73u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    provider: data.0,
                    recipient: data.1,
                    withdrawnAmount: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawnAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Withdrawal {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Withdrawal> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Withdrawal) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    ///Container for all the [`EntropyEvents`](self) events.
    pub enum EntropyEventsEvents {
        #[allow(missing_docs)]
        ProviderFeeManagerUpdated(ProviderFeeManagerUpdated),
        #[allow(missing_docs)]
        ProviderFeeUpdated(ProviderFeeUpdated),
        #[allow(missing_docs)]
        ProviderUriUpdated(ProviderUriUpdated),
        #[allow(missing_docs)]
        Registered(Registered),
        #[allow(missing_docs)]
        Requested(Requested),
        #[allow(missing_docs)]
        RequestedWithCallback(RequestedWithCallback),
        #[allow(missing_docs)]
        Revealed(Revealed),
        #[allow(missing_docs)]
        RevealedWithCallback(RevealedWithCallback),
        #[allow(missing_docs)]
        Withdrawal(Withdrawal),
    }
    #[automatically_derived]
    impl EntropyEventsEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                2u8,
                18u8,
                137u8,
                17u8,
                188u8,
                112u8,
                112u8,
                253u8,
                108u8,
                16u8,
                11u8,
                17u8,
                108u8,
                45u8,
                217u8,
                163u8,
                187u8,
                107u8,
                241u8,
                50u8,
                213u8,
                37u8,
                154u8,
                101u8,
                202u8,
                141u8,
                12u8,
                134u8,
                204u8,
                215u8,
                143u8,
                73u8,
            ],
            [
                30u8,
                250u8,
                209u8,
                214u8,
                145u8,
                104u8,
                255u8,
                46u8,
                41u8,
                196u8,
                86u8,
                97u8,
                238u8,
                215u8,
                125u8,
                45u8,
                226u8,
                184u8,
                201u8,
                95u8,
                65u8,
                44u8,
                210u8,
                42u8,
                101u8,
                177u8,
                90u8,
                56u8,
                226u8,
                79u8,
                112u8,
                136u8,
            ],
            [
                32u8,
                226u8,
                194u8,
                252u8,
                114u8,
                178u8,
                203u8,
                159u8,
                186u8,
                233u8,
                215u8,
                216u8,
                253u8,
                75u8,
                223u8,
                91u8,
                220u8,
                196u8,
                87u8,
                144u8,
                67u8,
                225u8,
                233u8,
                133u8,
                78u8,
                43u8,
                175u8,
                4u8,
                91u8,
                106u8,
                49u8,
                211u8,
            ],
            [
                44u8,
                15u8,
                165u8,
                96u8,
                161u8,
                230u8,
                209u8,
                24u8,
                84u8,
                243u8,
                249u8,
                101u8,
                210u8,
                98u8,
                231u8,
                86u8,
                193u8,
                182u8,
                210u8,
                61u8,
                43u8,
                254u8,
                143u8,
                14u8,
                84u8,
                183u8,
                128u8,
                125u8,
                215u8,
                155u8,
                148u8,
                107u8,
            ],
            [
                57u8,
                199u8,
                41u8,
                246u8,
                107u8,
                12u8,
                138u8,
                165u8,
                67u8,
                217u8,
                43u8,
                200u8,
                63u8,
                183u8,
                224u8,
                145u8,
                76u8,
                151u8,
                1u8,
                50u8,
                107u8,
                150u8,
                54u8,
                91u8,
                89u8,
                63u8,
                40u8,
                186u8,
                112u8,
                105u8,
                118u8,
                228u8,
            ],
            [
                58u8,
                176u8,
                215u8,
                161u8,
                145u8,
                111u8,
                188u8,
                241u8,
                227u8,
                236u8,
                83u8,
                46u8,
                108u8,
                155u8,
                58u8,
                29u8,
                188u8,
                184u8,
                39u8,
                163u8,
                3u8,
                141u8,
                124u8,
                255u8,
                161u8,
                14u8,
                173u8,
                195u8,
                159u8,
                182u8,
                38u8,
                133u8,
            ],
            [
                64u8,
                135u8,
                49u8,
                88u8,
                169u8,
                225u8,
                68u8,
                101u8,
                153u8,
                181u8,
                222u8,
                225u8,
                75u8,
                253u8,
                101u8,
                46u8,
                83u8,
                166u8,
                244u8,
                134u8,
                5u8,
                218u8,
                181u8,
                170u8,
                172u8,
                59u8,
                138u8,
                18u8,
                165u8,
                108u8,
                127u8,
                206u8,
            ],
            [
                64u8,
                190u8,
                34u8,
                95u8,
                21u8,
                23u8,
                114u8,
                65u8,
                109u8,
                135u8,
                133u8,
                100u8,
                126u8,
                86u8,
                65u8,
                160u8,
                181u8,
                53u8,
                7u8,
                98u8,
                61u8,
                14u8,
                227u8,
                251u8,
                136u8,
                128u8,
                43u8,
                125u8,
                107u8,
                219u8,
                247u8,
                40u8,
            ],
            [
                164u8,
                200u8,
                90u8,
                182u8,
                102u8,
                119u8,
                206u8,
                213u8,
                202u8,
                171u8,
                187u8,
                186u8,
                21u8,
                23u8,
                20u8,
                136u8,
                121u8,
                68u8,
                185u8,
                224u8,
                254u8,
                224u8,
                95u8,
                50u8,
                14u8,
                66u8,
                161u8,
                177u8,
                58u8,
                1u8,
                251u8,
                198u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for EntropyEventsEvents {
        const NAME: &'static str = "EntropyEventsEvents";
        const COUNT: usize = 9usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <ProviderFeeManagerUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProviderFeeManagerUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProviderFeeManagerUpdated)
                }
                Some(
                    <ProviderFeeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProviderFeeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProviderFeeUpdated)
                }
                Some(
                    <ProviderUriUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProviderUriUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProviderUriUpdated)
                }
                Some(<Registered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Registered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Registered)
                }
                Some(<Requested as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Requested as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Requested)
                }
                Some(
                    <RequestedWithCallback as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RequestedWithCallback as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RequestedWithCallback)
                }
                Some(<Revealed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Revealed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Revealed)
                }
                Some(
                    <RevealedWithCallback as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RevealedWithCallback as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RevealedWithCallback)
                }
                Some(<Withdrawal as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Withdrawal as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Withdrawal)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for EntropyEventsEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ProviderFeeManagerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProviderFeeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProviderUriUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Registered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Requested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RequestedWithCallback(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Revealed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RevealedWithCallback(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Withdrawal(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ProviderFeeManagerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProviderFeeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProviderUriUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Registered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Requested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RequestedWithCallback(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Revealed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RevealedWithCallback(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Withdrawal(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EntropyEvents`](self) contract instance.

See the [wrapper's documentation](`EntropyEventsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EntropyEventsInstance<T, P, N> {
        EntropyEventsInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EntropyEventsInstance<T, P, N>>,
    > {
        EntropyEventsInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        EntropyEventsInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`EntropyEvents`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EntropyEvents`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EntropyEventsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EntropyEventsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EntropyEventsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EntropyEventsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EntropyEvents`](self) contract instance.

See the [wrapper's documentation](`EntropyEventsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<EntropyEventsInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> EntropyEventsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EntropyEventsInstance<T, P, N> {
            EntropyEventsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EntropyEventsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EntropyEventsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ProviderFeeManagerUpdated`] event.
        pub fn ProviderFeeManagerUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProviderFeeManagerUpdated, N> {
            self.event_filter::<ProviderFeeManagerUpdated>()
        }
        ///Creates a new event filter for the [`ProviderFeeUpdated`] event.
        pub fn ProviderFeeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProviderFeeUpdated, N> {
            self.event_filter::<ProviderFeeUpdated>()
        }
        ///Creates a new event filter for the [`ProviderUriUpdated`] event.
        pub fn ProviderUriUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProviderUriUpdated, N> {
            self.event_filter::<ProviderUriUpdated>()
        }
        ///Creates a new event filter for the [`Registered`] event.
        pub fn Registered_filter(&self) -> alloy_contract::Event<T, &P, Registered, N> {
            self.event_filter::<Registered>()
        }
        ///Creates a new event filter for the [`Requested`] event.
        pub fn Requested_filter(&self) -> alloy_contract::Event<T, &P, Requested, N> {
            self.event_filter::<Requested>()
        }
        ///Creates a new event filter for the [`RequestedWithCallback`] event.
        pub fn RequestedWithCallback_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RequestedWithCallback, N> {
            self.event_filter::<RequestedWithCallback>()
        }
        ///Creates a new event filter for the [`Revealed`] event.
        pub fn Revealed_filter(&self) -> alloy_contract::Event<T, &P, Revealed, N> {
            self.event_filter::<Revealed>()
        }
        ///Creates a new event filter for the [`RevealedWithCallback`] event.
        pub fn RevealedWithCallback_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RevealedWithCallback, N> {
            self.event_filter::<RevealedWithCallback>()
        }
        ///Creates a new event filter for the [`Withdrawal`] event.
        pub fn Withdrawal_filter(&self) -> alloy_contract::Event<T, &P, Withdrawal, N> {
            self.event_filter::<Withdrawal>()
        }
    }
}
