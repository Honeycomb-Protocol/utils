use crate::errors::UtilErrors;

use {
    anchor_lang::{prelude::*, solana_program},
    mpl_token_metadata::{
        self,
        instruction::{builders::ApproveCollectionAuthorityBuilder, InstructionBuilder},
        pda::find_collection_authority_account,
    },
};

pub fn approve<'info>(
    _collection_mint: AccountInfo<'info>,
    _collection_metadata: AccountInfo<'info>,
    _update_authority: AccountInfo<'info>,
    _new_authority: AccountInfo<'info>,
    _new_authority_record: AccountInfo<'info>,
    _system_program: AccountInfo<'info>,
    _rent_program: AccountInfo<'info>,
    _payer: AccountInfo<'info>,
    _signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    return Ok(());
    // let (collection_authority_record, _) =
    //     find_collection_authority_account(&collection_mint.key(), &update_authority.key());
    // if new_authority_record.key() != collection_authority_record {
    //     return Err(UtilErrors::InvalidNewAuthorityRecord.into());
    // }
    // let mut binding = ApproveCollectionAuthorityBuilder::new();
    // let verify_builder = binding
    //     .collection_authority_record(collection_authority_record)
    //     .new_collection_authority(new_authority.key())
    //     .update_authority(update_authority.key())
    //     .payer(payer.key())
    //     .metadata(collection_metadata.key())
    //     .mint(collection_mint.key())
    //     .system_program(system_program.key())
    //     .rent(rent_program.key());

    // let verify_ix = verify_builder.build().unwrap().instruction();

    // let mut account_infos = vec![
    //     new_authority_record,
    //     new_authority,
    //     update_authority,
    //     payer,
    //     collection_metadata,
    //     collection_mint,
    //     system_program,
    //     rent_program,
    // ];

    // if let Some(signer_seeds) = signer_seeds {
    //     return solana_program::program::invoke_signed(
    //         &verify_ix,
    //         &account_infos[..],
    //         signer_seeds,
    //     )
    //     .map_err(Into::into);
    // } else {
    //     return solana_program::program::invoke(&verify_ix, &account_infos[..]).map_err(Into::into);
    // }
}
