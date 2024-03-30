import * as anchor from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import {
  Connection,
  Keypair,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  ExtensionType,
  TOKEN_2022_PROGRAM_ID,
  closeAccount,
  createInitializeMintCloseAuthorityInstruction,
  createInitializeMintInstruction,
  getMintLen,
} from "@solana/spl-token";
import type { Constants } from "../target/types/constants";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Constants as anchor.Program<Constants>;


// Playground wallet
const payer = program.provider.wallet.payer;

// Connection to devnet cluster
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

// Transaction signature returned from sent transaction
let transactionSignature: string;

// Generate new keypair for Mint Account
const mintKeypair = Keypair.generate();
// Address for Mint Account
const mint = mintKeypair.publicKey;
// Decimals for Mint Account
const decimals = 2;
// Authority that can mint new tokens
const mintAuthority = program.provider.publicKey;
// Authority that can close the Mint Account
const closeAuthority = program.provider.publicKey;

// Size of Mint Account with extension
const mintLen = getMintLen([ExtensionType.MintCloseAuthority]);
// Minimum lamports required for Mint Account
const lamports = await connection.getMinimumBalanceForRentExemption(mintLen);

// Instruction to invoke System Program to create new account
const createAccountInstruction = SystemProgram.createAccount({
  fromPubkey: payer.publicKey, // Account that will transfer lamports to created account
  newAccountPubkey: mint, // Address of the account to create
  space: mintLen, // Amount of bytes to allocate to the created account
  lamports, // Amount of lamports transferred to created account
  programId: TOKEN_2022_PROGRAM_ID, // Program assigned as owner of created account
});

// Instruction to initialize the MintCloseAuthority Extension
const initializeMintCloseAuthorityInstruction =
  createInitializeMintCloseAuthorityInstruction(
    mint, // Mint Account address
    closeAuthority, // Designated Close Authority
    TOKEN_2022_PROGRAM_ID // Token Extension Program ID
  );

// Instruction to initialize Mint Account data
const initializeMintInstruction = createInitializeMintInstruction(
  mint, // Mint Account Address
  decimals, // Decimals of Mint
  mintAuthority, // Designated Mint Authority
  null, // Optional Freeze Authority
  TOKEN_2022_PROGRAM_ID // Token Extension Program ID
);

// Add instructions to new transaction
const transaction = new Transaction().add(
  createAccountInstruction,
  initializeMintCloseAuthorityInstruction,
  initializeMintInstruction
);

// Send transaction
transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [payer, mintKeypair] // Signers
);

console.log(
  "\nCreate Mint Account:",
  `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
);

// Send transaction to close Mint Account
transactionSignature = await closeAccount(
  connection,
  payer, // Transaction fee payer
  mint, // Mint Account address
  payer.publicKey, // Account to receive lamports from closed account
  closeAuthority, // Close Authority for Mint Account
  undefined, // Additional signers
  undefined, // Confirmation options
  TOKEN_2022_PROGRAM_ID // Token Extension Program ID
);

console.log(
  "\nClose Mint Account:",
  `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`
);
