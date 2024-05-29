select default::Vendor {
  pubkey,
  products: {
    mint_authority,
    mint_account,
    metadata: {
      name,
      symbol,
      uri,
      additional
    },
    block_ts := .tx.block_ts,
    device_count := count(.devices)
  }
}
filter .pubkey = <str>$vendor_pubkey
