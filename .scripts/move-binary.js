// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * This script is used to rename the binary with the platform specific postfix.
 * When `tauri build` is ran, it looks for the binary name appended with the platform specific postfix.
 */
import { execa } from "execa"
import { existsSync, renameSync } from "fs"

let extension = ""
if (process.platform === "win32") {
  extension = ".exe"
}

async function main() {
  // get target triple for current platform
  const rustInfo = (await execa("rustc", ["-vV"])).stdout
  const targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
  if (!targetTriple) {
    throw new Error("Failed to determine platform target triple")
  }
  // Rename binary if it does not already exist
  if (!existsSync(`public/bin/anvil-${targetTriple}`))
    renameSync(`public/bin/anvil`, `public/bin/anvil-${targetTriple}${extension}`)
}

main().catch((e) => {
  throw e
})
