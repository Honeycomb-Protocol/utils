pub use {
    anchor_lang::{prelude::*, solana_program},
    mpl_token_metadata::{
        self,
        instruction::{
            builders::{
                BurnBuilder, CreateBuilder, DelegateBuilder, LockBuilder, MintBuilder,
                RevokeBuilder, TransferBuilder, UnlockBuilder, UpdateBuilder, VerifyBuilder,
            },
            BurnArgs, CreateArgs, DelegateArgs, InstructionBuilder, LockArgs, MintArgs, RevokeArgs,
            TransferArgs, UnlockArgs, UpdateArgs, VerificationArgs,
        },
        state::{AssetData, PrintSupply},
    },
};

pub fn create_nft<'info>(
    asset_data: AssetData,
    initialize_mint: bool,
    update_authority_as_signer: bool,
    metadata: AccountInfo<'info>,
    master_edition: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    update_authority: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    spl_token_program: AccountInfo<'info>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    create(
        CreateArgs::V1 {
            asset_data,
            decimals: Some(0),
            print_supply: Some(PrintSupply::Zero),
        },
        initialize_mint,
        update_authority_as_signer,
        metadata,
        Some(master_edition),
        mint,
        authority,
        payer,
        update_authority,
        system_program,
        sysvar_instructions,
        spl_token_program,
        signer_seeds,
    )
}

pub fn create<'info>(
    args: CreateArgs,
    initialize_mint: bool,
    update_authority_as_signer: bool,
    metadata: AccountInfo<'info>,
    master_edition: Option<AccountInfo<'info>>,
    mint: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    update_authority: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    spl_token_program: AccountInfo<'info>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = CreateBuilder::new();
    let create_builder = binding
        .metadata(metadata.key())
        .mint(mint.key())
        .authority(authority.key())
        .payer(payer.key())
        .update_authority(update_authority.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key())
        .spl_token_program(spl_token_program.key())
        .initialize_mint(initialize_mint)
        .update_authority_as_signer(update_authority_as_signer);

    let mut account_infos = vec![metadata];

    if let Some(master_edition) = master_edition {
        create_builder.master_edition(master_edition.key());
        account_infos.push(master_edition);
    }

    account_infos = [
        account_infos,
        vec![
            mint,
            authority,
            payer,
            update_authority,
            system_program,
            sysvar_instructions,
            spl_token_program,
        ],
    ]
    .concat();

    let create_ix = create_builder.build(args).unwrap().instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(
            &create_ix,
            &account_infos[..],
            signer_seeds,
        )
        .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&create_ix, &account_infos[..]).map_err(Into::into);
    }
}

pub fn mint<'info>(
    args: MintArgs,
    token: AccountInfo<'info>,
    token_owner: Option<AccountInfo<'info>>,
    metadata: AccountInfo<'info>,
    master_edition: Option<AccountInfo<'info>>,
    token_record: Option<AccountInfo<'info>>,
    mint: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    delegate_record: Option<AccountInfo<'info>>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    spl_token_program: AccountInfo<'info>,
    spl_ata_program: AccountInfo<'info>,
    authorization_rules_program: Option<AccountInfo<'info>>,
    authorization_rules: Option<AccountInfo<'info>>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = MintBuilder::new();

    let mint_builder = binding
        .token(token.key())
        .metadata(metadata.key())
        .mint(mint.key())
        .authority(authority.key())
        .payer(payer.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key())
        .spl_token_program(spl_token_program.key())
        .spl_ata_program(spl_ata_program.key());

    let mut account_infos = vec![token];

    if let Some(token_owner) = token_owner {
        mint_builder.token_owner(token_owner.key());
        account_infos.push(token_owner);
    }

    account_infos = [account_infos, vec![metadata]].concat();

    if let Some(master_edition) = master_edition {
        mint_builder.master_edition(master_edition.key());
        account_infos.push(master_edition);
    }

    if let Some(token_record) = token_record {
        mint_builder.token_record(token_record.key());
        account_infos.push(token_record);
    }

    account_infos = [account_infos, vec![mint, authority]].concat();

    if let Some(delegate_record) = delegate_record {
        mint_builder.delegate_record(delegate_record.key());
        account_infos.push(delegate_record);
    }

    account_infos = [
        account_infos,
        vec![
            payer,
            system_program,
            sysvar_instructions,
            spl_token_program,
            spl_ata_program,
        ],
    ]
    .concat();

    if let Some(authorization_rules_program) = authorization_rules_program {
        mint_builder.authorization_rules_program(authorization_rules_program.key());
        account_infos.push(authorization_rules_program);
    }

    if let Some(authorization_rules) = authorization_rules {
        mint_builder.authorization_rules(authorization_rules.key());
        account_infos.push(authorization_rules);
    }

    let mint_ix = mint_builder.build(args).unwrap().instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(&mint_ix, &account_infos[..], signer_seeds)
            .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&mint_ix, &account_infos[..]).map_err(Into::into);
    }
}

