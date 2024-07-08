// GENERATED by @edgedb/generate v0.5.3

import type {Executor} from "edgedb";

export type GetVendorsArgs = {
  readonly "offset": number;
  readonly "limit": number;
};

export type GetVendorsReturns = Array<{
  "pubkey": string;
  "products_count": number;
  "products": Array<{
    "mint_authority": string | null;
    "mint_account": string;
    "metadata": {
      "name": string | null;
      "symbol": string | null;
      "uri": string | null;
      "additional": Array<[string, string]>;
    } | null;
    "block_ts": Date | null;
    "device_count": number;
  }>;
}>;

export function getVendors(client: Executor, args: GetVendorsArgs): Promise<GetVendorsReturns> {
  return client.query(`\
select default::Vendor {
  pubkey,
  products_count := count(.products),
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
offset <int64>$offset
limit <int64>$limit`, args);

}
