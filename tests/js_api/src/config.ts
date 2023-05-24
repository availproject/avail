export interface Config {
  mnemonic: string;
  size: number;
  ApiURL: string;
  app_id: number;
  batch?: number;
  count?: number;
  amount?: number;
  receiver?: string;
}

export default {
  mnemonic: 'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice',
  size: 0,
  ApiURL: 'wss://testnet.avail.tools/ws',
  app_id: 0,
  batch: 0,
  count: 0,
  amount: 0,
  receiver: '',
};
