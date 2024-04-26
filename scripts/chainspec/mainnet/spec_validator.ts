import { assertEquals, assert, assertNotEquals } from "https://deno.land/std@0.223.0/assert/mod.ts";

const read_file = Deno.readTextFileSync("../../../misc/genesis/mainnet.chain.spec.json");
const chain_spec = JSON.parse(read_file);

const genesis = chain_spec["genesis"]["runtimeGenesis"]["patch"];

// Mainnet Chain Spec validation
let accounts_that_needs_funding: string[] = [];

// Bootnodes TODO

// Technical Committee
console.log("Checking Technical Committee...");
{
    const actual_members = genesis["technicalCommittee"]["members"];
    const expected_members = [
        "5HQrpJz56YGbbrdx2zuHnHswgG4DtDXHXuRdzcHmwWjV6ZCj", 
        "5HpXW1Hf8Y9TQ2nkPXDRJts4TCAhJkSLaSLg3b5PyGSFstce",
        "5D58at8TNNJTMhk9uSGDC1mMibifh8wBtg7ZA7kbXmYKhnVP",
        "5G22uXJAhBjsyD4G3faNsrV8hCrbbeQYwq8v9jXQG5uLzNCc",
        "5CoVaWrZnaV3BSeUJCA8Ca3SPMJDtjT1zPvZkzovkxJU7dkr",
        "5FWvYmrz6ZupPjG6PMJhm3XPmhf8QLUELUbudH8oEtDwfn2n",
        "5EEwFNeCahA19W99mX3Fedh3q9b9G6MADZzv1myQi2VW6daq"
    ];
    assertEquals(actual_members, expected_members, "Technical Committee Members are not OK");
    accounts_that_needs_funding = accounts_that_needs_funding.concat(expected_members);
}


// Treasury Committee
console.log("Checking Treasury Committee...");
{
    const actual_members = genesis["treasuryCommittee"]["members"];
    const expected_members = [
        "5D4xZY3K9J5AT4P5DBwNQe6uAstyG5x76t41km4Pio6nD5T4", 
        "5CM6cxth8hh6wEWsK7FXguFhyrA8r33BUPxuQgNUZR8RhnNh",
        "5Ek3BuV6NnVymsXGJRui7BH8RdzZyTHWa1nQmAFmXesb5h7o",
        "5FEs8KEjWvCXsJpumse5mwBXY2aNQJ8pwuoExjJttZkfHyXn",
        "5F9KZ7sPAwwSY7miS7MWQPbTqQcrJTVFFSboFESpPaYpszbu",
        "5HT8nipVsJvPFBmR7pFgxkurfgufn9hxVmRpwy8TPY97xoY1",
        "5EcehPTkTdT8XhLdaRxSq9L3EYkt3xqaWziVffHf9mzSbT6H"
    ];
    assertEquals(actual_members, expected_members, "Treasury Committee Members are not OK");
    accounts_that_needs_funding = accounts_that_needs_funding.concat(expected_members);
}

// Sudo
console.log("Checking Sudo...");
{
    const actual = genesis["sudo"]["key"];
    const expected = "5HL2vXZwYzAPn3uhjmDhbJwcySZnZqUfEyC2epWfDvz6YcBt";
    assertEquals(actual, expected, "Sudo is not OK");
    accounts_that_needs_funding = accounts_that_needs_funding.concat(expected);
}

// Staking
console.log("Checking Staking...");
console.log("   Checking Validator Count");
{
    const actual = genesis["staking"]["validatorCount"].toString();
    const expected = "8";
    assertEquals(actual, expected, "Staking Validator Count is not OK");
}

console.log("   Checking Stakers...");
{
    const actual_stakers = genesis["staking"]["stakers"];
    const expected_stakers = [
        "5DvV9ja7y1iLwyyHcXUhSTTPLBt6pfXzcjtEQBVDvLPMrCFc", 
        "5GuPR92DPMtfRQsTnhNoChi5NXRsYku8Qr5vJK3DdWxhhf1w",
        "5GMqZDmBjfTG2NmknpwU74eBgh6kVf9XywxyErxu3BbMFZat",
        "5FXG7qY4JcUYWPSdsncwwavQq7jsYTTS1DVfVh1WQndSehmU",
        "5FGxV7MLTXS5oCsQrctv3pquZZP21Uv9EXhfht83h3cvKkFi",
        "5FX6PRNa7j6UYLJAG2YfYqLu4HeTZZSEr1nTkhBXeQWdhKpr",
        "5EHxoz7vjDyd9nEmqfc7Lgs47zoRhZdfxRwJ9UYoRGrqnfmE",
        "5DcP87pHMRYF9N3RNDdcwJMUU2kNJAickdACtnsVmmf5Qkp3"
    ];
    assertEquals(actual_stakers.length, expected_stakers.length);

    for(let i = 0; i < expected_stakers.length; ++i) {
        assertEquals(actual_stakers[i][0], expected_stakers[i]);
        assertEquals(actual_stakers[i][1], expected_stakers[i]);
        assertEquals(actual_stakers[i][3], "Validator");
    }

    accounts_that_needs_funding = accounts_that_needs_funding.concat(expected_stakers);
}

