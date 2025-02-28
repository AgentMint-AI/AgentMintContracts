// import * as anchor from '@project-serum/anchor';
// import { PublicKey, Keypair, SystemProgram } from '@solana/web3.js';

// // Define the program ID and the provider
// const PROGRAM_ID = new PublicKey('GvpMGor7n2G2qqpMJHgkFaqWLNz55STM5ABVCR5D2UJr'); // Replace with your program ID
// const provider = anchor.AnchorProvider.env();
// anchor.setProvider(provider);

// // Define the interface for the UserTokens account
// interface UserTokens {
//     tokens: PublicKey[];
// }

// // Load the program
// const program = new anchor.Program(idl, PROGRAM_ID, provider);

// // Function to create a new token
// async function createToken(name: string, symbol: string, uri: string, decimals: number) {
//     // Generate a new keypair for the user
//     const user = Keypair.generate();

//     // Create the user_tokens account
//     const userTokensAccount = Keypair.generate();

//     // Create the transaction to create the token
//     const tx = await program.rpc.createToken(name, symbol, uri, decimals, {
//         accounts: {
//             user: user.publicKey,
//             userTokens: userTokensAccount.publicKey,
//             tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
//             systemProgram: SystemProgram.programId,
//         },
//         signers: [user, userTokensAccount],
//     });

//     console.log("Transaction signature", tx);
// }

// // Call the createToken function
// createToken("MyToken", "MTK", "https://mytoken.com/metadata", 9)
//     .then(() => console.log("Token created successfully"))
//     .catch(err => console.error("Error creating token:", err));
