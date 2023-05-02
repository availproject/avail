export interface Config
{
    mnemonic: String;
    size: number;
    ApiURL: String;
    app_id: number;
    batch?: number;
    count?: number;
    amount?: number;
    receiver?: String;
}

export default{
    mnemonic: "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
    size: 100,
    ApiURL: "wss://testnet.avail.tools/ws",
    app_id: 0,
    batch: 0,
    count: 0,
    amount: 0,
    receiver: ''
}