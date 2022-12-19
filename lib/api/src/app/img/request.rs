use std::{
    convert::TryInto,
    fs::{create_dir, File},
    io::Write,
    path::{Path, PathBuf},
};

use actix_multipart::{Field, Multipart};
use actix_web::web;
use config::Config;
use error::api_error::{Origin, RestError};
use futures::{StreamExt, TryStreamExt};

use super::MAX_UPLOAD_SIZE;

/// Reads, validates and stores a multipart for img_scan endpoint requests
pub(crate) async fn read_payload(
    config: &Config,
    mut payload: Multipart,
) -> Result<PathBuf, RestError> {
    // Generate file
    let rand_file = gen_local_file(config).await?;

    // Get first payload
    let field = payload
        .try_next()
        .await
        .ok()
        .flatten()
        .ok_or(RestError::Missing(Origin::File))?;

    // Read payload into file
    read_field(field, &rand_file).await?;

    Ok(rand_file)
}

async fn gen_local_file(config: &Config) -> Result<PathBuf, RestError> {
    let path = config.get_img_scan_upload_path();
    let rand_file = Path::new(&path);

    if !rand_file.exists() {
        let path = config.get_img_scan_upload_path();
        web::block(move || create_dir(Path::new(&path))).await??;
    }

    Ok(rand_file.join(format!("{}_img_scan", utils::rand_alpha_numeric(75))))
}

async fn read_field(mut field: Field, local_file: &PathBuf) -> Result<(), RestError> {
    let local_file_cloned = local_file.clone();
    let mut local_file = web::block(move || File::create(&local_file_cloned)).await??;

    // Whether the magic number has been verified or not
    let mut verified = false;

    // The current amount of uploaded bytes
    let mut size = 0;

    while let Some(chunk) = field
        .next()
        .await
        .map(|i| i.map_err(|_| RestError::IoError))
    {
        let chunk = chunk?;

        size += chunk.len();

        if size > MAX_UPLOAD_SIZE {
            return Err(RestError::BadRequest.into());
        }

        if !verified {
            check_magic_bytes(&chunk)?;
            verified = true;
        }

        local_file.write_all(&chunk)?;
    }

    Ok(())
}

/// Verifies the input files magic number
fn check_magic_bytes(chunk: &[u8]) -> Result<(), RestError> {
    let magic_bytes: [u8; 4] = chunk[0..4].try_into().map_err(|_| RestError::BadRequest)?;
    if !is_supported_format(magic_bytes) {
        return Err(RestError::FormatNotSupported.into());
    }
    Ok(())
}

/// Returns `true` if given magic_nr bytes represent a supported image format
#[inline]
fn is_supported_format(magic_nr: [u8; 4]) -> bool {
    match magic_nr {
        // JPG
        [255, 216, 255, 224] => true,
        // PNG
        [137, 80, 78, 71] => true,
        _ => false,
    }
}
