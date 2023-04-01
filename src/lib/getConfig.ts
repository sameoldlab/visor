import { exists, readTextFile, BaseDirectory } from "@tauri-apps/api/fs"
import type { Address } from "viem"

const getConfig = async () => {
  const exist = await exists("config.json", {
    dir: BaseDirectory.AppLocalData,
  })
  if (!exist) throw new Error("config file does not exist")

  const config = await readTextFile("config.json", {
        dir: BaseDirectory.AppLocalData,
      })

  return (JSON.parse(config) as {
      available_accounts: Address[]
      base_fee: string
      gas_limit: string
      gas_price: string
      genesis_timestamp: string
      private_keys: Address[]
      wallet: { mnemonic: string; derivation_path: string }
    }) 
}

export default getConfig