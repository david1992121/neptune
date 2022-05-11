import {
  Connection,
  clusterApiUrl,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  TransactionInstruction,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  Transaction,
  SYSVAR_CLOCK_PUBKEY,
} from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  burnCheckedInstructionData,
  createAssociatedTokenAccount,
  createMint, getAccount, mintTo, TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import BN from "bn.js";
import * as borsh from '@project-serum/borsh';

//POPULATE THIS WITH YOUR PROGRAM ID
let programId = new PublicKey("");


const clientScript = async () => {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  let aliceKp = new Keypair();
  let airdropSig1 = await connection.requestAirdrop(aliceKp.publicKey, 2 * LAMPORTS_PER_SOL);
  let clientAuth = new Keypair();
  let airdropSig2 = await connection.requestAirdrop(clientAuth.publicKey, 2 * LAMPORTS_PER_SOL);
  console.log(`airdrop sigs ${airdropSig1} and ${airdropSig2}`)
  
  await delay(1500);
  console.log("creating mint");
  let mintKp = new Keypair(); 
  let mintPk = await createMint(
    connection,
    aliceKp,
    clientAuth.publicKey,
    clientAuth.publicKey,
    9,
    mintKp,
    {},
    TOKEN_PROGRAM_ID,
  );

  await delay(1500);
  console.log("creating alices token account");
  let aliceTokenPk = await createAssociatedTokenAccount(
    connection,
    aliceKp,
    mintPk,
    aliceKp.publicKey,
    {},
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID, 
  );

  await delay(1500);
  console.log("minting tokens to alice");
  let alicesTokenAmount = 1000 * LAMPORTS_PER_SOL;
  let mintToSig = await mintTo(
    connection,
    aliceKp,
    mintPk,
    aliceTokenPk,
    clientAuth,
    alicesTokenAmount,
    [],
    {},
    TOKEN_PROGRAM_ID,
  );

  let [aliceLockState, bump] = await PublicKey.findProgramAddress(
    [aliceKp.publicKey.toBuffer()],
    programId,
  );

  await delay(1500);
  console.log("creating a lock state token account");
  let lockStateTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    aliceKp,
    mintPk,
    aliceLockState,
    true,
    undefined,
    undefined,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  await delay(1500);
  console.log("building tx");
  const createAccountSchema = borsh.struct([
    borsh.u8('instruction'),
    borsh.u8('bump'),
  ]);
  let createBuffer = Buffer.alloc(1000);
  createAccountSchema.encode(
    {
      instruction: 0,
      bump,
    },
    createBuffer,
  );
  createBuffer = createBuffer.slice(0, createAccountSchema.getSpan(createBuffer))
  let createAccountIx = new TransactionInstruction({
    programId,
    keys: [
      {pubkey: aliceKp.publicKey, isSigner: true, isWritable: false},
      {pubkey: aliceLockState, isSigner: false, isWritable: true},
      {pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false},
      {pubkey: SystemProgram.programId, isSigner:false, isWritable: false},
      {pubkey: lockStateTokenAccount.address, isSigner: false, isWritable: false }
    ],
    data: createBuffer
  });
  
  const lockTokenSchema = borsh.struct([
    borsh.u8('instruction'),
    borsh.u64('amount'),
  ]);
  let lockBuffer = Buffer.alloc(1000);
  lockTokenSchema.encode(
    {
      instruction: 1,
      amount: new BN(alicesTokenAmount),
    },
    lockBuffer,
  );
  lockBuffer = lockBuffer.slice(0, lockTokenSchema.getSpan(lockBuffer))

  let lockTokenIx = new TransactionInstruction({
    programId,
    keys: [
      {pubkey: aliceKp.publicKey, isSigner: true, isWritable: false},
      {pubkey: aliceTokenPk, isSigner: false, isWritable: true},
      {pubkey: aliceLockState, isSigner: false, isWritable: true},
      {pubkey: lockStateTokenAccount.address, isSigner: false, isWritable: true},
      {pubkey: TOKEN_PROGRAM_ID, isSigner:false, isWritable: false},
      {pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false }
    ],
    data: lockBuffer
  });

  const tx = new Transaction().add(
    createAccountIx,
    lockTokenIx,
  );

  console.log("sending transaction");
  let sig = await connection.sendTransaction(
    tx, 
    [aliceKp],
  );
  console.log("signature: ", sig);

  console.log('waiting 5 seconds before checking data')
  setTimeout(await checkAccountData, 5000,
    connection, 
    aliceTokenPk, 
    lockStateTokenAccount.address,
    alicesTokenAmount,
  );

  setTimeout(await tryUnlockTransaction, 10000,
    connection,
    aliceKp,
    aliceTokenPk,
    aliceLockState,
    lockStateTokenAccount.address,
  );
}

const checkAccountData = async(
  connection: Connection,
  aliceTokenPk: PublicKey,
  lockTokenPk: PublicKey,
  expectedTokenAmount: number,
) => {
  console.log("checking data")
  let aliceTokenAccount = await getAccount(connection, aliceTokenPk, "confirmed", TOKEN_PROGRAM_ID);
  let lockTokenAccount = await getAccount(connection, lockTokenPk, "confirmed", TOKEN_PROGRAM_ID);

  let aliceTokenAccountAmount = parseInt(aliceTokenAccount.amount.toString(), 10);
  let lockTokenAccountAmount = parseInt(lockTokenAccount.amount.toString(), 10);

  console.log("alice token account amount", aliceTokenAccountAmount);
  console.log("lock token account amount", lockTokenAccountAmount);
  if (aliceTokenAccountAmount !== 0) {
    throw new Error("Alice's token balance should be zero!")
  }
  if (lockTokenAccountAmount !== expectedTokenAmount) {
    throw new Error("The lock state token account's balance should be zero!")
  }
  console.log("SUCCESS!!!")
}

const tryUnlockTransaction = async(
  connection: Connection,
  aliceKp: Keypair,
  aliceTokenPk: PublicKey,
  aliceLockState: PublicKey,
  lockStateTokenAccount:PublicKey,
) => {
  console.log("building a transaction that should fail");
  let unlockTokenSchema = borsh.struct([
    borsh.u8('instruction'),
  ]);
  let unlockBuffer = Buffer.alloc(1000);
  unlockTokenSchema.encode(
    {
      instruction: 2,
    },
    unlockBuffer,
  );
  unlockBuffer = unlockBuffer.slice(0, unlockTokenSchema.getSpan(unlockBuffer));
  let unlockTokenIx = new TransactionInstruction({
    programId,
    keys: [
      {pubkey: aliceKp.publicKey, isSigner: true, isWritable: false},
      {pubkey: aliceTokenPk, isSigner: false, isWritable: true},
      {pubkey: aliceLockState, isSigner: false, isWritable: true},
      {pubkey: lockStateTokenAccount, isSigner: false, isWritable: true},
      {pubkey: TOKEN_PROGRAM_ID, isSigner:false, isWritable: false},
      {pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false }
    ],
    data: unlockBuffer
  });

  const unlockTx = new Transaction().add(
    unlockTokenIx,
  );

  let unlockSig = await connection.sendTransaction(
    unlockTx,
    [aliceKp],
  );
  console.log("unlock tx sig", unlockSig);
}

function delay(timeInMillis: number): Promise<void> {
  return new Promise((resolve) => setTimeout(() => resolve(), timeInMillis));
}

clientScript();