// Session
console.log("Checking Session Keys...");
{
    const actual_keys = genesis["session"]["keys"];
    const expected_stash_accounts = [
        "5DvV9ja7y1iLwyyHcXUhSTTPLBt6pfXzcjtEQBVDvLPMrCFc", 
        "5GuPR92DPMtfRQsTnhNoChi5NXRsYku8Qr5vJK3DdWxhhf1w",
        "5GMqZDmBjfTG2NmknpwU74eBgh6kVf9XywxyErxu3BbMFZat",
        "5FXG7qY4JcUYWPSdsncwwavQq7jsYTTS1DVfVh1WQndSehmU",
        "5FGxV7MLTXS5oCsQrctv3pquZZP21Uv9EXhfht83h3cvKkFi",
        "5FX6PRNa7j6UYLJAG2YfYqLu4HeTZZSEr1nTkhBXeQWdhKpr",
        "5EHxoz7vjDyd9nEmqfc7Lgs47zoRhZdfxRwJ9UYoRGrqnfmE",
        "5DcP87pHMRYF9N3RNDdcwJMUU2kNJAickdACtnsVmmf5Qkp3"
    ];

    const expected_session_keys = [
        {
            "authority_discovery": "5CwJxm67vcXZ7U8VVCPm9h9ADBB42vNUCyVGwyQUjugsMWKE",
            "babe": "5CwJxm67vcXZ7U8VVCPm9h9ADBB42vNUCyVGwyQUjugsMWKE",
            "grandpa": "5DrvsiEJpHibsLgCGmRg43Qq18NHQzd89PEQzuYMj4GPEBZC",
            "im_online": "5CwJxm67vcXZ7U8VVCPm9h9ADBB42vNUCyVGwyQUjugsMWKE" 
        },
        {
            "authority_discovery": "5DUxdn6djBW7aMTwvaMvb8ty9i36qyn5vete6gQrJzBijswq",
            "babe": "5DUxdn6djBW7aMTwvaMvb8ty9i36qyn5vete6gQrJzBijswq",
            "grandpa": "5GNSGaKbzGF4QgLbKQoRwwHLu2vAhwVAN1oRuSt8zS4DM3FV",
            "im_online": "5DUxdn6djBW7aMTwvaMvb8ty9i36qyn5vete6gQrJzBijswq"
        },
        {
            "authority_discovery": "5ELFgZdQtnfnhv9gUMxu7kT7K4rZ3JXze7td4CSgRFu2UFFm",
            "babe": "5ELFgZdQtnfnhv9gUMxu7kT7K4rZ3JXze7td4CSgRFu2UFFm",
            "grandpa": "5FbuCKrH9ZtPpo9rz9dokoWgCb1w3QjaFeuFHJ1riZqHRhQe",
            "im_online": "5ELFgZdQtnfnhv9gUMxu7kT7K4rZ3JXze7td4CSgRFu2UFFm"
        },
        {
            "authority_discovery": "5GGqdgLAAH9ZF718hjPuUgPgNh4owsMejkJwS9zosyMWvJoo",
            "babe": "5GGqdgLAAH9ZF718hjPuUgPgNh4owsMejkJwS9zosyMWvJoo",
            "grandpa": "5FGBnS3jNDfVE8pYrZq4qLeXhxmhzu33AHUdvecS7LTVYXUJ",
            "im_online": "5GGqdgLAAH9ZF718hjPuUgPgNh4owsMejkJwS9zosyMWvJoo"
        },
        {
            "authority_discovery": "5Gb19xRRPGuwPRqeoejGz69Vhjc4TBoLUNTckVxiFWZBBkHB",
            "babe": "5Gb19xRRPGuwPRqeoejGz69Vhjc4TBoLUNTckVxiFWZBBkHB",
            "grandpa": "5C5PVWHxGXVzwfjrhi59Sa34EnBE6Y2q5FTRx3Fey2ZnYGoM",
            "im_online": "5Gb19xRRPGuwPRqeoejGz69Vhjc4TBoLUNTckVxiFWZBBkHB"
        },
        {
            "authority_discovery": "5DXayiN8jpg6iiGe2sCDM6LnVPT18prgKER4kkTzQ2o4W4Mk",
            "babe": "5DXayiN8jpg6iiGe2sCDM6LnVPT18prgKER4kkTzQ2o4W4Mk",
            "grandpa": "5DCBp3hVGvur2e1KsXmZZJS2Ctua7WZFkYqoixAQNtAJPy7x",
            "im_online": "5DXayiN8jpg6iiGe2sCDM6LnVPT18prgKER4kkTzQ2o4W4Mk"
        },
        {
            "authority_discovery": "5DfHXsGkYhVHzhFdDUrhbg4TYxEhNPRUBCwTdM4CsW1KhGa6",
            "babe": "5DfHXsGkYhVHzhFdDUrhbg4TYxEhNPRUBCwTdM4CsW1KhGa6",
            "grandpa": "5DEKGTdQHVrW6Yt1KLGG94mfCw7TBuCmN78VEaqd9bubxTZF",
            "im_online": "5DfHXsGkYhVHzhFdDUrhbg4TYxEhNPRUBCwTdM4CsW1KhGa6"
        },
        {
            "authority_discovery": "5GeYPskHoX9mAgBseWGyPRuXTSaLMVpGu7ePUhdpZK4fDmhB",
            "babe": "5GeYPskHoX9mAgBseWGyPRuXTSaLMVpGu7ePUhdpZK4fDmhB",
            "grandpa": "5CB9iBTtuwFsGvqfCSAC61xf7bJuW22ZMMLD86YrYfKkCBVM",
            "im_online": "5GeYPskHoX9mAgBseWGyPRuXTSaLMVpGu7ePUhdpZK4fDmhB"
        }
    ]

    assertEquals(actual_keys.length, expected_stash_accounts.length);
    assertEquals(expected_session_keys.length, expected_stash_accounts.length);

    for(let i = 0; i < expected_stash_accounts.length; ++i) {
        assertEquals(actual_keys[i][0], expected_stash_accounts[i]);
        assertEquals(actual_keys[i][1], expected_stash_accounts[i]);
        assertEquals(actual_keys[i][2], expected_session_keys[i]);
    }
}

