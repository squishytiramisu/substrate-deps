use crate::error::*;
use crate::metadata::Manifest;

use cargo_deps::{get_dep_graph, render_dep_graph, Config};
use clap::ArgMatches;
use std::{
    fs,
    io::{self, Write},
};

lazy_static! {
    static ref FRAME: [String; 50] = [
        "pallet-alliance".to_owned(),
        "pallet-assets".to_owned(),
        "pallet-atomic-swap".to_owned(),
        "pallet-aura".to_owned(),
        "pallet-authority-discovery".to_owned(),
        "pallet-authorship".to_owned(),
        "pallet-babe".to_owned(),
        "pallet-balances".to_owned(),
        "pallet-bounties".to_owned(),
        "pallet-collective".to_owned(),
        "pallet-contracts".to_owned(),
        "pallet-contracts-primitives".to_owned(),
        "pallet-contracts-rpc".to_owned(),
        "pallet-contracts-rpc-runtime-api".to_owned(),
        "pallet-democracy".to_owned(),
        "pallet-election-provider-multi-phase".to_owned(),
        "pallet-elections-phragmen".to_owned(),
        "pallet-example-basic".to_owned(),
        "pallet-example-offchain-worker".to_owned(),
        "pallet-example-parallel".to_owned(),
        "pallet-grandpa".to_owned(),
        "pallet-identity".to_owned(),
        "pallet-im-online".to_owned(),
        "pallet-indices".to_owned(),
        "pallet-lottery".to_owned(),
        "pallet-membership".to_owned(),
        "pallet-multisig".to_owned(),
        "pallet-nicks".to_owned(),
        "pallet-offences".to_owned(),
        "pallet-proxy".to_owned(),
        "pallet-randomness-collective-flip".to_owned(),
        "pallet-recovery".to_owned(),
        "pallet-scheduler".to_owned(),
        "pallet-scored-pool".to_owned(),
        "pallet-session".to_owned(),
        "pallet-society".to_owned(),
        "pallet-staking".to_owned(),
        "pallet-sudo".to_owned(),
        "pallet-timestamp".to_owned(),
        "pallet-transaction-payment".to_owned(),
        "pallet-treasury".to_owned(),
        "pallet-uniques".to_owned(),
        "pallet-utility".to_owned(),
        "pallet-vesting".to_owned(),
        "cumulus-pallet-aura-ext".to_owned(),
        "cumulus-pallet-parachain-system".to_owned(),
        "cumulus-pallet-xcmp-queue".to_owned(),
        "pallet-collator-selection".to_owned(),
        "cumulus-pallet-dmp-queue".to_owned(),
        "cumulus-pallet-solo-to-para".to_owned(),
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
