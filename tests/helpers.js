const fs = require("fs");
const {Keypair} = require("@solana/web3.js");

async function createKeypairFromFile(
    filePath,
) {
    const secretKeyString = await fs.readFileSync(filePath);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

async function printProgramLogsForSignature(connection, signature) {
    let output = await connection.getTransaction(signature)
    console.log("************  Program Logs  ************")
    console.log(output['meta']['logMessages'].reduce((acc,log) => acc += "\n" + log))
}
async function initializeLocalPayer(connection) {
    const keypair = await createKeypairFromFile(process.env.KEYPAIR_PATH)
    let balance = await connection.getBalance(keypair.publicKey);
    if (balance < 1000000000) {
        console.log("Requesting 1 Sol airdrop for payer: ", keypair.publicKey.toBase58())
        await connection.requestAirdrop(keypair.publicKey, 1000000000);
        console.log("Airdrop Successful")
    }
    return keypair
}

module.exports = {initializeLocalPayer, printProgramLogsForSignature, createKeypairFromFile}