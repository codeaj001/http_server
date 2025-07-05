const axios = require("axios");

const HTTP_URL = process.env.HTTP_URL || "http://localhost:8080";

async function testKeypairGeneration() {
    try {
        const res = await axios.post(`${HTTP_URL}/keypair`);
        console.log("✓ Keypair generation:", res.data.success);
        return res.data.data;
    } catch (error) {
        console.log("✗ Keypair generation failed:", error.message);
        return null;
    }
}

async function testMessageSigning(keypair) {
    try {
        const res = await axios.post(`${HTTP_URL}/message/sign`, {
            message: "Hello, Solana!",
            secret: keypair.secret
        });
        console.log("✓ Message signing:", res.data.success);
        return res.data.data;
    } catch (error) {
        console.log("✗ Message signing failed:", error.message);
        return null;
    }
}

async function testMessageVerification(signature, message, pubkey) {
    try {
        const res = await axios.post(`${HTTP_URL}/message/verify`, {
            message: message,
            signature: signature,
            pubkey: pubkey
        });
        console.log("✓ Message verification:", res.data.success, "Valid:", res.data.data.valid);
        return res.data.data.valid;
    } catch (error) {
        console.log("✗ Message verification failed:", error.message);
        return false;
    }
}

async function main() {
    console.log("Testing Solana HTTP Server...");
    
    const keypair = await testKeypairGeneration();
    if (!keypair) return;
    
    const signedMessage = await testMessageSigning(keypair);
    if (!signedMessage) return;
    
    const isValid = await testMessageVerification(
        signedMessage.signature, 
        signedMessage.message, 
        signedMessage.pubkey
    );
    
    console.log("\nTest Summary:");
    console.log("- Keypair generation: ✓");
    console.log("- Message signing: ✓");
    console.log("- Message verification:", isValid ? "✓" : "✗");
}

main().catch(console.error);

