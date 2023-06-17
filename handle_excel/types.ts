import { map, xlsx } from "./packages.ts";

function validateKeys(keys: IterableIterator<string>): number[] {
  const rows: Map<number, string[]> = new Map();
  for (const key of keys) {
    const column: string = key.slice(0, 1);
    const row: number = +key.slice(1);
    if (row) {
      rows.set(row, [column, ...(rows.get(row) || [])]);
    }
  }
  const rerows: number[] = [];
  for (let [row, columns] of rows.entries()) {
    if (columns.length >= 6) {
      columns = columns.filter((x) => x !== "I" && x !== "H");
      if (columns.length === 6) {
        rerows.push(row);
      }
    }
  }
  return rerows;
}

type TypesRow = {
  id: number;
  date: number;
  pill_number: number;
  type_number: number;
  type_name: string;
  type_quantity: number;
  cost: number;
};

export async function scrapTypesSheet(path: string) {
  const sheet: Map<string, any> = map(
    map(await xlsx.readFile(path).Sheets).get("كارت صنف المبيعات"),
  );
  const rows: TypesRow[] = [];
  for (const rowNumber of validateKeys(sheet.keys())) {
    const getKey = (x: string) => sheet.get(x + rowNumber)?.v;
    const row: TypesRow = {
      id: rowNumber,
      date: getKey("B") as number,
      pill_number: getKey("C") as number,
      type_number: typeof getKey("D") === "number"
        ? getKey("D")
        : +getKey("D").trim(),
      type_name: getKey("E") as string,
      type_quantity: getKey("F") as number,
      cost: getKey("G") as number,
    };
    if (typeof row.pill_number === "number") {
      rows.push(row);
    }
  }
  return rows;
}
