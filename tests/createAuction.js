const solanaWeb3 = require('@solana/web3.js');
const dotenv = require('dotenv');
const {TransactionInstruction, PublicKey, Transaction, Keypair, Connection, sendAndConfirmTransaction} = require("@solana/web3.js");
const fs = require("fs");
const {initializeLocalPayer, printProgramLogsForSignature} = require("./helpers");
const borsh = require("borsh");
dotenv.config({path: __dirname + '/.env'});

const programId = new PublicKey(process.env.PROGRAM_ID);


(async function() {
    const connection = new Connection("http://127.0.0.1:8899");
    let payer = await initializeLocalPayer(connection);
    let tx = new Transaction()

    class CreateAuction {
        constructor(data) {
            Object.assign(this, data)
        }
    }
    const tx_data_layout = new Map([
        [CreateAuction,
            {
                kind: 'struct',
                fields: [
                    ['instruction_id', 'u8'],
                    ['duration', 'u64'],
                    ['amount_of_tokens', 'u64'],
                    ['start_price', 'u64'],
                    ['reserve_price', 'u64'],
                    ['auction_item', 'u256'],
                ],
            },
        ],
    ])
    const data = new CreateAuction({
        instruction_id: 0,
        duration: 1999,
        amount_of_tokens: 10000,
        start_price: 200000,
        reserve_price: 200000,
        auction_item: Keypair.generate().publicKey._bn
    })
    const tx_data = borsh.serialize(tx_data_layout, data);

    let tx_ix = new TransactionInstruction({
        data: tx_data,
        keys: [{
            pubkey: payer.publicKey,
            isSigner: false,
            isWritable: false
        }],
        programId
    })
    tx.add(tx_ix)
    console.log("Processing Transaction...")
    let result = await sendAndConfirmTransaction(connection, tx, [payer])
    console.log("Result signature: ", result)
    await printProgramLogsForSignature(connection, result)
})()




