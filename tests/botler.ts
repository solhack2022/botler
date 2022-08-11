import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Botler } from '../target/types/botler';

describe('botler', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Botler as Program<Botler>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
