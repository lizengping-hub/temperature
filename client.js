// client.js is used to introduce the reader to generating clients from IDLs.
// It is not expected users directly test with this example. For a more
// ergonomic example, see `tests/basic-0.js` in this workspace.

const anchor = require('@project-serum/anchor');
const PublicKey = anchor.web3.PublicKey

async function main() {
  // #region main
  // Read the generated IDL.
    process.env.ANCHOR_WALLET="/root/.config/solana/id.json"
    process.env.ANCHOR_PROVIDER_URL="https://api.mainnet-beta.solana.com"
    anchor.setProvider(anchor.Provider.env());
  const idl = JSON.parse(require('fs').readFileSync('./target/idl/raydium_anchor.json', 'utf8'));

  // Address of the deployed program.
  const programId = new anchor.web3.PublicKey('BJDz4e9rWPnnhDaRp8sEUxvSSvr28zWoZoNGxeeJ5mSS');

  // Generate the program client from IDL.
  const program = new anchor.Program(idl, programId);
  // The Account to create.
  const base = anchor.web3.Keypair.generate();
  const seeds = [
      Buffer.from(anchor.utils.bytes.utf8.encode("Pool")),
      base.publicKey.toBytes()
  ]
    const [test,bump] = await PublicKey.findProgramAddress(
        seeds,
        programId,
    );
    console.log("base",base.publicKey.toString())
    console.log("test",test.toString())
    const accounts = {
        base: base.publicKey,
            test,
            payer: anchor.getProvider().wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
    }
    console.log(accounts)
  await program.rpc.testInit(
      new anchor.BN(bump),
      [new PublicKey('BJDz4e9rWPnnhDaRp8sEUxvSSvr28zWoZoNGxeeJ5mSS')],
      [new PublicKey('BJDz4e9rWPnnhDaRp8sEUxvSSvr28zWoZoNGxeeJ5mSS'),new PublicKey('BJDz4e9rWPnnhDaRp8sEUxvSSvr28zWoZoNGxeeJ5mSS')],
      new anchor.BN(bump),
      {
          accounts,
        signers: [],
      }
  )

}

console.log('Running client.');
main().then(() => console.log('Success'));
