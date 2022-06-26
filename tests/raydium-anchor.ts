import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { RaydiumAnchor } from '../target/types/raydium_anchor';

describe('raydium-anchor', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.RaydiumAnchor as Program<RaydiumAnchor>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
