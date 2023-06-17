import {xlsx,map} from './packages.ts'

function validateKeys(keys : IterableIterator<string>) : number[] {
    const rows : Map<number,string[]> = new Map();
    for(const key of keys){
        const column : string = key.slice(0,1)
        const row : number = +key.slice(1)
        if(row) {
            rows.set(row,[column,...(rows.get(row) || [])])
        }
    }
    const rerows : number[] = [];
    for(let [row,columns] of rows.entries()){
        if(columns.length >= 5){
            columns = columns.filter(x => x !== 'I' && x!== 'G' && x !== 'H')
            if(columns.length === 5){
                rerows.push(row)
            }
        }
    }
    return rerows
}

type SellsRow = {
    id : number,
    date : number,
    pill_number : number,
    tax_number : number,
    client_name : string,
    value : number,
}

export async function scrapSellsSheet(path : string) {
    const sheet : Map<string,any> 
    = map(map(await xlsx.readFile(path).Sheets).get("المبيعات"))
    const rows : SellsRow[] = [];
    for(const rowNumber of validateKeys(sheet.keys())){
        const getKey = (x : string) => sheet.get(x + rowNumber)?.v;
        const row : SellsRow = {
            id : rowNumber,
            date : getKey('B') as number,
            pill_number : getKey('C') as number,
            tax_number : getKey('D') as number,
            client_name : getKey('E') as string,
            value : getKey('F') as number
        };
        if(typeof row.pill_number === 'number'){
            rows.push(row)
        }
    }
    return rows;
}