pub fn burn<'info>(
    args: BurnArgs,
    authority: AccountInfo<'info>,
    collection_metadata: Option<AccountInfo<'info>>,
    metadata: AccountInfo<'info>,
    edition: Option<AccountInfo<'info>>,
    mint: AccountInfo<'info>,
    token: AccountInfo<'info>,
    master_edition: Option<AccountInfo<'info>>,
    master_edition_mint: Option<AccountInfo<'info>>,
    master_edition_token: Option<AccountInfo<'info>>,
    edition_marker: Option<AccountInfo<'info>>,
    token_record: Option<AccountInfo<'info>>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    spl_token_program: AccountInfo<'info>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = BurnBuilder::new();

    let burn_builder = binding
        .authority(authority.key())
        .metadata(metadata.key())
        .mint(mint.key())
        .token(token.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key())
        .spl_token_program(spl_token_program.key());

    let mut account_infos = vec![authority];

    if let Some(collection_metadata) = collection_metadata {
        burn_builder.collection_metadata(collection_metadata.key());
        account_infos.push(collection_metadata);
    }

    account_infos = [account_infos, vec![metadata]].concat();

    if let Some(edition) = edition {
        burn_builder.edition(edition.key());
        account_infos.push(edition);
    }

    account_infos = [account_infos, vec![mint, token]].concat();

    if let Some(master_edition) = master_edition {
        burn_builder.master_edition(master_edition.key());
        account_infos.push(master_edition);
    }

    if let Some(master_edition_mint) = master_edition_mint {
        burn_builder.master_edition_mint(master_edition_mint.key());
        account_infos.push(master_edition_mint);
    }

    if let Some(master_edition_token) = master_edition_token {
        burn_builder.master_edition_token(master_edition_token.key());
        account_infos.push(master_edition_token);
    }

    if let Some(edition_marker) = edition_marker {
        burn_builder.edition_marker(edition_marker.key());
        account_infos.push(edition_marker);
    }

    if let Some(token_record) = token_record {
        burn_builder.token_record(token_record.key());
        account_infos.push(token_record);
    }

    account_infos = [
        account_infos,
        vec![system_program, sysvar_instructions, spl_token_program],
    ]
    .concat();

    let burn_ix = burn_builder.build(args).unwrap().instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(&burn_ix, &account_infos[..], signer_seeds)
            .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&burn_ix, &account_infos[..]).map_err(Into::into);
    }
}

pub fn update<'info>(
    args: UpdateArgs,
    delegate_record: Option<AccountInfo<'info>>,
    token: Option<AccountInfo<'info>>,
    mint: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    edition: Option<AccountInfo<'info>>,
    authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    authorization_rules_program: Option<AccountInfo<'info>>,
    authorization_rules: Option<AccountInfo<'info>>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = UpdateBuilder::new();
    let update_builder = binding
        .authority(authority.key())
        .mint(mint.key())
        .metadata(metadata.key())
        .payer(payer.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key());

    let mut account_infos = vec![authority];

    if let Some(delegate_record) = delegate_record {
        update_builder.delegate_record(delegate_record.key());
        account_infos.push(delegate_record);
    }

    if let Some(token) = token {
        update_builder.token(token.key());
        account_infos.push(token);
    }

    account_infos = [account_infos, vec![mint, metadata]].concat();

    if let Some(edition) = edition {
        update_builder.edition(edition.key());
        account_infos.push(edition);
    }

    account_infos = [
        account_infos,
        vec![payer, system_program, sysvar_instructions],
    ]
    .concat();

    if let Some(authorization_rules_program) = authorization_rules_program {
        update_builder.authorization_rules_program(authorization_rules_program.key());
        account_infos.push(authorization_rules_program);
    }

    if let Some(authorization_rules) = authorization_rules {
        update_builder.authorization_rules(authorization_rules.key());
        account_infos.push(authorization_rules);
    }

    let update_id = update_builder.build(args).unwrap().instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(
            &update_id,
            &account_infos[..],
            signer_seeds,
        )
        .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&update_id, &account_infos[..]).map_err(Into::into);
    }
}

