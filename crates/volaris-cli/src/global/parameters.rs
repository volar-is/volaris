use crate::global::states::{EraseMode, EraseSourceDir, ForceMode, HashMode, HeaderLocation};
use crate::global::structs::CryptoParams;
use crate::global::structs::PackParams;
use crate::warn;
use anyhow::{Context, Result};
use clap::ArgMatches;
use corecrypto::header::{HashingAlgorithm, ARGON2ID_LATEST, BLAKE3BALLOON_LATEST};
use corecrypto::primitives::Algorithm;

use super::states::{Compression, DirectoryMode, Key, KeyParams, PrintMode};
use super::structs::KeyManipulationParams;

pub fn get_params(name: &str, sub_matches: &ArgMatches) -> Result<Vec<String>> {
    let values = sub_matches
        .get_many::<String>(name)
        .with_context(|| format!("No {name} provided"))?
        .map(String::from)
        .collect();
    Ok(values)
}

pub fn get_param(name: &str, sub_matches: &ArgMatches) -> Result<String> {
    let value = sub_matches
        .get_one::<String>(name)
        .with_context(|| format!("No {} provided", name))?
        .to_string();
    Ok(value)
}

pub fn parameter_handler(sub_matches: &ArgMatches) -> Result<CryptoParams> {
    let key = Key::init(sub_matches, &KeyParams::default(), "keyfile")?;

    let hash_mode = if sub_matches.contains_id("hash") {
        HashMode::CalculateHash
    } else {
        HashMode::NoHash
    };

    let force = forcemode(sub_matches);

    let erase = if sub_matches.contains_id("erase") {
        let result = sub_matches
            .get_one::<String>("erase")
            .context("No amount of passes specified")?
            .parse();

        if let Ok(value) = result {
            EraseMode::EraseFile(value)
        } else {
            warn!("No amount of passes provided - using the default.");
            EraseMode::EraseFile(1)
        }
    } else {
        EraseMode::IgnoreFile
    };

    let header_location = if let Some(header_value) = sub_matches.get_one::<String>("header") {
        HeaderLocation::Detached(header_value.to_string())
    } else {
        HeaderLocation::Embedded
    };

    let hashing_algorithm = hashing_algorithm(sub_matches);

    Ok(CryptoParams {
        hash_mode,
        force,
        erase,
        key,
        header_location,
        hashing_algorithm,
    })
}

pub fn hashing_algorithm(sub_matches: &ArgMatches) -> HashingAlgorithm {
    if sub_matches.contains_id("argon") {
        HashingAlgorithm::Argon2id(ARGON2ID_LATEST)
    } else {
        HashingAlgorithm::Blake3Balloon(BLAKE3BALLOON_LATEST)
    }
}

pub fn algorithm(sub_matches: &ArgMatches) -> Algorithm {
    if sub_matches.contains_id("aes") {
        Algorithm::Aes256Gcm
    } else {
        Algorithm::XChaCha20Poly1305
    }
}

pub fn erase_params(sub_matches: &ArgMatches) -> Result<(i32, ForceMode)> {
    let passes = if sub_matches.contains_id("passes") {
        let result = sub_matches
            .get_one::<String>("passes")
            .context("No amount of passes specified")?
            .parse::<i32>();
        if let Ok(value) = result {
            value
        } else {
            warn!("Unable to read number of passes provided - using the default.");
            1
        }
    } else {
        warn!("Number of passes not provided - using the default.");
        1
    };

    let force = forcemode(sub_matches);

    Ok((passes, force))
}

pub fn pack_params(sub_matches: &ArgMatches) -> Result<(CryptoParams, PackParams)> {
    let key = Key::init(sub_matches, &KeyParams::default(), "keyfile")?;

    let hash_mode = if sub_matches.contains_id("hash") {
        HashMode::CalculateHash
    } else {
        HashMode::NoHash
    };

    let force = forcemode(sub_matches);

    let erase = EraseMode::IgnoreFile;

    let header_location = if let Some(header_value) = sub_matches.get_one::<String>("header") {
        HeaderLocation::Detached(header_value.to_string())
    } else {
        HeaderLocation::Embedded
    };

    let hashing_algorithm = hashing_algorithm(sub_matches);

    let crypto_params = CryptoParams {
        hash_mode,
        force,
        erase,
        key,
        header_location,
        hashing_algorithm,
    };

    let print_mode = if sub_matches.contains_id("verbose") {
        PrintMode::Verbose
    } else {
        PrintMode::Quiet
    };

    let dir_mode = if sub_matches.contains_id("recursive") {
        DirectoryMode::Recursive
    } else {
        DirectoryMode::Singular
    };

    let erase_source = if sub_matches.contains_id("erase") {
        EraseSourceDir::Erase
    } else {
        EraseSourceDir::Retain
    };

    let compression = if sub_matches.contains_id("zstd") {
        Compression::Zstd
    } else {
        Compression::None
    };

    let pack_params = PackParams {
        dir_mode,
        print_mode,
        erase_source,
        compression,
    };

    Ok((crypto_params, pack_params))
}

pub fn forcemode(sub_matches: &ArgMatches) -> ForceMode {
    if sub_matches.contains_id("force") {
        ForceMode::Force
    } else {
        ForceMode::Prompt
    }
}

pub fn key_manipulation_params(sub_matches: &ArgMatches) -> Result<KeyManipulationParams> {
    let key_old = Key::init(
        sub_matches,
        &KeyParams {
            user: true,
            env: false,
            autogenerate: false,
            keyfile: true,
        },
        "keyfile-old",
    )?;

    let key_new = Key::init(
        sub_matches,
        &KeyParams {
            user: true,
            env: false,
            autogenerate: true,
            keyfile: true,
        },
        "keyfile-new",
    )?;

    let hashing_algorithm = hashing_algorithm(sub_matches);

    Ok(KeyManipulationParams {
        key_old,
        key_new,
        hashing_algorithm,
    })
}
