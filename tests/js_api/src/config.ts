import defaultConfig from './config.json'

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
    mnemonic: defaultConfig.mnemonic || "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
    size: defaultConfig.size || 100,
    ApiURL: defaultConfig.ApiURL || "wss://testnet.avail.tools/ws",
    app_id: defaultConfig.app_id || 0,
    batch: defaultConfig.batch || 0,
    count: defaultConfig.count || 0,
    amount: defaultConfig.amount || 0,
    receiver: defaultConfig.receiver || ''
}