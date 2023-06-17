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
            columns = columns.filter(x => x !== 'F' && x!== 'E')
            if(columns.length === 6 || columns.length === 5){
                rerows.push(row)
            }
        }
    }
    return rerows
}

type BoughtsRow = {
    id : number,
    pill_number : number,
    cargo_name : string,
    cargo_number : number,
    cost : number,
    type_number : number,
    date : number,
}

export async function scrapBoughtsSheet(path : string) {
    const sheet : Map<string,any> 
    = map(map(await xlsx.readFile(path).Sheets).get("اصناف "))
    const rows : BoughtsRow[] = [];
    for(const rowNumber of validateKeys(sheet.keys())){
        const getKey = (x : string) => sheet.get(x + rowNumber)?.v;
        const row : BoughtsRow = {
            id : rowNumber,
            pill_number : getKey('A') as number,
            cargo_name : getKey('B') as string,
            cargo_number : getKey('C') as number,
            cost : getKey('D') as number,
            type_number : getKey('G') || rows.find(x => x.id === (rowNumber -1))?.type_number,
            date : getKey('H') as number,
        };
        if(typeof row.pill_number === 'number'){
            rows.push(row)
        }
    }
    return rows;
}