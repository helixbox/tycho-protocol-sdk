use crate::{abi, modules::VAULT_ADDRESS};
use substreams::hex;
use substreams_ethereum::{
    pb::eth::v2::{Call, Log, TransactionTrace},
    Event, Function,
};
use tycho_substreams::{
    attributes::{json_serialize_address_list, json_serialize_bigint_list},
    prelude::*,
};

/// Helper function to get pool_registered event
fn get_pool_registered(
    tx: &TransactionTrace,
    pool_address: &Vec<u8>,
) -> abi::vault::events::PoolRegistered {
    tx.logs_with_calls()
        .filter(|(log, _)| log.address == VAULT_ADDRESS)
        .filter_map(|(log, _)| abi::vault::events::PoolRegistered::match_and_decode(log))
        .find(|pool| pool.pool_address == *pool_address)
        .unwrap()
        .clone()
}

fn get_token_registered(
    tx: &TransactionTrace,
    pool_id: &[u8],
) -> abi::vault::events::TokensRegistered {
    tx.logs_with_calls()
        .filter(|(log, _)| log.address == VAULT_ADDRESS)
        .filter_map(|(log, _)| abi::vault::events::TokensRegistered::match_and_decode(log))
        .find(|ev| ev.pool_id == pool_id)
        .unwrap()
        .clone()
}

