pub mod bpf_writer;
pub mod errors;
pub mod metadata;
pub mod traits;

pub use {bpf_writer::*, errors::*, metadata::*, traits::*};

use {
    anchor_lang::{prelude::*, solana_program},
    errors::UtilErrors,
    mpl_token_metadata::{self, state::Metadata},
};

pub fn reallocate<'info>(
    len: isize,
    account_info: AccountInfo<'info>,
    payer_info: AccountInfo<'info>,
    rent_sysvar: &Sysvar<'info, Rent>,
    system_program: &Program<'info, System>,
) -> Result<()> {
    let curr_len = isize::try_from(account_info.data_len()).unwrap();
    let new_len = curr_len + len;
    let curr_rent = rent_sysvar.minimum_balance(usize::try_from(curr_len).unwrap());
    let new_rent = rent_sysvar.minimum_balance(usize::try_from(new_len).unwrap());
    let rent_diff: isize = isize::try_from(new_rent).unwrap() - isize::try_from(curr_rent).unwrap();

    let account_info_borrow = account_info.clone();
    if rent_diff > 0 {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
                payer_info.key,
                account_info_borrow.key,
                u64::try_from(rent_diff).unwrap(),
            ),
            &[
                payer_info,
                account_info_borrow,
                system_program.to_account_info(),
            ],
        )?;
    } else if rent_diff < 0 {
        let parsed_rent_diff = u64::try_from(rent_diff * -1).unwrap();

        **payer_info.lamports.borrow_mut() = payer_info
            .lamports()
            .checked_add(parsed_rent_diff)
            .ok_or(UtilErrors::Overflow)?;

        **account_info.lamports.borrow_mut() = account_info
            .lamports()
            .checked_sub(parsed_rent_diff)
            .ok_or(UtilErrors::Overflow)?;
    } else {
        return Ok(());
    }

    account_info
        .realloc(usize::try_from(new_len).unwrap(), false)
        .map_err(Into::into)
}

pub enum ValidateCollectionCreatorOutput {
    Collection { address: Pubkey },
    Creator { address: Pubkey },
}
pub fn validate_collection_creator<'info>(
    metadata: Metadata,
    collections: &Vec<Pubkey>,
    creators: &Vec<Pubkey>,
) -> Result<ValidateCollectionCreatorOutput> {
    if collections.len() > 0 {
        if let Some(collection) = metadata.collection {
            if collection.verified && collections.contains(&collection.key) {
                return Ok(ValidateCollectionCreatorOutput::Collection {
                    address: collection.key,
                });
            }
        }
    }

    if creators.len() > 0 {
        if let Some(metadata_creators) = metadata.data.creators {
            let found = metadata_creators
                .iter()
                .position(|x| x.verified && creators.contains(&x.address));
            if let Some(index) = found {
                return Ok(ValidateCollectionCreatorOutput::Creator {
                    address: metadata_creators[index].address,
                });
            }
        }
    }

    Err(UtilErrors::InvalidNFT.into())
}
