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

# To Mint more tokens:

spl-token mint <token_mint_address> <amount> <recipient_address>

spl-token mint 6fh4SJSGiiGVt3gMvAm7Ur8SjtYaiJz7oAnVbt43J8bS 300000000 -- 7pn1pj19MT1sgp3TaSu3YPj64YzZSGtisgBQCBKEcCRz
No recipient is required if minting to owners address.

Transfer Tokens

spl-token transfer --fund-recipient --allow-unfunded-recipient <TOKEN_ADDRESS> <AMOUNT> <RECIPIENT_ADDRESS> --from <SENDER_WALLET_ADDRESS> --fee-payer <FEE_PAYER_WALLET_ADDRESS>

spl-token transfer --fund-recipient --allow-unfunded-recipient 6fh4SJSGiiGVt3gMvAm7Ur8SjtYaiJz7oAnVbt43J8bS 500 9dzzxenev2kfjf1fmofmne3bg2wjhpmf2jbiiunjko9d --from 9Bfzsf4Y2YJbKMcquYQ8EqR6woyAHUmbb1TzosYeKCaL --fee-payer 9Bfzsf4Y2YJbKMcquYQ8EqR6woyAHUmbb1TzosYeKCaL

spl-token transfer 6fh4SJSGiiGVt3gMvAm7Ur8SjtYaiJz7oAnVbt43J8bS 10000 9dzzxenev2kfjf1fmofmne3bg2wjhpmf2jbiiunjko9d --allow-unfunded-recipient --from 7pn1pj19MT1sgp3TaSu3YPj64YzZSGtisgBQCBKEcCRz

# AFTER DEPLOYING NEW LOTTERY CONTRACT

-Update program Id
-Create master