pub fn transfer<'info>(
    amount: u64,
    source_token_account: AccountInfo<'info>,
    source_token_account_owner: AccountInfo<'info>,
    destination_token_account: AccountInfo<'info>,
    destination_token_account_owner: AccountInfo<'info>,
    token_mint: AccountInfo<'info>,
    token_metadata: AccountInfo<'info>,
    token_edition: Option<AccountInfo<'info>>,
    source_token_account_record: Option<AccountInfo<'info>>,
    destination_token_account_record: Option<AccountInfo<'info>>,
    authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    associated_token_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    authorization_rules_program: Option<AccountInfo<'info>>,
    authorization_rules: Option<AccountInfo<'info>>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = TransferBuilder::new();
    let transfer_builder = binding
        .token(source_token_account.key())
        .token_owner(source_token_account_owner.key())
        .destination(destination_token_account.key())
        .destination_owner(destination_token_account_owner.key())
        .mint(token_mint.key())
        .metadata(token_metadata.key())
        .authority(authority.key())
        .payer(payer.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key())
        .spl_token_program(token_program.key())
        .spl_ata_program(associated_token_program.key());

    let mut account_infos = vec![
        source_token_account,
        source_token_account_owner,
        destination_token_account,
        destination_token_account_owner,
        token_mint,
        token_metadata,
    ];

    if let Some(token_edition) = token_edition {
        transfer_builder.edition(token_edition.key());
        account_infos.push(token_edition);
    }

    if let Some(source_token_account_record) = source_token_account_record {
        transfer_builder.owner_token_record(source_token_account_record.key());
        account_infos.push(source_token_account_record);
    }

    if let Some(destination_token_account_record) = destination_token_account_record {
        transfer_builder.destination_token_record(destination_token_account_record.key());
        account_infos.push(destination_token_account_record);
    }

    account_infos = [
        account_infos,
        vec![
            authority,
            payer,
            system_program,
            sysvar_instructions,
            token_program,
            associated_token_program,
        ],
    ]
    .concat();

    if let Some(authorization_rules_program) = authorization_rules_program {
        transfer_builder.authorization_rules_program(authorization_rules_program.key());
        account_infos.push(authorization_rules_program);
    }

    if let Some(authorization_rules) = authorization_rules {
        transfer_builder.authorization_rules(authorization_rules.key());
        account_infos.push(authorization_rules);
    }

    let transfer_ix = transfer_builder
        .build(TransferArgs::V1 {
            amount,
            authorization_data: None,
        })
        .unwrap()
        .instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(
            &transfer_ix,
            &account_infos[..],
            signer_seeds,
        )
        .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&transfer_ix, &account_infos[..])
            .map_err(Into::into);
    }
}

pub fn lock<'info>(
    authority: AccountInfo<'info>,
    token_mint: AccountInfo<'info>,
    token_account: AccountInfo<'info>,
    token_account_owner: Option<AccountInfo<'info>>,
    token_metadata: AccountInfo<'info>,
    token_edition: Option<AccountInfo<'info>>,
    token_record: Option<AccountInfo<'info>>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    authorization_rules_program: Option<AccountInfo<'info>>,
    authorization_rules: Option<AccountInfo<'info>>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = LockBuilder::new();
    let lock_builder = binding
        .authority(authority.key())
        .token(token_account.key())
        .mint(token_mint.key())
        .metadata(token_metadata.key())
        .payer(payer.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key())
        .spl_token_program(token_program.key());

    let mut account_infos = vec![authority];

    if let Some(token_account_owner) = token_account_owner {
        lock_builder.token_owner(token_account_owner.key());
        account_infos.push(token_account_owner);
    }

    account_infos = [
        account_infos,
        vec![token_account, token_mint, token_metadata],
    ]
    .concat();

    if let Some(token_edition) = token_edition {
        lock_builder.edition(token_edition.key());
        account_infos.push(token_edition);
    }

    if let Some(token_record) = token_record {
        lock_builder.token_record(token_record.key());
        account_infos.push(token_record);
    }

    account_infos = [
        account_infos,
        vec![payer, system_program, sysvar_instructions, token_program],
    ]
    .concat();

    if let Some(authorization_rules_program) = authorization_rules_program {
        lock_builder.authorization_rules_program(authorization_rules_program.key());
        account_infos.push(authorization_rules_program);
    }

    if let Some(authorization_rules) = authorization_rules {
        lock_builder.authorization_rules(authorization_rules.key());
        account_infos.push(authorization_rules);
    }

    let lock_ix = lock_builder
        .build(LockArgs::V1 {
            authorization_data: None,
        })
        .unwrap()
        .instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(&lock_ix, &account_infos[..], signer_seeds)
            .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&lock_ix, &account_infos[..]).map_err(Into::into);
    }
}

