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

interface IMockEntropy {
    event ProviderFeeManagerUpdated(address provider, address oldFeeManager, address newFeeManager);
    event ProviderFeeUpdated(address provider, uint128 oldFee, uint128 newFee);
    event ProviderUriUpdated(address provider, bytes oldUri, bytes newUri);
    event Registered(EntropyStructs.ProviderInfo provider);
    event Requested(EntropyStructs.Request request);
    event RequestedWithCallback(address indexed provider, address indexed requestor, uint64 indexed sequenceNumber, bytes32 userRandomNumber, EntropyStructs.Request request);
    event Revealed(EntropyStructs.Request request, bytes32 userRevelation, bytes32 providerRevelation, bytes32 blockHash, bytes32 randomNumber);
    event RevealedWithCallback(EntropyStructs.Request request, bytes32 userRandomNumber, bytes32 providerRevelation, bytes32 randomNumber);
    event Withdrawal(address provider, address recipient, uint128 withdrawnAmount);

    function combineRandomValues(bytes32 userRandomness, bytes32 providerRandomness, bytes32 blockHash) external pure returns (bytes32 combinedRandomness);
    function constructUserCommitment(bytes32 userRandomness) external pure returns (bytes32 userCommitment);
    function getAccruedPythFees() external view returns (uint128 accruedPythFeesInWei);
    function getDefaultProvider() external view returns (address provider);
    function getFee(address provider) external view returns (uint128 feeAmount);
    function getLatestSequenceNumber(address provider) external view returns (uint64);
    function getProviderInfo(address provider) external view returns (EntropyStructs.ProviderInfo memory info);
    function getRequest(address provider, uint64 sequenceNumber) external view returns (EntropyStructs.Request memory req);
    function register(uint128 feeInWei, bytes32 commitment, bytes memory commitmentMetadata, uint64 chainLength, bytes memory uri) external;
    function request(address provider, bytes32 userCommitment, bool useBlockHash) external payable returns (uint64 assignedSequenceNumber);
    function requestWithCallback(address provider, bytes32 userRandomNumber) external payable returns (uint64 assignedSequenceNumber);
    function reveal(address provider, uint64 sequenceNumber, bytes32 userRevelation, bytes32 providerRevelation) external returns (bytes32 randomNumber);
    function revealWithCallback(address provider, uint64 sequenceNumber, bytes32 userRandomNumber, bytes32 providerRevelation) external;
    function setFeeManager(address manager) external;
    function setProviderFee(uint128 newFeeInWei) external;
    function setProviderFeeAsFeeManager(address provider, uint128 newFeeInWei) external;
    function setProviderUri(bytes memory newUri) external;
    function triggerCallback(uint64 sequenceNumber) external;
    function withdraw(uint128 amount) external;
    function withdrawAsFeeManager(address provider, uint128 amount) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "combineRandomValues",
    "inputs": [
      {
        "name": "userRandomness",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "providerRandomness",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "blockHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "combinedRandomness",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "constructUserCommitment",
    "inputs": [
      {
        "name": "userRandomness",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "userCommitment",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getAccruedPythFees",
    "inputs": [],
    "outputs": [
      {
        "name": "accruedPythFeesInWei",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDefaultProvider",
    "inputs": [],
    "outputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getFee",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "feeAmount",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLatestSequenceNumber",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getProviderInfo",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "info",
        "type": "tuple",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRequest",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "sequenceNumber",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "req",
        "type": "tuple",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "register",
    "inputs": [
      {
        "name": "feeInWei",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "commitment",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "commitmentMetadata",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "chainLength",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "uri",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "request",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "userCommitment",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "useBlockHash",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [
      {
        "name": "assignedSequenceNumber",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "requestWithCallback",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "userRandomNumber",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "assignedSequenceNumber",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "reveal",
    "inputs": [
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
        "name": "userRevelation",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "providerRevelation",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "randomNumber",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revealWithCallback",
    "inputs": [
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
        "name": "userRandomNumber",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "providerRevelation",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setFeeManager",
    "inputs": [
      {
        "name": "manager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setProviderFee",
    "inputs": [
      {
        "name": "newFeeInWei",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setProviderFeeAsFeeManager",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "newFeeInWei",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setProviderUri",
    "inputs": [
      {
        "name": "newUri",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "triggerCallback",
    "inputs": [
      {
        "name": "sequenceNumber",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdraw",
    "inputs": [
      {
        "name": "amount",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawAsFeeManager",
    "inputs": [
      {
        "name": "provider",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint128",
        "internalType": "uint128"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
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
pub mod IMockEntropy {
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
    /**Function with signature `combineRandomValues(bytes32,bytes32,bytes32)` and selector `0x14e82e8c`.
```solidity
function combineRandomValues(bytes32 userRandomness, bytes32 providerRandomness, bytes32 blockHash) external pure returns (bytes32 combinedRandomness);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct combineRandomValuesCall {
        #[allow(missing_docs)]
        pub userRandomness: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub providerRandomness: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`combineRandomValues(bytes32,bytes32,bytes32)`](combineRandomValuesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct combineRandomValuesReturn {
        #[allow(missing_docs)]
        pub combinedRandomness: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<combineRandomValuesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: combineRandomValuesCall) -> Self {
                    (value.userRandomness, value.providerRandomness, value.blockHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for combineRandomValuesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        userRandomness: tuple.0,
                        providerRandomness: tuple.1,
                        blockHash: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<combineRandomValuesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: combineRandomValuesReturn) -> Self {
                    (value.combinedRandomness,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for combineRandomValuesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        combinedRandomness: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for combineRandomValuesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = combineRandomValuesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "combineRandomValues(bytes32,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [20u8, 232u8, 46u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userRandomness),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.providerRandomness),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockHash),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `constructUserCommitment(bytes32)` and selector `0xc715aa2e`.
```solidity
function constructUserCommitment(bytes32 userRandomness) external pure returns (bytes32 userCommitment);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructUserCommitmentCall {
        #[allow(missing_docs)]
        pub userRandomness: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`constructUserCommitment(bytes32)`](constructUserCommitmentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructUserCommitmentReturn {
        #[allow(missing_docs)]
        pub userCommitment: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<constructUserCommitmentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: constructUserCommitmentCall) -> Self {
                    (value.userRandomness,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for constructUserCommitmentCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { userRandomness: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<constructUserCommitmentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: constructUserCommitmentReturn) -> Self {
                    (value.userCommitment,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for constructUserCommitmentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { userCommitment: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for constructUserCommitmentCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = constructUserCommitmentReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "constructUserCommitment(bytes32)";
            const SELECTOR: [u8; 4] = [199u8, 21u8, 170u8, 46u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userRandomness),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getAccruedPythFees()` and selector `0xc970835c`.
```solidity
function getAccruedPythFees() external view returns (uint128 accruedPythFeesInWei);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAccruedPythFeesCall {}
    ///Container type for the return parameters of the [`getAccruedPythFees()`](getAccruedPythFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAccruedPythFeesReturn {
        #[allow(missing_docs)]
        pub accruedPythFeesInWei: u128,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<getAccruedPythFeesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAccruedPythFeesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAccruedPythFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
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
            impl ::core::convert::From<getAccruedPythFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAccruedPythFeesReturn) -> Self {
                    (value.accruedPythFeesInWei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAccruedPythFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        accruedPythFeesInWei: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAccruedPythFeesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAccruedPythFeesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAccruedPythFees()";
            const SELECTOR: [u8; 4] = [201u8, 112u8, 131u8, 92u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getDefaultProvider()` and selector `0x82ee990c`.
```solidity
function getDefaultProvider() external view returns (address provider);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDefaultProviderCall {}
    ///Container type for the return parameters of the [`getDefaultProvider()`](getDefaultProviderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDefaultProviderReturn {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<getDefaultProviderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDefaultProviderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDefaultProviderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<getDefaultProviderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDefaultProviderReturn) -> Self {
                    (value.provider,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDefaultProviderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { provider: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDefaultProviderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDefaultProviderReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDefaultProvider()";
            const SELECTOR: [u8; 4] = [130u8, 238u8, 153u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getFee(address)` and selector `0xb88c9148`.
```solidity
function getFee(address provider) external view returns (uint128 feeAmount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getFeeCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getFee(address)`](getFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getFeeReturn {
        #[allow(missing_docs)]
        pub feeAmount: u128,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<getFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getFeeCall) -> Self {
                    (value.provider,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { provider: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
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
            impl ::core::convert::From<getFeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getFeeReturn) -> Self {
                    (value.feeAmount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { feeAmount: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getFeeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getFeeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getFee(address)";
            const SELECTOR: [u8; 4] = [184u8, 140u8, 145u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getLatestSequenceNumber(address)` and selector `0xcbf7053d`.
```solidity
function getLatestSequenceNumber(address provider) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestSequenceNumberCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLatestSequenceNumber(address)`](getLatestSequenceNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestSequenceNumberReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<getLatestSequenceNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestSequenceNumberCall) -> Self {
                    (value.provider,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestSequenceNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { provider: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<getLatestSequenceNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestSequenceNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestSequenceNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestSequenceNumberCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestSequenceNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestSequenceNumber(address)";
            const SELECTOR: [u8; 4] = [203u8, 247u8, 5u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getProviderInfo(address)` and selector `0x7583902f`.
```solidity
function getProviderInfo(address provider) external view returns (EntropyStructs.ProviderInfo memory info);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getProviderInfoCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getProviderInfo(address)`](getProviderInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getProviderInfoReturn {
        #[allow(missing_docs)]
        pub info: <EntropyStructs::ProviderInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<getProviderInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: getProviderInfoCall) -> Self {
                    (value.provider,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getProviderInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { provider: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (EntropyStructs::ProviderInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <EntropyStructs::ProviderInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getProviderInfoReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getProviderInfoReturn) -> Self {
                    (value.info,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getProviderInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { info: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getProviderInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getProviderInfoReturn;
            type ReturnTuple<'a> = (EntropyStructs::ProviderInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getProviderInfo(address)";
            const SELECTOR: [u8; 4] = [117u8, 131u8, 144u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getRequest(address,uint64)` and selector `0x6151ab1f`.
```solidity
function getRequest(address provider, uint64 sequenceNumber) external view returns (EntropyStructs.Request memory req);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequestCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequenceNumber: u64,
    }
    ///Container type for the return parameters of the [`getRequest(address,uint64)`](getRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRequestReturn {
        #[allow(missing_docs)]
        pub req: <EntropyStructs::Request as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u64);
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
            impl ::core::convert::From<getRequestCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRequestCall) -> Self {
                    (value.provider, value.sequenceNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        provider: tuple.0,
                        sequenceNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (EntropyStructs::Request,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <EntropyStructs::Request as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getRequestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRequestReturn) -> Self {
                    (value.req,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { req: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRequestReturn;
            type ReturnTuple<'a> = (EntropyStructs::Request,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRequest(address,uint64)";
            const SELECTOR: [u8; 4] = [97u8, 81u8, 171u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `register(uint128,bytes32,bytes,uint64,bytes)` and selector `0x38b049c6`.
```solidity
function register(uint128 feeInWei, bytes32 commitment, bytes memory commitmentMetadata, uint64 chainLength, bytes memory uri) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerCall {
        #[allow(missing_docs)]
        pub feeInWei: u128,
        #[allow(missing_docs)]
        pub commitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub commitmentMetadata: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub chainLength: u64,
        #[allow(missing_docs)]
        pub uri: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`register(uint128,bytes32,bytes,uint64,bytes)`](registerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u128,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                u64,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<registerCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerCall) -> Self {
                    (
                        value.feeInWei,
                        value.commitment,
                        value.commitmentMetadata,
                        value.chainLength,
                        value.uri,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        feeInWei: tuple.0,
                        commitment: tuple.1,
                        commitmentMetadata: tuple.2,
                        chainLength: tuple.3,
                        uri: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<registerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "register(uint128,bytes32,bytes,uint64,bytes)";
            const SELECTOR: [u8; 4] = [56u8, 176u8, 73u8, 198u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.feeInWei),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitment),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.commitmentMetadata,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainLength),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.uri,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `request(address,bytes32,bool)` and selector `0x93cbf217`.
```solidity
function request(address provider, bytes32 userCommitment, bool useBlockHash) external payable returns (uint64 assignedSequenceNumber);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub userCommitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub useBlockHash: bool,
    }
    ///Container type for the return parameters of the [`request(address,bytes32,bool)`](requestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestReturn {
        #[allow(missing_docs)]
        pub assignedSequenceNumber: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<requestCall> for UnderlyingRustTuple<'_> {
                fn from(value: requestCall) -> Self {
                    (value.provider, value.userCommitment, value.useBlockHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        provider: tuple.0,
                        userCommitment: tuple.1,
                        useBlockHash: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<requestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: requestReturn) -> Self {
                    (value.assignedSequenceNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assignedSequenceNumber: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "request(address,bytes32,bool)";
            const SELECTOR: [u8; 4] = [147u8, 203u8, 242u8, 23u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userCommitment),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.useBlockHash,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `requestWithCallback(address,bytes32)` and selector `0x19cb825f`.
```solidity
function requestWithCallback(address provider, bytes32 userRandomNumber) external payable returns (uint64 assignedSequenceNumber);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestWithCallbackCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub userRandomNumber: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`requestWithCallback(address,bytes32)`](requestWithCallbackCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestWithCallbackReturn {
        #[allow(missing_docs)]
        pub assignedSequenceNumber: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<requestWithCallbackCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestWithCallbackCall) -> Self {
                    (value.provider, value.userRandomNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestWithCallbackCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        provider: tuple.0,
                        userRandomNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<requestWithCallbackReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: requestWithCallbackReturn) -> Self {
                    (value.assignedSequenceNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for requestWithCallbackReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assignedSequenceNumber: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestWithCallbackCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestWithCallbackReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestWithCallback(address,bytes32)";
            const SELECTOR: [u8; 4] = [25u8, 203u8, 130u8, 95u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userRandomNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `reveal(address,uint64,bytes32,bytes32)` and selector `0x9371df51`.
```solidity
function reveal(address provider, uint64 sequenceNumber, bytes32 userRevelation, bytes32 providerRevelation) external returns (bytes32 randomNumber);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revealCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequenceNumber: u64,
        #[allow(missing_docs)]
        pub userRevelation: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub providerRevelation: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`reveal(address,uint64,bytes32,bytes32)`](revealCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revealReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u64,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<revealCall> for UnderlyingRustTuple<'_> {
                fn from(value: revealCall) -> Self {
                    (
                        value.provider,
                        value.sequenceNumber,
                        value.userRevelation,
                        value.providerRevelation,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revealCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        provider: tuple.0,
                        sequenceNumber: tuple.1,
                        userRevelation: tuple.2,
                        providerRevelation: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
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
            impl ::core::convert::From<revealReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revealReturn) -> Self {
                    (value.randomNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revealReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { randomNumber: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revealCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revealReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reveal(address,uint64,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [147u8, 113u8, 223u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userRevelation),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.providerRevelation),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `revealWithCallback(address,uint64,bytes32,bytes32)` and selector `0x3d30bc0e`.
```solidity
function revealWithCallback(address provider, uint64 sequenceNumber, bytes32 userRandomNumber, bytes32 providerRevelation) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revealWithCallbackCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sequenceNumber: u64,
        #[allow(missing_docs)]
        pub userRandomNumber: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub providerRevelation: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`revealWithCallback(address,uint64,bytes32,bytes32)`](revealWithCallbackCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revealWithCallbackReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u64,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<revealWithCallbackCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: revealWithCallbackCall) -> Self {
                    (
                        value.provider,
                        value.sequenceNumber,
                        value.userRandomNumber,
                        value.providerRevelation,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for revealWithCallbackCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        provider: tuple.0,
                        sequenceNumber: tuple.1,
                        userRandomNumber: tuple.2,
                        providerRevelation: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<revealWithCallbackReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: revealWithCallbackReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for revealWithCallbackReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revealWithCallbackCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = revealWithCallbackReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revealWithCallback(address,uint64,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [61u8, 48u8, 188u8, 14u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.userRandomNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.providerRevelation),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setFeeManager(address)` and selector `0x472d35b9`.
```solidity
function setFeeManager(address manager) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFeeManagerCall {
        #[allow(missing_docs)]
        pub manager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setFeeManager(address)`](setFeeManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFeeManagerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
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
            impl ::core::convert::From<setFeeManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: setFeeManagerCall) -> Self {
                    (value.manager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFeeManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { manager: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<setFeeManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setFeeManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFeeManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setFeeManagerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setFeeManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setFeeManager(address)";
            const SELECTOR: [u8; 4] = [71u8, 45u8, 53u8, 185u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.manager,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setProviderFee(uint128)` and selector `0xace63a7e`.
```solidity
function setProviderFee(uint128 newFeeInWei) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProviderFeeCall {
        #[allow(missing_docs)]
        pub newFeeInWei: u128,
    }
    ///Container type for the return parameters of the [`setProviderFee(uint128)`](setProviderFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProviderFeeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
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
            impl ::core::convert::From<setProviderFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setProviderFeeCall) -> Self {
                    (value.newFeeInWei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setProviderFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newFeeInWei: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<setProviderFeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProviderFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProviderFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProviderFeeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProviderFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProviderFee(uint128)";
            const SELECTOR: [u8; 4] = [172u8, 230u8, 58u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.newFeeInWei),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setProviderFeeAsFeeManager(address,uint128)` and selector `0xc03c035d`.
```solidity
function setProviderFeeAsFeeManager(address provider, uint128 newFeeInWei) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProviderFeeAsFeeManagerCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newFeeInWei: u128,
    }
    ///Container type for the return parameters of the [`setProviderFeeAsFeeManager(address,uint128)`](setProviderFeeAsFeeManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProviderFeeAsFeeManagerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u128);
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
            impl ::core::convert::From<setProviderFeeAsFeeManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProviderFeeAsFeeManagerCall) -> Self {
                    (value.provider, value.newFeeInWei)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProviderFeeAsFeeManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        provider: tuple.0,
                        newFeeInWei: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<setProviderFeeAsFeeManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProviderFeeAsFeeManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProviderFeeAsFeeManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProviderFeeAsFeeManagerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProviderFeeAsFeeManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProviderFeeAsFeeManager(address,uint128)";
            const SELECTOR: [u8; 4] = [192u8, 60u8, 3u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.newFeeInWei),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setProviderUri(bytes)` and selector `0xb469f1c9`.
```solidity
function setProviderUri(bytes memory newUri) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProviderUriCall {
        #[allow(missing_docs)]
        pub newUri: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`setProviderUri(bytes)`](setProviderUriCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProviderUriReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
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
            impl ::core::convert::From<setProviderUriCall> for UnderlyingRustTuple<'_> {
                fn from(value: setProviderUriCall) -> Self {
                    (value.newUri,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setProviderUriCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newUri: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<setProviderUriReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProviderUriReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProviderUriReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProviderUriCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProviderUriReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProviderUri(bytes)";
            const SELECTOR: [u8; 4] = [180u8, 105u8, 241u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.newUri,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `triggerCallback(uint64)` and selector `0xbd21ecd8`.
```solidity
function triggerCallback(uint64 sequenceNumber) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggerCallbackCall {
        #[allow(missing_docs)]
        pub sequenceNumber: u64,
    }
    ///Container type for the return parameters of the [`triggerCallback(uint64)`](triggerCallbackCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggerCallbackReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<triggerCallbackCall> for UnderlyingRustTuple<'_> {
                fn from(value: triggerCallbackCall) -> Self {
                    (value.sequenceNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggerCallbackCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { sequenceNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<triggerCallbackReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: triggerCallbackReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for triggerCallbackReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for triggerCallbackCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = triggerCallbackReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "triggerCallback(uint64)";
            const SELECTOR: [u8; 4] = [189u8, 33u8, 236u8, 216u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.sequenceNumber),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `withdraw(uint128)` and selector `0x02387a7b`.
```solidity
function withdraw(uint128 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawCall {
        #[allow(missing_docs)]
        pub amount: u128,
    }
    ///Container type for the return parameters of the [`withdraw(uint128)`](withdrawCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u128,);
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
            impl ::core::convert::From<withdrawCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawCall) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<withdrawReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<128>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdraw(uint128)";
            const SELECTOR: [u8; 4] = [2u8, 56u8, 122u8, 123u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `withdrawAsFeeManager(address,uint128)` and selector `0x308fe218`.
```solidity
function withdrawAsFeeManager(address provider, uint128 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawAsFeeManagerCall {
        #[allow(missing_docs)]
        pub provider: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: u128,
    }
    ///Container type for the return parameters of the [`withdrawAsFeeManager(address,uint128)`](withdrawAsFeeManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawAsFeeManagerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u128);
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
            impl ::core::convert::From<withdrawAsFeeManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawAsFeeManagerCall) -> Self {
                    (value.provider, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawAsFeeManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        provider: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
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
            impl ::core::convert::From<withdrawAsFeeManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawAsFeeManagerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawAsFeeManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawAsFeeManagerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<128>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawAsFeeManagerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawAsFeeManager(address,uint128)";
            const SELECTOR: [u8; 4] = [48u8, 143u8, 226u8, 24u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.provider,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`IMockEntropy`](self) function calls.
    pub enum IMockEntropyCalls {
        #[allow(missing_docs)]
        combineRandomValues(combineRandomValuesCall),
        #[allow(missing_docs)]
        constructUserCommitment(constructUserCommitmentCall),
        #[allow(missing_docs)]
        getAccruedPythFees(getAccruedPythFeesCall),
        #[allow(missing_docs)]
        getDefaultProvider(getDefaultProviderCall),
        #[allow(missing_docs)]
        getFee(getFeeCall),
        #[allow(missing_docs)]
        getLatestSequenceNumber(getLatestSequenceNumberCall),
        #[allow(missing_docs)]
        getProviderInfo(getProviderInfoCall),
        #[allow(missing_docs)]
        getRequest(getRequestCall),
        #[allow(missing_docs)]
        register(registerCall),
        #[allow(missing_docs)]
        request(requestCall),
        #[allow(missing_docs)]
        requestWithCallback(requestWithCallbackCall),
        #[allow(missing_docs)]
        reveal(revealCall),
        #[allow(missing_docs)]
        revealWithCallback(revealWithCallbackCall),
        #[allow(missing_docs)]
        setFeeManager(setFeeManagerCall),
        #[allow(missing_docs)]
        setProviderFee(setProviderFeeCall),
        #[allow(missing_docs)]
        setProviderFeeAsFeeManager(setProviderFeeAsFeeManagerCall),
        #[allow(missing_docs)]
        setProviderUri(setProviderUriCall),
        #[allow(missing_docs)]
        triggerCallback(triggerCallbackCall),
        #[allow(missing_docs)]
        withdraw(withdrawCall),
        #[allow(missing_docs)]
        withdrawAsFeeManager(withdrawAsFeeManagerCall),
    }
    #[automatically_derived]
    impl IMockEntropyCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [2u8, 56u8, 122u8, 123u8],
            [20u8, 232u8, 46u8, 140u8],
            [25u8, 203u8, 130u8, 95u8],
            [48u8, 143u8, 226u8, 24u8],
            [56u8, 176u8, 73u8, 198u8],
            [61u8, 48u8, 188u8, 14u8],
            [71u8, 45u8, 53u8, 185u8],
            [97u8, 81u8, 171u8, 31u8],
            [117u8, 131u8, 144u8, 47u8],
            [130u8, 238u8, 153u8, 12u8],
            [147u8, 113u8, 223u8, 81u8],
            [147u8, 203u8, 242u8, 23u8],
            [172u8, 230u8, 58u8, 126u8],
            [180u8, 105u8, 241u8, 201u8],
            [184u8, 140u8, 145u8, 72u8],
            [189u8, 33u8, 236u8, 216u8],
            [192u8, 60u8, 3u8, 93u8],
            [199u8, 21u8, 170u8, 46u8],
            [201u8, 112u8, 131u8, 92u8],
            [203u8, 247u8, 5u8, 61u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IMockEntropyCalls {
        const NAME: &'static str = "IMockEntropyCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::combineRandomValues(_) => {
                    <combineRandomValuesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::constructUserCommitment(_) => {
                    <constructUserCommitmentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAccruedPythFees(_) => {
                    <getAccruedPythFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDefaultProvider(_) => {
                    <getDefaultProviderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getFee(_) => <getFeeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getLatestSequenceNumber(_) => {
                    <getLatestSequenceNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getProviderInfo(_) => {
                    <getProviderInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRequest(_) => {
                    <getRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::register(_) => <registerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::request(_) => <requestCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::requestWithCallback(_) => {
                    <requestWithCallbackCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reveal(_) => <revealCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::revealWithCallback(_) => {
                    <revealWithCallbackCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setFeeManager(_) => {
                    <setFeeManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProviderFee(_) => {
                    <setProviderFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProviderFeeAsFeeManager(_) => {
                    <setProviderFeeAsFeeManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProviderUri(_) => {
                    <setProviderUriCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::triggerCallback(_) => {
                    <triggerCallbackCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdraw(_) => <withdrawCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::withdrawAsFeeManager(_) => {
                    <withdrawAsFeeManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<IMockEntropyCalls>] = &[
                {
                    fn withdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <withdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::withdraw)
                    }
                    withdraw
                },
                {
                    fn combineRandomValues(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <combineRandomValuesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::combineRandomValues)
                    }
                    combineRandomValues
                },
                {
                    fn requestWithCallback(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <requestWithCallbackCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::requestWithCallback)
                    }
                    requestWithCallback
                },
                {
                    fn withdrawAsFeeManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <withdrawAsFeeManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::withdrawAsFeeManager)
                    }
                    withdrawAsFeeManager
                },
                {
                    fn register(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <registerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::register)
                    }
                    register
                },
                {
                    fn revealWithCallback(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <revealWithCallbackCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::revealWithCallback)
                    }
                    revealWithCallback
                },
                {
                    fn setFeeManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <setFeeManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::setFeeManager)
                    }
                    setFeeManager
                },
                {
                    fn getRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <getRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::getRequest)
                    }
                    getRequest
                },
                {
                    fn getProviderInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <getProviderInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::getProviderInfo)
                    }
                    getProviderInfo
                },
                {
                    fn getDefaultProvider(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <getDefaultProviderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::getDefaultProvider)
                    }
                    getDefaultProvider
                },
                {
                    fn reveal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <revealCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::reveal)
                    }
                    reveal
                },
                {
                    fn request(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <requestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::request)
                    }
                    request
                },
                {
                    fn setProviderFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <setProviderFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::setProviderFee)
                    }
                    setProviderFee
                },
                {
                    fn setProviderUri(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <setProviderUriCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::setProviderUri)
                    }
                    setProviderUri
                },
                {
                    fn getFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <getFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::getFee)
                    }
                    getFee
                },
                {
                    fn triggerCallback(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <triggerCallbackCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::triggerCallback)
                    }
                    triggerCallback
                },
                {
                    fn setProviderFeeAsFeeManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <setProviderFeeAsFeeManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::setProviderFeeAsFeeManager)
                    }
                    setProviderFeeAsFeeManager
                },
                {
                    fn constructUserCommitment(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <constructUserCommitmentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::constructUserCommitment)
                    }
                    constructUserCommitment
                },
                {
                    fn getAccruedPythFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <getAccruedPythFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::getAccruedPythFees)
                    }
                    getAccruedPythFees
                },
                {
                    fn getLatestSequenceNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<IMockEntropyCalls> {
                        <getLatestSequenceNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(IMockEntropyCalls::getLatestSequenceNumber)
                    }
                    getLatestSequenceNumber
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::combineRandomValues(inner) => {
                    <combineRandomValuesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::constructUserCommitment(inner) => {
                    <constructUserCommitmentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAccruedPythFees(inner) => {
                    <getAccruedPythFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDefaultProvider(inner) => {
                    <getDefaultProviderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getFee(inner) => {
                    <getFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getLatestSequenceNumber(inner) => {
                    <getLatestSequenceNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getProviderInfo(inner) => {
                    <getProviderInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRequest(inner) => {
                    <getRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::register(inner) => {
                    <registerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::request(inner) => {
                    <requestCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::requestWithCallback(inner) => {
                    <requestWithCallbackCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::reveal(inner) => {
                    <revealCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::revealWithCallback(inner) => {
                    <revealWithCallbackCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setFeeManager(inner) => {
                    <setFeeManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProviderFee(inner) => {
                    <setProviderFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProviderFeeAsFeeManager(inner) => {
                    <setProviderFeeAsFeeManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProviderUri(inner) => {
                    <setProviderUriCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::triggerCallback(inner) => {
                    <triggerCallbackCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdraw(inner) => {
                    <withdrawCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdrawAsFeeManager(inner) => {
                    <withdrawAsFeeManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::combineRandomValues(inner) => {
                    <combineRandomValuesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::constructUserCommitment(inner) => {
                    <constructUserCommitmentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAccruedPythFees(inner) => {
                    <getAccruedPythFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDefaultProvider(inner) => {
                    <getDefaultProviderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getFee(inner) => {
                    <getFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getLatestSequenceNumber(inner) => {
                    <getLatestSequenceNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getProviderInfo(inner) => {
                    <getProviderInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRequest(inner) => {
                    <getRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::register(inner) => {
                    <registerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::request(inner) => {
                    <requestCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::requestWithCallback(inner) => {
                    <requestWithCallbackCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::reveal(inner) => {
                    <revealCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::revealWithCallback(inner) => {
                    <revealWithCallbackCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setFeeManager(inner) => {
                    <setFeeManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setProviderFee(inner) => {
                    <setProviderFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setProviderFeeAsFeeManager(inner) => {
                    <setProviderFeeAsFeeManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setProviderUri(inner) => {
                    <setProviderUriCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::triggerCallback(inner) => {
                    <triggerCallbackCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdraw(inner) => {
                    <withdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawAsFeeManager(inner) => {
                    <withdrawAsFeeManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`IMockEntropy`](self) events.
    pub enum IMockEntropyEvents {
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
    impl IMockEntropyEvents {
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
    impl alloy_sol_types::SolEventInterface for IMockEntropyEvents {
        const NAME: &'static str = "IMockEntropyEvents";
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
    impl alloy_sol_types::private::IntoLogData for IMockEntropyEvents {
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
    /**Creates a new wrapper around an on-chain [`IMockEntropy`](self) contract instance.

See the [wrapper's documentation](`IMockEntropyInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IMockEntropyInstance<T, P, N> {
        IMockEntropyInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<IMockEntropyInstance<T, P, N>>,
    > {
        IMockEntropyInstance::<T, P, N>::deploy(provider)
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
        IMockEntropyInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`IMockEntropy`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IMockEntropy`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IMockEntropyInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IMockEntropyInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IMockEntropyInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IMockEntropyInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IMockEntropy`](self) contract instance.

See the [wrapper's documentation](`IMockEntropyInstance`) for more details.*/
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
        ) -> alloy_contract::Result<IMockEntropyInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> IMockEntropyInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IMockEntropyInstance<T, P, N> {
            IMockEntropyInstance {
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
    > IMockEntropyInstance<T, P, N> {
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
        ///Creates a new call builder for the [`combineRandomValues`] function.
        pub fn combineRandomValues(
            &self,
            userRandomness: alloy::sol_types::private::FixedBytes<32>,
            providerRandomness: alloy::sol_types::private::FixedBytes<32>,
            blockHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, combineRandomValuesCall, N> {
            self.call_builder(
                &combineRandomValuesCall {
                    userRandomness,
                    providerRandomness,
                    blockHash,
                },
            )
        }
        ///Creates a new call builder for the [`constructUserCommitment`] function.
        pub fn constructUserCommitment(
            &self,
            userRandomness: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, constructUserCommitmentCall, N> {
            self.call_builder(
                &constructUserCommitmentCall {
                    userRandomness,
                },
            )
        }
        ///Creates a new call builder for the [`getAccruedPythFees`] function.
        pub fn getAccruedPythFees(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAccruedPythFeesCall, N> {
            self.call_builder(&getAccruedPythFeesCall {})
        }
        ///Creates a new call builder for the [`getDefaultProvider`] function.
        pub fn getDefaultProvider(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDefaultProviderCall, N> {
            self.call_builder(&getDefaultProviderCall {})
        }
        ///Creates a new call builder for the [`getFee`] function.
        pub fn getFee(
            &self,
            provider: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getFeeCall, N> {
            self.call_builder(&getFeeCall { provider })
        }
        ///Creates a new call builder for the [`getLatestSequenceNumber`] function.
        pub fn getLatestSequenceNumber(
            &self,
            provider: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestSequenceNumberCall, N> {
            self.call_builder(
                &getLatestSequenceNumberCall {
                    provider,
                },
            )
        }
        ///Creates a new call builder for the [`getProviderInfo`] function.
        pub fn getProviderInfo(
            &self,
            provider: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getProviderInfoCall, N> {
            self.call_builder(&getProviderInfoCall { provider })
        }
        ///Creates a new call builder for the [`getRequest`] function.
        pub fn getRequest(
            &self,
            provider: alloy::sol_types::private::Address,
            sequenceNumber: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRequestCall, N> {
            self.call_builder(
                &getRequestCall {
                    provider,
                    sequenceNumber,
                },
            )
        }
        ///Creates a new call builder for the [`register`] function.
        pub fn register(
            &self,
            feeInWei: u128,
            commitment: alloy::sol_types::private::FixedBytes<32>,
            commitmentMetadata: alloy::sol_types::private::Bytes,
            chainLength: u64,
            uri: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerCall, N> {
            self.call_builder(
                &registerCall {
                    feeInWei,
                    commitment,
                    commitmentMetadata,
                    chainLength,
                    uri,
                },
            )
        }
        ///Creates a new call builder for the [`request`] function.
        pub fn request(
            &self,
            provider: alloy::sol_types::private::Address,
            userCommitment: alloy::sol_types::private::FixedBytes<32>,
            useBlockHash: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, requestCall, N> {
            self.call_builder(
                &requestCall {
                    provider,
                    userCommitment,
                    useBlockHash,
                },
            )
        }
        ///Creates a new call builder for the [`requestWithCallback`] function.
        pub fn requestWithCallback(
            &self,
            provider: alloy::sol_types::private::Address,
            userRandomNumber: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, requestWithCallbackCall, N> {
            self.call_builder(
                &requestWithCallbackCall {
                    provider,
                    userRandomNumber,
                },
            )
        }
        ///Creates a new call builder for the [`reveal`] function.
        pub fn reveal(
            &self,
            provider: alloy::sol_types::private::Address,
            sequenceNumber: u64,
            userRevelation: alloy::sol_types::private::FixedBytes<32>,
            providerRevelation: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, revealCall, N> {
            self.call_builder(
                &revealCall {
                    provider,
                    sequenceNumber,
                    userRevelation,
                    providerRevelation,
                },
            )
        }
        ///Creates a new call builder for the [`revealWithCallback`] function.
        pub fn revealWithCallback(
            &self,
            provider: alloy::sol_types::private::Address,
            sequenceNumber: u64,
            userRandomNumber: alloy::sol_types::private::FixedBytes<32>,
            providerRevelation: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, revealWithCallbackCall, N> {
            self.call_builder(
                &revealWithCallbackCall {
                    provider,
                    sequenceNumber,
                    userRandomNumber,
                    providerRevelation,
                },
            )
        }
        ///Creates a new call builder for the [`setFeeManager`] function.
        pub fn setFeeManager(
            &self,
            manager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setFeeManagerCall, N> {
            self.call_builder(&setFeeManagerCall { manager })
        }
        ///Creates a new call builder for the [`setProviderFee`] function.
        pub fn setProviderFee(
            &self,
            newFeeInWei: u128,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProviderFeeCall, N> {
            self.call_builder(&setProviderFeeCall { newFeeInWei })
        }
        ///Creates a new call builder for the [`setProviderFeeAsFeeManager`] function.
        pub fn setProviderFeeAsFeeManager(
            &self,
            provider: alloy::sol_types::private::Address,
            newFeeInWei: u128,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProviderFeeAsFeeManagerCall, N> {
            self.call_builder(
                &setProviderFeeAsFeeManagerCall {
                    provider,
                    newFeeInWei,
                },
            )
        }
        ///Creates a new call builder for the [`setProviderUri`] function.
        pub fn setProviderUri(
            &self,
            newUri: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProviderUriCall, N> {
            self.call_builder(&setProviderUriCall { newUri })
        }
        ///Creates a new call builder for the [`triggerCallback`] function.
        pub fn triggerCallback(
            &self,
            sequenceNumber: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, triggerCallbackCall, N> {
            self.call_builder(
                &triggerCallbackCall {
                    sequenceNumber,
                },
            )
        }
        ///Creates a new call builder for the [`withdraw`] function.
        pub fn withdraw(
            &self,
            amount: u128,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawCall, N> {
            self.call_builder(&withdrawCall { amount })
        }
        ///Creates a new call builder for the [`withdrawAsFeeManager`] function.
        pub fn withdrawAsFeeManager(
            &self,
            provider: alloy::sol_types::private::Address,
            amount: u128,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawAsFeeManagerCall, N> {
            self.call_builder(
                &withdrawAsFeeManagerCall {
                    provider,
                    amount,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IMockEntropyInstance<T, P, N> {
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
