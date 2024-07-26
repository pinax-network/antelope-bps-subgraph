import { BigInt, BigDecimal, log } from "@graphprotocol/graph-ts";
import { Pays } from "./pb/antelope/bps/v1/Pays";
import { Pay } from "./pb/antelope/bps/v1/Pay";
import { Pay as PayEntity, Bp as BpEntity} from "../generated/schema";
import { Protobuf } from 'as-proto/assembly';
import { Timestamp } from "./pb/google/protobuf/Timestamp";

function addAssets(quantity1: string, quantity2: string): string {
    const asset1 = quantity1.split(" ");
    const asset2 = quantity2.split(" ");
    if (asset1[1] != asset2[1]) {
        throw `symbols mismach: ${asset1[1]} != ${asset2[1]}`;
    }
    const sum = parseFloat(asset1[0]) + parseFloat(asset2[0]);
    return `${toFixed(sum, 4)} ${asset1[1]}`;

}


function toBigDecimal(num: f64): BigDecimal {
    let numStr = toFixed(num, 18); // Adjust precision as needed
    return BigDecimal.fromString(numStr);
}

function toFixed(num: f64, decimals: i32): string {
    let factor = Math.pow(10, decimals) as f64;
    let rounded = Math.round(num * factor) / factor;
    let result = rounded.toString();
    let dotIndex = result.indexOf(".");
    if (dotIndex == -1) {
        result += ".";
        dotIndex = result.length - 1;
    }
    let decimalPartLength = result.length - dotIndex - 1;
    for (let i = 0; i < decimals - decimalPartLength; i++) {
        result += "0";
    }
    return result;
}

function findPay(pays: Array<Pay>, key: string): Pay | null {
    for (let i = 0; i < pays.length; i++) {
      let vpay = pays[i];
      if (`${vpay.trxId}-${vpay.actionIndex}` == key) {
        return vpay;
      }
    }
    return null;
}

function timestampToString(timestamp: Timestamp): string {
    const milliseconds = timestamp.seconds * 1000 + timestamp.nanos / 1000000;
    const date = new Date(milliseconds);
    return date.toISOString();
}

export function handlePays(bytes: Uint8Array): void {
    const paysProto: Pays = Protobuf.decode<Pays>(bytes, Pays.decode);
    const pays = paysProto.vpays.concat(paysProto.bpays);

    if (pays.length == 0) {
        log.info("No pays found", []);
        return;
    }

    for (let i=0; i<pays.length; i++) {

        const pay = pays[i];
        const key = `${pay.trxId}-${pay.actionIndex}`;

        const payEntity = new PayEntity(key);
        payEntity.bp = pay.bp;
        payEntity.transaction_id = pay.trxId;
        payEntity.type = findPay(paysProto.vpays, key) ? "VPAY" : "VPAY";
        payEntity.quantity = pay.quantity;
        payEntity.value = toBigDecimal(pay.value);
        payEntity.block_num = BigInt.fromU64(pay.blockNum);
        payEntity.timestamp = timestampToString(pay.timestamp!);
        payEntity.save();

        let bpEntity = BpEntity.load(pay.bp)
        if(!bpEntity) {
            bpEntity = new BpEntity(pay.bp);
            bpEntity.name = pay.bp;
            bpEntity.paid_value = toBigDecimal(pay.value);
            bpEntity.paid_quantity = pay.quantity;
            bpEntity.paid_count = 1;
        }
        else {
            bpEntity.paid_value += toBigDecimal(pay.value);
            bpEntity.paid_count += 1;
            bpEntity.paid_quantity = addAssets(bpEntity.paid_quantity, pay.quantity);
        }
        bpEntity.save();
    }
}