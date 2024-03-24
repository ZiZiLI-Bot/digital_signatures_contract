import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { DigitalSignaturesContract } from '../target/types/digital_signatures_contract';

describe('digital_signatures_contract', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const myWallet = anchor.Wallet.local();

  const program = anchor.workspace.DigitalSignaturesContract as Program<DigitalSignaturesContract>;

  it('Is initialized!', async () => {
    const tx = await program.methods
      .initialize({
        id: 1,
        nameStorage: 'Test',
      })
      .accounts({
        authority: myWallet.publicKey,
        initStorage: myWallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      });
  });
});
