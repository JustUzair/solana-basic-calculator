const assert = require("assert");
const anchor = require("@coral-xyz/anchor");
const { AnchorProvider } = require("@coral-xyz/anchor");
const {SystemProgram} = anchor.web3;

describe('calculator',function(){
    const provider = AnchorProvider.local()
    anchor.setProvider(provider)
    const calculator = anchor.web3.Keypair.generate();
    const program = anchor.workspace.Calculator

    it("creates a calculator", async ()=>{
        // create : is the function present under the program macro inside the module
        await program.rpc.create("Welcome to Solana!!",{
            accounts:{
                calculator:calculator.publicKey,
                user:provider.wallet.publicKey,
                systemProgram:SystemProgram.programId
            },
            signers:[calculator]
           
        });
        const account = await program.account.calculator.fetch(calculator.publicKey)
        // console.log(`RESULT : `,account);

        assert.ok(account.greeting == "Welcome to Solana!!")
    })

    it("adds two numbers",async ()=>{
        await program.rpc.add(new anchor.BN(4), new anchor.BN(3),{
            accounts:{
                calculator:calculator.publicKey
            }

        });
        const account = await program.account.calculator.fetch(calculator.publicKey)
        // console.log(`RESULT : `,account);

        assert.ok(account.result.eq(new anchor.BN(7)))
    })

    it("subtracts two numbers",async ()=>{
        await program.rpc.subtract(new anchor.BN(32), new anchor.BN(33),{
            accounts:{
                calculator:calculator.publicKey
            }

        });
                
        
        const account = await program.account.calculator.fetch(calculator.publicKey)
        // console.log(`RESULT : `,account);
       
        
        assert.ok(account.result.eq(new anchor.BN(-1)))
    })

    it("multiplies two numbers",async ()=>{
        await program.rpc.multiply(new anchor.BN(2), new anchor.BN(3),{
            accounts:{
                calculator:calculator.publicKey
            }

        });
        const account = await program.account.calculator.fetch(calculator.publicKey)
        // console.log(`RESULT : `,account);
        
        assert.ok(account.result.eq(new anchor.BN(6)))
    })


    it("divides two numbers",async ()=>{
        await program.rpc.divide(new anchor.BN(10), new anchor.BN(3),{
            accounts:{
                calculator:calculator.publicKey
            }

        });
        const account = await program.account.calculator.fetch(calculator.publicKey)
        // console.log(`RESULT : `,account);
        assert.ok(account.result.eq(new anchor.BN(3)))
        assert.ok(account.remainder.eq(new anchor.BN(1)))

    })
})