// This is the main function that handles the creation of `ProtocolComponent`s with `Attribute`s
//  based on the specific factory address. There's 3 factory groups that are represented here:
//  - Weighted Pool Factories
//  - Linear Pool Factories
//  - Stable Pool Factories
// (Balancer does have a bit more (esp. in the deprecated section) that could be implemented as
//  desired.)
// We use the specific ABIs to decode both the log event and corresponding call to gather
//  `PoolCreated` event information alongside the `Create` call data that provide us details to
//  fulfill both the required details + any extra `Attributes`
// Ref: https://docs-v2.balancer.fi/reference/contracts/deployment-addresses/arbitrum.html
pub fn address_map(
    pool_factory_address: &[u8],
    log: &Log,
    call: &Call,
    tx: &TransactionTrace,
) -> Option<ProtocolComponent> {
    match *pool_factory_address {
        hex!("7dFdEF5f355096603419239CE743BfaF1120312B") => {
            let create_call =
                abi::weighted_pool_factory_v1::functions::Create::match_and_decode(call)?;
            let pool_created =
                abi::weighted_pool_factory_v1::events::PoolCreated::match_and_decode(log)?;
            let pool_registered = get_pool_registered(tx, &pool_created.pool);

            Some(
                ProtocolComponent::new(
                    &format!("0x{}", hex::encode(pool_registered.pool_id))
                )
                .with_contracts(&[pool_created.pool, VAULT_ADDRESS.to_vec()])
                .with_tokens(&create_call.tokens)
                .with_attributes(&[
                    ("pool_type", "WeightedPoolFactoryV1".as_bytes()),
                    ("normalized_weights", &json_serialize_bigint_list(&create_call.weights)),
                    (
                        "fee",
                        &create_call
                            .swap_fee_percentage
                            .to_signed_bytes_be(),
                    ),
                    ("manual_updates", &[1u8]),
                ])
                .as_swap_type("balancer_v2_pool", ImplementationType::Vm),
            )
        }
        hex!("8df6EfEc5547e31B0eb7d1291B511FF8a2bf987c") => {
            let create_call =
                abi::weighted_pool_factory_v2::functions::Create::match_and_decode(call)?;
            let pool_created =
                abi::weighted_pool_factory_v2::events::PoolCreated::match_and_decode(log)?;
            let pool_registered = get_pool_registered(tx, &pool_created.pool);

            Some(
                ProtocolComponent::new(
                    &format!("0x{}", hex::encode(pool_registered.pool_id))
                )
                .with_contracts(&[pool_created.pool, VAULT_ADDRESS.to_vec()])
                .with_tokens(&create_call.tokens)
                .with_attributes(&[
                    ("pool_type", "WeightedPoolFactoryV2".as_bytes()),
                    (
                        "normalized_weights",
                        &json_serialize_bigint_list(&create_call.normalized_weights),
                    ),
                    ("rate_providers", &json_serialize_address_list(&create_call.rate_providers)),
                    (
                        "fee",
                        &create_call
                            .swap_fee_percentage
                            .to_signed_bytes_be(),
                    ),
                    ("manual_updates", &[1u8]),
                ])
                .as_swap_type("balancer_v2_pool", ImplementationType::Vm),
            )
        }
        hex!("f1665E19bc105BE4EDD3739F88315cC699cc5b65") => {
            let create_call =
                abi::weighted_pool_factory_v3::functions::Create::match_and_decode(call)?;
            let pool_created =
                abi::weighted_pool_factory_v3::events::PoolCreated::match_and_decode(log)?;
            let pool_registered = get_pool_registered(tx, &pool_created.pool);

            Some(
                ProtocolComponent::new(
                    &format!("0x{}", hex::encode(pool_registered.pool_id))
                )
                .with_contracts(&[pool_created.pool, VAULT_ADDRESS.to_vec()])
                .with_tokens(&create_call.tokens)
                .with_attributes(&[
                    ("pool_type", "WeightedPoolFactoryV3".as_bytes()),
                    (
                        "normalized_weights",
                        &json_serialize_bigint_list(&create_call.normalized_weights),
                    ),
                    ("rate_providers", &json_serialize_address_list(&create_call.rate_providers)),
                    (
                        "fee",
                        &create_call
                            .swap_fee_percentage
                            .to_signed_bytes_be(),
                    ),
                    ("manual_updates", &[1u8]),
                ])
                .as_swap_type("balancer_v2_pool", ImplementationType::Vm),
            )
        }
        hex!("c7E5ED1054A24Ef31D827E6F86caA58B3Bc168d7") => {
            let create_call =
                abi::weighted_pool_factory_v4::functions::Create::match_and_decode(call)?;
            let pool_created =
                abi::weighted_pool_factory_v4::events::PoolCreated::match_and_decode(log)?;
            let pool_registered = get_pool_registered(tx, &pool_created.pool);

            Some(
                ProtocolComponent::new(
                    &format!("0x{}", hex::encode(pool_registered.pool_id))
                )
                .with_contracts(&[pool_created.pool, VAULT_ADDRESS.to_vec()])
                .with_tokens(&create_call.tokens)
                .with_attributes(&[
                    ("pool_type", "WeightedPoolFactoryV4".as_bytes()),
                    (
                        "normalized_weights",
                        &json_serialize_bigint_list(&create_call.normalized_weights),
                    ),
                    ("rate_providers", &json_serialize_address_list(&create_call.rate_providers)),
                    (
                        "fee",
                        &create_call
                            .swap_fee_percentage
                            .to_signed_bytes_be(),
                    ),
                    ("manual_updates", &[1u8]),
                ])
                .as_swap_type("balancer_v2_pool", ImplementationType::Vm),
            )
        }
        hex!("A8920455934Da4D853faac1f94Fe7bEf72943eF1") => {
            let create_call =
                abi::composable_stable_pool_factory::functions::Create::match_and_decode(call)?;
            let pool_created =
                abi::composable_stable_pool_factory::events::PoolCreated::match_and_decode(log)?;
            let pool_registered = get_pool_registered(tx, &pool_created.pool);
            let tokens_registered = get_token_registered(tx, &pool_registered.pool_id);

            Some(
                ProtocolComponent::new(
                    &format!("0x{}", hex::encode(pool_registered.pool_id))
                )
                .with_contracts(&[pool_created.pool.clone(), VAULT_ADDRESS.to_vec()])
                .with_tokens(&tokens_registered.tokens)
                .with_attributes(&[
                    ("pool_type", "ComposableStablePoolFactory".as_bytes()),
                    ("bpt", &pool_created.pool),
                    (
                        "fee",
                        &create_call
                            .swap_fee_percentage
                            .to_signed_bytes_be(),
                    ),
                    ("rate_providers", &json_serialize_address_list(&create_call.rate_providers)),
                    ("manual_updates", &[1u8]),
                ])
                .as_swap_type("balancer_v2_pool", ImplementationType::Vm),
            )
        }
        hex!("7ADbdabaA80F654568421887c12F09E0C7BD9629") => {
            let create_call =
                abi::erc_linear_pool_factory::functions::Create::match_and_decode(call)?;
            let pool_created =
                abi::erc_linear_pool_factory::events::PoolCreated::match_and_decode(log)?;
            let pool_registered = get_pool_registered(tx, &pool_created.pool);
            let tokens_registered = get_token_registered(tx, &pool_registered.pool_id);

            Some(
                ProtocolComponent::new(
                    &format!("0x{}", hex::encode(pool_registered.pool_id))
                )
                .with_contracts(&[pool_created.pool.clone(), VAULT_ADDRESS.to_vec()])
                .with_tokens(&tokens_registered.tokens)
                .with_attributes(&[
                    ("pool_type", "ERC4626LinearPoolFactory".as_bytes()),
                    (
                        "upper_target",
                        &create_call
                            .upper_target
                            .to_signed_bytes_be(),
                    ),
                    ("manual_updates", &[1u8]),
                    ("bpt", &pool_created.pool),
                    ("main_token", &create_call.main_token),
                    ("wrapped_token", &create_call.wrapped_token),
                    (
                        "fee",
                        &create_call
                            .swap_fee_percentage
                            .to_signed_bytes_be(),
                    ),
                ])
                .as_swap_type("balancer_v2_pool", ImplementationType::Vm),
            )
        }
        // ❌ Reading the deployed factory for Gearbox showcases that it's currently disabled
        // hex!("39A79EB449Fc05C92c39aA6f0e9BfaC03BE8dE5B") => {
        //     let create_call =
        //         abi::gearbox_linear_pool_factory::functions::Create::match_and_decode(call)?;
        //     let pool_created =
        //         abi::gearbox_linear_pool_factory::events::PoolCreated::match_and_decode(log)?;

        //     Some(tycho::ProtocolComponent {
        //         id: hex::encode(&pool_created.pool),
        //         tokens: vec![create_call.main_token, create_call.wrapped_token],
        //         contracts: vec![pool_addr.into(), pool_created.pool],
        //         static_att: vec![
        //             tycho::Attribute {
        //                 name: "pool_type".into(),
        //                 value: "GearboxLinearPoolFactory".into(),
        //                 change: tycho::ChangeType::Creation.into(),
        //             },
        //             tycho::Attribute {
        //                 name: "upper_target".into(),
        //                 value: create_call.upper_target.to_signed_bytes_be(),
        //                 change: tycho::ChangeType::Creation.into(),
        //             },
        //         ],
        //         change: tycho::ChangeType::Creation.into(),
        //     })
        // }
        // ❌ The `ManagedPoolFactory` is a bit ✨ unique ✨, so we'll leave it commented out for
        // now Take a look at it's `Create` call to see how the params are structured.
        // hex!("BF904F9F340745B4f0c4702c7B6Ab1e808eA6b93") => {
        //     let create_call =
        // abi::managed_pool_factory::functions::Create::match_and_decode(call)?;
        //     let pool_created =
        //         abi::managed_pool_factory::events::PoolCreated::match_and_decode(log)?;

        //     Some(tycho::ProtocolComponent {
        //         id: hex::encode(&pool_created.pool),
        //         tokens: create_call.tokens,
        //         contracts: vec![pool_addr.into(), pool_created.pool],
        //         static_att: vec![
        //             tycho::Attribute {
        //                 name: "pool_type".into(),
        //                 value: "ManagedPoolFactory".into(),
        //                 change: tycho::ChangeType::Creation.into(),
        //             },
        //         ],
        //         change: tycho::ChangeType::Creation.into(),
        //     })
        // }
        hex!("19DFEF0a828EEC0c85FbB335aa65437417390b85") => {
            let create_call =
                abi::yearn_linear_pool_factory::functions::Create::match_and_decode(call)?;
            let pool_created =
                abi::yearn_linear_pool_factory::events::PoolCreated::match_and_decode(log)?;
            let pool_registered = get_pool_registered(tx, &pool_created.pool);
            let tokens_registered = get_token_registered(tx, &pool_registered.pool_id);

            Some(
                ProtocolComponent::new(
                    &format!("0x{}", hex::encode(pool_registered.pool_id))
                )
                .with_contracts(&[pool_created.pool.clone(), VAULT_ADDRESS.to_vec()])
                .with_tokens(&tokens_registered.tokens)
                .with_attributes(&[
                    ("pool_type", "YearnLinearPoolFactory".as_bytes()),
                    (
                        "upper_target",
                        &create_call
                            .upper_target
                            .to_signed_bytes_be(),
                    ),
                    ("manual_updates", &[1u8]),
                    ("bpt", &pool_created.pool),
                    ("main_token", &create_call.main_token),
                    ("wrapped_token", &create_call.wrapped_token),
                    (
                        "fee",
                        &create_call
                            .swap_fee_percentage
                            .to_signed_bytes_be(),
                    ),
                ])
                .as_swap_type("balancer_v2_pool", ImplementationType::Vm),
            )
        }
        // The `WeightedPool2TokenFactory` is a deprecated contract, but we've included
        // it to be able to track one of the highest TVL pools: 80BAL-20WETH.
        hex!("CF0a32Bbef8F064969F21f7e02328FB577382018") => {
            let create_call =
                abi::weighted_pool_tokens_factory::functions::Create::match_and_decode(call)?;
            let pool_created =
                abi::weighted_pool_tokens_factory::events::PoolCreated::match_and_decode(log)?;
            let pool_registered = get_pool_registered(tx, &pool_created.pool);

            Some(
                ProtocolComponent::new(
                    &format!("0x{}", hex::encode(pool_registered.pool_id))
                )
                .with_contracts(&[pool_created.pool, VAULT_ADDRESS.to_vec()])
                .with_tokens(&create_call.tokens)
                .with_attributes(&[
                    ("pool_type", "WeightedPool2TokensFactory".as_bytes()),
                    ("weights", &json_serialize_bigint_list(&create_call.weights)),
                    (
                        "fee",
                        &create_call
                            .swap_fee_percentage
                            .to_signed_bytes_be(),
                    ),
                    ("manual_updates", &[1u8]),
                ])
                .as_swap_type("balancer_v2_pool", ImplementationType::Vm),
            )
        }
        _ => None,
    }
}
