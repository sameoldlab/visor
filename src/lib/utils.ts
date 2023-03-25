export const trunc = (hex: string) =>
`${hex.slice(0, 6)}~${hex.slice(hex.length - 5, hex.length - 1)}`

export const naturalDate = (unix_time: bigint) =>
new Date(Number(unix_time) * 1e3).toLocaleString() // Internationalization for freee? Should probably test this

export const formatEther = (gwei: bigint, fractionDigits = 4) => (Number(gwei)/1e18).toFixed(fractionDigits)