pub fn unlock<'info>(
    authority: AccountInfo<'info>,
    token_mint: AccountInfo<'info>,
    token_account: AccountInfo<'info>,
    token_account_owner: Option<AccountInfo<'info>>,
    token_metadata: AccountInfo<'info>,
    token_edition: Option<AccountInfo<'info>>,
    token_record: Option<AccountInfo<'info>>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    authorization_rules_program: Option<AccountInfo<'info>>,
    authorization_rules: Option<AccountInfo<'info>>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = UnlockBuilder::new();
    let unlock_builder = binding
        .authority(authority.key())
        .token(token_account.key())
        .mint(token_mint.key())
        .metadata(token_metadata.key())
        .payer(payer.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key())
        .spl_token_program(token_program.key());

    let mut account_infos = vec![authority];

    if let Some(token_account_owner) = token_account_owner {
        unlock_builder.token_owner(token_account_owner.key());
        account_infos.push(token_account_owner);
    }

    account_infos = [
        account_infos,
        vec![token_account, token_mint, token_metadata],
    ]
    .concat();

    if let Some(token_edition) = token_edition {
        unlock_builder.edition(token_edition.key());
        account_infos.push(token_edition);
    }

    if let Some(token_record) = token_record {
        unlock_builder.token_record(token_record.key());
        account_infos.push(token_record);
    }

    account_infos = [
        account_infos,
        vec![payer, system_program, sysvar_instructions, token_program],
    ]
    .concat();

    if let Some(authorization_rules_program) = authorization_rules_program {
        unlock_builder.authorization_rules_program(authorization_rules_program.key());
        account_infos.push(authorization_rules_program);
    }

    if let Some(authorization_rules) = authorization_rules {
        unlock_builder.authorization_rules(authorization_rules.key());
        account_infos.push(authorization_rules);
    }

    let unlock_ix = unlock_builder
        .build(UnlockArgs::V1 {
            authorization_data: None,
        })
        .unwrap()
        .instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(
            &unlock_ix,
            &account_infos[..],
            signer_seeds,
        )
        .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&unlock_ix, &account_infos[..]).map_err(Into::into);
    }
}

pub fn delegate<'info>(
    args: DelegateArgs,
    delegate_record: Option<AccountInfo<'info>>,
    delegate: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    master_edition: Option<AccountInfo<'info>>,
    token_record: Option<AccountInfo<'info>>,
    mint: AccountInfo<'info>,
    token_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    authorization_rules_program: Option<AccountInfo<'info>>,
    authorization_rules: Option<AccountInfo<'info>>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = DelegateBuilder::new();
    let delegate_builder = binding
        .delegate(delegate.key())
        .metadata(metadata.key())
        .mint(mint.key())
        .token(token_account.key())
        .authority(authority.key())
        .payer(payer.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key())
        .spl_token_program(token_program.key());

    let mut account_infos = vec![];

    if let Some(delegate_record) = delegate_record {
        delegate_builder.delegate_record(delegate_record.key());
        account_infos.push(delegate_record);
    }

    account_infos = [account_infos, vec![delegate, metadata]].concat();

    if let Some(master_edition) = master_edition {
        delegate_builder.master_edition(master_edition.key());
        account_infos.push(master_edition);
    }

    if let Some(token_record) = token_record {
        delegate_builder.token_record(token_record.key());
        account_infos.push(token_record);
    }

    account_infos = [
        account_infos,
        vec![
            mint,
            token_account,
            authority,
            payer,
            system_program,
            sysvar_instructions,
            token_program,
        ],
    ]
    .concat();

    if let Some(authorization_rules_program) = authorization_rules_program {
        delegate_builder.authorization_rules_program(authorization_rules_program.key());
        account_infos.push(authorization_rules_program);
    }

    if let Some(authorization_rules) = authorization_rules {
        delegate_builder.authorization_rules(authorization_rules.key());
        account_infos.push(authorization_rules);
    }

    let delegate_ix = delegate_builder.build(args).unwrap().instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(
            &delegate_ix,
            &account_infos[..],
            signer_seeds,
        )
        .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&delegate_ix, &account_infos[..])
            .map_err(Into::into);
    }
}

