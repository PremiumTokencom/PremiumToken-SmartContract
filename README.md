anchor deploy --provider.cluster https://solana-devnet.g.alchemy.com/v2/ahVe4HIn1SyckYI9y4tTvlzzqQsxKaox

anchor test --provider.cluster https://solana-devnet.g.alchemy.com/v2/ahVe4HIn1SyckYI9y4tTvlzzqQsxKaox

# If error while deploying...

cargo clean

export ANCHOR_PROVIDER_URL=https://solana-devnet.g.alchemy.com/v2/ahVe4HIn1SyckYI9y4tTvlzzqQsxKaox

# To fetch token account information

spl-token account-info --address HxFzk8oXdGAXWj5B86RppMpr1FWP78Lz6jgfzzhdnwR2

# To Generate a New Program ID:

solana-keygen new --outfile target/deploy/constants-keypair.json
Key phrase eg:
witch collapse practice feed shame open despair creek road again ice least

Extract the Public Key:
solana-keygen pubkey target/deploy/constants-keypair.json
Replace the existing declare_id! line in your program with the new program ID.

declare_id!("YourNewProgramIdHere");


# AFTER DEPLOYING NEW CONTRACT

-Update program Id
-Create master