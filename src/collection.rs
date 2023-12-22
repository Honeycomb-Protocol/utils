use anchor_lang::prelude::*;

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
}