pub fn revoke<'info>(
    args: RevokeArgs,
    delegate_record: Option<AccountInfo<'info>>,
    delegate: AccountInfo<'info>,
    metadata: AccountInfo<'info>,
    master_edition: Option<AccountInfo<'info>>,
    token_record: Option<AccountInfo<'info>>,
    mint: AccountInfo<'info>,
    token_account: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    authorization_rules_program: Option<AccountInfo<'info>>,
    authorization_rules: Option<AccountInfo<'info>>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = RevokeBuilder::new();
    let revoke_builder = binding
        .delegate(delegate.key())
        .metadata(metadata.key())
        .mint(mint.key())
        .token(token_account.key())
        .authority(authority.key())
        .payer(payer.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key())
        .spl_token_program(token_program.key());

    let mut account_infos = vec![];

    if let Some(delegate_record) = delegate_record {
        revoke_builder.delegate_record(delegate_record.key());
        account_infos.push(delegate_record);
    }

    account_infos = [account_infos, vec![delegate, metadata]].concat();

    if let Some(master_edition) = master_edition {
        revoke_builder.master_edition(master_edition.key());
        account_infos.push(master_edition);
    }

    if let Some(token_record) = token_record {
        revoke_builder.token_record(token_record.key());
        account_infos.push(token_record);
    }

    account_infos = [
        account_infos,
        vec![
            mint,
            token_account,
            authority,
            payer,
            system_program,
            sysvar_instructions,
            token_program,
        ],
    ]
    .concat();

    if let Some(authorization_rules_program) = authorization_rules_program {
        revoke_builder.authorization_rules_program(authorization_rules_program.key());
        account_infos.push(authorization_rules_program);
    }

    if let Some(authorization_rules) = authorization_rules {
        revoke_builder.authorization_rules(authorization_rules.key());
        account_infos.push(authorization_rules);
    }

    let revoke_ix = revoke_builder.build(args).unwrap().instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(
            &revoke_ix,
            &account_infos[..],
            signer_seeds,
        )
        .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&revoke_ix, &account_infos[..]).map_err(Into::into);
    }
}

pub fn verify<'info>(
    args: VerificationArgs,
    authority: AccountInfo<'info>,
    delegate_record: Option<AccountInfo<'info>>,
    metadata: AccountInfo<'info>,
    collection_mint: Option<AccountInfo<'info>>,
    collection_metadata: Option<AccountInfo<'info>>,
    collection_master_edition: Option<AccountInfo<'info>>,
    system_program: AccountInfo<'info>,
    sysvar_instructions: AccountInfo<'info>,
    signer_seeds: Option<&[&[&[u8]]; 1]>,
) -> Result<()> {
    let mut binding = VerifyBuilder::new();
    let verify_builder = binding
        .authority(authority.key())
        .metadata(metadata.key())
        .system_program(system_program.key())
        .sysvar_instructions(sysvar_instructions.key());

    let mut account_infos = vec![authority];

    if let Some(delegate_record) = delegate_record {
        verify_builder.delegate_record(delegate_record.key());
        account_infos.push(delegate_record);
    }

    account_infos = [account_infos, vec![metadata]].concat();

    if let Some(collection_mint) = collection_mint {
        verify_builder.collection_mint(collection_mint.key());
        account_infos.push(collection_mint);
    }

    if let Some(collection_metadata) = collection_metadata {
        verify_builder.collection_metadata(collection_metadata.key());
        account_infos.push(collection_metadata);
    }

    if let Some(collection_master_edition) = collection_master_edition {
        verify_builder.collection_master_edition(collection_master_edition.key());
        account_infos.push(collection_master_edition);
    }

    account_infos = [account_infos, vec![system_program, sysvar_instructions]].concat();

    let verify_ix = verify_builder.build(args).unwrap().instruction();

    if let Some(signer_seeds) = signer_seeds {
        return solana_program::program::invoke_signed(
            &verify_ix,
            &account_infos[..],
            signer_seeds,
        )
        .map_err(Into::into);
    } else {
        return solana_program::program::invoke(&verify_ix, &account_infos[..]).map_err(Into::into);
    }
}
