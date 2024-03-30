import * as anchor from '@project-serum/anchor';
import { SystemProgram, PublicKey } from '@solana/web3.js';

import * as fs from 'fs';

(async () => {
  // Set up connection and wallet
  const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl('devnet'));
  const wallet = anchor.Wallet.local(); // Assumes you have a local wallet set up

  // Set up provider
  const provider = new anchor.AnchorProvider(connection, wallet, { commitment: 'confirmed' });
  anchor.setProvider(provider);

  // Load your program IDL
  const idl = JSON.parse(fs.readFileSync('../target/idl/lottery.json', 'utf8'));

  // Program ID and program
  const programId = new PublicKey('9CtBiDQzHos17YZ6YL9iVLvwVifqXYoy48t6pKWUjWSy');
  const program = new anchor.Program(idl, programId, provider);

  // Derive the master account's public key
  const [masterPublicKey, masterBump] = await PublicKey.findProgramAddress(
    [Buffer.from('master')], // The seed for the master account
    program.programId
  );

  console.log('masterPublicKey: ', masterPublicKey);

  // Create the master account
  await program.rpc.initMaster({
    accounts: {
      master: masterPublicKey,
      payer: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    options: { commitment: 'confirmed' },
  });
  console.log('Master account created:', masterPublicKey.toBase58());
})();
