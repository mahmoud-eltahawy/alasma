import { scrapSellsSheet } from "./sells.ts";
import { scrapBoughtsSheet } from "./boughts.ts";

import { scrapTypesSheet } from "./types.ts";

const path = Deno.args[0];
const sheet = Deno.args[1];

if (sheet === "BUY") {
  console.info(JSON.stringify(await scrapBoughtsSheet(path)));
} else if (sheet === "SELL") {
  console.info(JSON.stringify(await scrapSellsSheet(path)));
} else if (sheet === "TYPE") {
  console.info(JSON.stringify(await scrapTypesSheet(path)));
}
