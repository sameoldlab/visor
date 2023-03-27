a GUI for [anvil](https://github.com/foundry-rs/foundry/blob/master/anvil)

Inspired by [Ganache UI](https://github.com/trufflesuite/ganache-ui)


![image](public/ui.png)

⚠⚠ functional, but early WIP⚠⚠

no package yet, but you can: 
```sh 
pnpm install
mkdir -p ./public/bin
ln -s /path/to/anvil ./public/bin/anvil-$TARGET-TRIPLE
npx tauri build
```
to get `$TARGET-TRIPLE` copy output from: `rustc -Vv | grep host | cut -f2 -d' '`

---
## Todo: 
- [x] Anvil Log
- [x] Accounts (Basic)
- [x] Blocks (Basic)
- [x] Transactions (Basic)
- [ ] Test RPC calls (Impersonate, get tokens, rollback, mine, etc.)
- [ ] Setup GUI
- [ ] Save Environment
- [ ] Accounts
- [ ] Blocks