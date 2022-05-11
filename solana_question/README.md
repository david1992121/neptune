## Solana Question
We have the beginnings of a basic Escrow program written in vanilla Solana. We also have a basic typescript client to interact with the program. The escrow program has three endpoints:

* `process_create_accounts`: this creates the accounts needed for a user escrow.
* `process_lock_tokens`: locks a user's tokens within a program controlled token account. Sets the unlock date for the escrow to be `1_000_000` seconds in the future.
* `process_unlock_tokens`: unlocks tokens from a program controlled escrow account if we've reached the escrow's unlock time.

Your assignment is to implement `process_lock_tokens` and `process_unlock_tokens` in the `processor.rs` file. We will be evaluating your ability to implement the program specifications, as well as the security of your program. The typescript client and the program's other rust files will be helpful for understanding the program's specs, but you will not need to make any changes to them. You are free to use any Solana documentation or Rust resources that you can find online throuhgout this interview.

The client is written such that it can be run easily as a script, and it contains tests to validate that your program works as expected. Once the program is written to your satisfaction, you can test by deploying the program to devnet and copy-pasting your program ID into the `programId` created on line 23 of `client.ts` Then, compile the typescript client via `tsc` and run the compiled javascript file with `node`. The client will take the following steps:

* Create a user named Alice 
* Create a new spl token mint
* Mint alice 1000 spl tokens
* Call `process_create_accounts`
* call `process_lock_tokens`
* call `process_unlock_tokens`

After the client calls `process_lock_tokens`, Alice's token account balance should be 0 and Alice's escrow account balance should be 1000. The call to `process_unlock_tokens` should fail because we haven't yet reached the unlock time for Alice's escrow. 
