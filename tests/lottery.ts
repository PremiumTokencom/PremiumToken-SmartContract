import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { PublicKey, SystemProgram, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, MintLayout, AccountLayout } from '@solana/spl-token';
import { Lottery } from '../target/types/lottery';
import assert from 'assert';

describe('Lottery program tests', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Lottery as Program<Lottery>;

  const master = Keypair.generate();
  const authority = provider.wallet.publicKey;
  const lottery = Keypair.generate();
  const ticket = Keypair.generate();
  const buyer = Keypair.generate();
  let tokenMint: PublicKey;
  let payerTokenAccount: PublicKey;

  //   before(async () => {
  //     // Create a new token mint
  //     tokenMint = await Token.createMint(
  //       provider.connection,
  //       buyer, // Assuming the buyer is the mint authority
  //       buyer.publicKey,
  //       null,
  //       9, // Assuming 9 decimal places for the token
  //       TOKEN_PROGRAM_ID
  //     );

  //     // Create a token account for the payer
  //     payerTokenAccount = await tokenMint.createAccount(buyer.publicKey);
  //   });

  it('Initializes the master account', async () => {
    await program.rpc.initMaster({
      accounts: {
        master: master.publicKey,
        payer: authority,
        systemProgram: SystemProgram.programId,
      },
      signers: [master],
    });

    const masterAccount = await program.account.master.fetch(master.publicKey);
    assert.equal(masterAccount.lastId, 0);
  });

  it('Creates a lottery', async () => {
    const ticketPrice = 1 * LAMPORTS_PER_SOL; // 1 SOL

    await program.rpc.createLottery(new anchor.BN(ticketPrice), {
      accounts: {
        lottery: lottery.publicKey,
        master: master.publicKey,
        authority: authority,
        systemProgram: SystemProgram.programId,
      },
      signers: [lottery],
    });

    const lotteryAccount = await program.account.lottery.fetch(lottery.publicKey);
    assert.equal(lotteryAccount.authority.toString(), authority.toString());
    assert.equal(lotteryAccount.ticketPrice.toNumber(), ticketPrice);
  });

  // Other tests...

  //   it('Accumulates the balance to tickets', async () => {
  //     // Assuming you have the public key of the token mint and the buyer's token account
  //     const tokenMintPublicKey = new PublicKey('6fh4SJSGiiGVt3gMvAm7Ur8SjtYaiJz7oAnVbt43J8bS');
  //     const buyerTokenAccountPublicKey = new PublicKey(
  //       'CWikpgc1EpczJqF3NQNVR6JF7V7ucG5JZh3LzfU4BLyB'
  //     );

  //     // Fetch the token account to get the token balance
  //     const tokenAccountInfo = await provider.connection.getAccountInfo(buyerTokenAccountPublicKey);
  //     const tokenAmount = Token.u64FromBuffer(tokenAccountInfo.data.slice(AccountLayout.span - 8));

  //     await program.rpc.accBalance(new anchor.BN(lottery.publicKey.toBytes()), {
  //       accounts: {
  //         lottery: lottery.publicKey,
  //         balToTickets: ticket.publicKey,
  //         tokenMint: tokenMintPublicKey,
  //         payerTokenAccount: buyerTokenAccountPublicKey,
  //         payer: buyer.publicKey,
  //         systemProgram: SystemProgram.programId,
  //       },
  //       signers: [ticket],
  //     });

  //     const balToTicketsAccount = await program.account.balToTickets.fetch(ticket.publicKey);
  //     assert.equal(balToTicketsAccount.tickets.toNumber(), tokenAmount.toNumber());
  //     assert.equal(balToTicketsAccount.lotteryId.toNumber(), lottery.publicKey.toBytes());
  //     assert.equal(balToTicketsAccount.authority.toString(), buyer.publicKey.toString());
  //   });
});
