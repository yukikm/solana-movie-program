pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

//
// https://explorer.solana.com/tx/2dYQZFZxK6LG4FCXVca8Hfb3WZoU1pTDVxRYRdzMSEcgdRqxympo6gLyvDX7oPBhe1NLdCBqqLHrNTmXTjUn7Y3N?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899
// kimurayuki@kimurayukinoMacBook-Pro solana-movie-program % solana logs HUUony9h2sNwZf1zfbHrMW5QW2h7qb9G3MJEHFW8nJ46
// Streaming transaction logs mentioning HUUony9h2sNwZf1zfbHrMW5QW2h7qb9G3MJEHFW8nJ46. Confirmed commitment
// Transaction executed in slot 551:
//   Signature: 2dYQZFZxK6LG4FCXVca8Hfb3WZoU1pTDVxRYRdzMSEcgdRqxympo6gLyvDX7oPBhe1NLdCBqqLHrNTmXTjUn7Y3N
//   Status: Ok
//   Log Messages:
//     Program HUUony9h2sNwZf1zfbHrMW5QW2h7qb9G3MJEHFW8nJ46 invoke [1]
//     Program log: process_instruction: HUUony9h2sNwZf1zfbHrMW5QW2h7qb9G3MJEHFW8nJ46: 6 accounts, data=[3]
//     Program log: Token mint: DEeuS2AvCPZbsrxyEfiezQNfbo3EAra8X2EfWHD22VNX
//     Program log: Mint authority: 5L4eXag8Yv96SKKeoaCVWiW97osqZCjLDY56Tkb9mK3E
//     Program 11111111111111111111111111111111 invoke [2]
//     Program 11111111111111111111111111111111 success
//     Program log: Created token mint account
//     Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
//     Program log: Instruction: InitializeMint
//     Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2920 of 153936 compute units
//     Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
//     Program log: Initialized token mint
//     Program HUUony9h2sNwZf1zfbHrMW5QW2h7qb9G3MJEHFW8nJ46 consumed 49477 of 200000 compute units
//     Program HUUony9h2sNwZf1zfbHrMW5QW2h7qb9G3MJEHFW8nJ46 success




// https://explorer.solana.com/tx/2PR12w6UHnU2qyjhJvaofutKvfRwXnH7BXsgahKe4dVN1P7E8s7WLjEQJy7VErxZNfohAZeU94UHW4ozcUjaCkjm?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899
// kimurayuki@kimurayukinoMacBook-Pro solana-movie-program % solana program deploy ./target/deploy/pda_local.so 
// Program Id: CHsZNe2edPd7R1Ex1teZVkSrLp7JPsr2q3yTcvjJ5CfD

// kimurayuki@kimurayukinoMacBook-Pro solana-movie-program % solana logs CHsZNe2edPd7R1Ex1teZVkSrLp7JPsr2q3yTcvjJ5CfD 
// Streaming transaction logs mentioning CHsZNe2edPd7R1Ex1teZVkSrLp7JPsr2q3yTcvjJ5CfD. Confirmed commitment
// Transaction executed in slot 1296:
//   Signature: 2PR12w6UHnU2qyjhJvaofutKvfRwXnH7BXsgahKe4dVN1P7E8s7WLjEQJy7VErxZNfohAZeU94UHW4ozcUjaCkjm
//   Status: Ok
//   Log Messages:
//     Program CHsZNe2edPd7R1Ex1teZVkSrLp7JPsr2q3yTcvjJ5CfD invoke [1]
//     Program log: process_instruction: CHsZNe2edPd7R1Ex1teZVkSrLp7JPsr2q3yTcvjJ5CfD: 6 accounts, data=[3]
//     Program log: Token mint: FxoLyvvyqJonwPrbkipSz2bQu3J2Cbbdd4i47uBrcd45
//     Program log: Mint authority: G2aaqvk4L8xtKewwaxDwa1thSBKneoaRrNKhPVLD2Hsy
//     Program 11111111111111111111111111111111 invoke [2]
//     Program 11111111111111111111111111111111 success
//     Program log: Created token mint account
//     Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
//     Program log: Instruction: InitializeMint
//     Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2920 of 147832 compute units
//     Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success
//     Program log: Initialized token mint
//     Program CHsZNe2edPd7R1Ex1teZVkSrLp7JPsr2q3yTcvjJ5CfD consumed 55581 of 200000 compute units
//     Program CHsZNe2edPd7R1Ex1teZVkSrLp7JPsr2q3yTcvjJ5CfD success