// Additional Balance accounts
accounts_that_needs_funding.push("5G41ZeH7Yigm6eVcyRzptt3Tc6P8kJhn98wtTGHqfD2qg9xp");
accounts_that_needs_funding.push("5E52kfQfQWiGbPxrsUkVzbcpa5YYHkiR4GSF4YzgdbjNJGR5");
accounts_that_needs_funding.push("5DviL963HpeTe8pdabWZTesg7UDk66QehmBbtoC2xgLH8PBX");
accounts_that_needs_funding.push("5Do8zxAbZmsV3Eqk6aUJk8wsg4eME3FysWfrp7VBANCe1PR1");
accounts_that_needs_funding.push("5D4eMx5gk48p1jzZMyioWRumhP2Hqctf9RHDMCf1ceQEhvbi");
accounts_that_needs_funding.push("5EJJA5scQCAGGqMShaq88qwoSAD3J9VNM8zPxrd2tkQjBkTV");
accounts_that_needs_funding.push("5Fy1Kh2c2Rv87u4V7ck4ewUmofQGtwB1uMHQWwA9XbH1FNce");
accounts_that_needs_funding.push("5FUU1HDBYfzVjUaxcdibUd59QsAYh2nNLYNK1i2TjjZQEhyc");
accounts_that_needs_funding.push("5FtmS7yZvxps28shKggLJejmDtsxyRgzhcS68sNFALuxw1UP");
accounts_that_needs_funding.push("5GyjM2XHpeWGMpsKDv2bkPmrpniufxXrs2GAYsDoduVtoxj3");
accounts_that_needs_funding.push("5FVVqhFAgEzhUk9M126iQ3Luv7Z5feAboTDiJEzAiBgycEX3");
accounts_that_needs_funding.push("5FZf49eUCD1H4vLXrgSSDN3v67XGhrzpiJN8UqhLogaD3u4Z");
accounts_that_needs_funding.push("5H8j3h8VaADoV6v6AuSw814YCrSZy3N8XameFLPg61h3RrV8");
accounts_that_needs_funding.push("5HKFLPzhYJzUinpc2RonqeQQ1m6QuXtsBJjbwyK4hPnSb14w");


// Deduping
{
    const unique: string[] = [];
    for(let i = 0; i < accounts_that_needs_funding.length; ++i) {
        const address = accounts_that_needs_funding[i];
        assertEquals(unique.find((v) => v == address), undefined);
        unique.push(address);
    }
}


// Balance addresses
console.log("Checking Balance Addresses...");
{
    const balances = genesis["balances"]["balances"];
    assertEquals(balances.length, accounts_that_needs_funding.length);
    for(let i = 0; i < accounts_that_needs_funding.length; ++i) {
        const address = accounts_that_needs_funding[i];
        assertNotEquals(balances.find((v: any) => v[0] == address), undefined, "Failed to find a balance address");
    }
}

// DA App Keys
console.log("Checking DA App Keys");
{
    const appkeys = genesis["dataAvailability"]["appKeys"];
    const expected_owner = "5HL2vXZwYzAPn3uhjmDhbJwcySZnZqUfEyC2epWfDvz6YcBt" // TODO

    for(let i = 0; i < appkeys.length; ++i) {
        const actual_owner = appkeys[i][1][0];
        assertEquals(actual_owner, expected_owner);
    }
}
