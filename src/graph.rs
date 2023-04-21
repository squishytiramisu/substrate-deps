use crate::error::*;
use crate::metadata::Manifest;

use cargo_deps::{get_dep_graph, render_dep_graph, Config};
use clap::ArgMatches;
use std::{
    fs,
    io::{self, Write},
};

lazy_static! {
    static ref FRAME: [String; 35] = [
        "pallet_alliance".to_owned(),
        "pallet_assets".to_owned(),
        "pallet_atomic_swap".to_owned(),
        "pallet_aura".to_owned(),
        "pallet_authority_discovery".to_owned(),
        "pallet_authorship".to_owned(),
        "pallet_babe".to_owned(),
        "pallet_balances".to_owned(),
        "pallet_bounties".to_owned(),
        "pallet_collective".to_owned(),
        "pallet_contracts".to_owned(),
        "pallet_contracts_primitives".to_owned(),
        "pallet_contracts_rpc".to_owned(),
        "pallet_contracts_rpc_runtime_api".to_owned(),
        "pallet_democracy".to_owned(),
        "pallet_election_provider_multi_phase".to_owned(),
        "pallet_elections_phragmen".to_owned(),
        "pallet_example_basic".to_owned(),
        "pallet_example_offchain_worker".to_owned(),
        "pallet_example_parallel".to_owned(),
        "pallet_grandpa".to_owned(),
        "pallet_identity".to_owned(),
        "pallet_im_online".to_owned(),
        "pallet_indices".to_owned(),
        "pallet_lottery".to_owned(),
        "pallet_membership".to_owned(),
        "pallet_multisig".to_owned(),
        "pallet_nicks".to_owned(),
        "pallet_offences".to_owned(),
        "pallet_proxy".to_owned(),
        "pallet_randomness_collective_flip".to_owned(),
        "pallet_recovery".to_owned(),
        "pallet_scheduler".to_owned(),
        "pallet_scored_pool".to_owned(),
        "pallet_session".to_owned(),
        "pallet_society".to_owned(),
        "pallet_staking".to_owned(),
        "pallet_sudo".to_owned(),
        "pallet_timestamp".to_owned(),
        "pallet_transaction_payment".to_owned(),
        "pallet_treasury".to_owned(),
        "pallet_uniques".to_owned(),
        "pallet_utility".to_owned(),
        "pallet_vesting".to_owned(),
    ];
}




pub fn execute_graph(m: &ArgMatches) -> CliResult<()> {
    // debug!("Manifest path: {:?}", manifest_path);

    let mut cfg = Config::default();
    cfg.manifest_path = m.value_of("manifest-path").unwrap_or("Cargo.toml").into();
    cfg.include_versions = m.is_present("include-versions");
    let manifest = read_manifest(&cfg.manifest_path)?;

    let mut filter = vec![manifest.package().as_ref().unwrap().name().to_owned()];
    filter.append(&mut FRAME.to_vec());
    cfg.filter = Some(filter);
    cfg.transitive_deps = false;

    // Get dependency graph & render it
    let o = get_dep_graph(cfg).and_then(render_dep_graph)?;
    io::stdout()
        .write_all(&o.into_bytes())
        .expect("Unable to write graph");

    Ok(())
}

fn read_manifest(manifest: &str) -> CliResult<Manifest> {
    let s = fs::read_to_string(manifest)?;
    let manifest: Manifest = toml::from_str(&s).map_err(|_| {
        CliError::Metadata(
            "Error reading pallet metadata: could parse crate manifest as TOML.".to_owned(),
        )
    })?;
    Ok(manifest)
}
