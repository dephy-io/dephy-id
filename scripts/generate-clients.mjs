#!/usr/bin/env zx
import "zx/globals";
import * as k from "@metaplex-foundation/kinobi";
import path from "path";

// Instanciate Kinobi.
const idl = path.resolve('program', 'idl.json');
const kinobi = k.createFromIdl(idl);

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    "dephyIoDephyId": { name: "dephyId" },
  })
);

// Update accounts.
kinobi.update(
  k.updateAccountsVisitor({
    dephyAccount: {
      seeds: [
        k.constantPdaSeedNodeFromString('utf8', "DePHY"),
      ],
    },
  })
);


// Render JavaScript.
const jsClient = path.join(__dirname, "..", "clients", "js");
kinobi.accept(
  k.renderJavaScriptExperimentalVisitor(
    path.join(jsClient, "src", "generated"),
    { prettier: require(path.join(jsClient, ".prettierrc.json")) }
  )
);

const indexer = path.join(__dirname, '..', 'indexer')
kinobi.accept(
  k.renderJavaScriptExperimentalVisitor(
    path.join(indexer, "src", "generated"),
    { prettier: require(path.join(jsClient, ".prettierrc.json")) }
  )
);

// Render Rust.
const rustClient = path.join(__dirname, "..", "clients", "rust");
kinobi.accept(
  k.renderRustVisitor(path.join(rustClient, "src", "generated"), {
    formatCode: true,
    crateFolder: rustClient,
  })
);
