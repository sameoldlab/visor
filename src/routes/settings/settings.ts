const EvmOpts = [
  {
    title: "Accounts",
    description: "Set the number of accounts. Default: 10",
    type: 0,
    args: "--accounts",
  },
  {
    title: "Account Balance",
    description: "Set the balance of the accounts. Default: 10000",
    type: 0,
    args: "--balance",
  },
  {
    title: "Derivation Path",
    description:
      "Set the derivation path of the child key to be derived [default: m/44'/60'/0'/0/]",
    type: "string",
    args: "--derivation-path",
  },
  {
    title: "Block Time",
    description: "Block time in seconds for interval mining",
    type: 0,
    args: "--block-time",
  },
  {
    title: "No mining",
    description: "Disable auto and interval mining, and mine on demand instead",
    type: true,
    args: "--no-mining",
  },
  {
    title: "Hardfork",
    description: "Choose the EVM hardfork to use. Default: latest",
    type: [
      { id: 1, name: "Latest", unavailable: false },
      { id: 2, name: "Geof", unavailable: false },
      { id: 3, name: "Work", unavailable: false },
      { id: 4, name: "Wdfs", unavailable: false },
    ],
    args: "--hardfork",
  },
  {
    title: "Mnemonic",
    description: "BIP39 mnemonic phrase used for generating accounts",
    type: "",
    args: "--mnemonic",
  },
  {
    title: "Fork URL",
    description:
      "Fetch state over a remote endpoint instead of starting from an empty state",
    type: "",
    args: "--fork-url",
  },
  {
    title: "Genesis File",
    description:
      "Initialize the genesis block with the given genesis.json file",
    type: "",
    args: "-init",
  },
]

export default EvmOpts
