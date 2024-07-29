import { BigInt, log } from "@graphprotocol/graph-ts";
import { Bps } from "./pb/antelope/bps/v1/Bps";
import { Pay } from "./pb/antelope/bps/v1/Pay";
import { Pay as PayEntity, Bp as BpEntity} from "../generated/schema";
import { Protobuf } from 'as-proto/assembly';
import { addAssets, timestampToString, toBigDecimal } from "./utils";


function findPay(pays: Array<Pay>, key: string): Pay | null {
    for (let i = 0; i < pays.length; i++) {
      let vpay = pays[i];
      if (`${vpay.trxId}-${vpay.actionIndex}` == key) {
        return vpay;
      }
    }
    return null;
}


export function handleBps(bytes: Uint8Array): void {
    const bpsProto: Bps = Protobuf.decode<Bps>(bytes, Bps.decode);
    const pays = bpsProto.vpays.concat(bpsProto.bpays);

    for (let i=0; i<bpsProto.regs.length; i++){
        const reg = bpsProto.regs[i];
        let bpEntity = BpEntity.load(reg.bp);
        if(!bpEntity) {
            bpEntity = new BpEntity(reg.bp);
            bpEntity.paidValue = toBigDecimal(0);
            bpEntity.paidCount = 0;
            bpEntity.updateCount = 0;
        }
        else {
            bpEntity.prevUpdateBlockNum = bpEntity.updateBlockNum;
        }
        bpEntity.name = reg.bp;
        bpEntity.url = reg.url;
        bpEntity.location = reg.location;
        bpEntity.publicKey = reg.publicKey;
        bpEntity.updateBlockNum = BigInt.fromU64(reg.blockNum);
        bpEntity.updateTimestamp = timestampToString(reg.timestamp!);
        bpEntity.updateCount += 1;
        bpEntity.save();
    }

    for (let i=0; i<pays.length; i++) {

        const pay = pays[i];
        const key = `${pay.trxId}-${pay.actionIndex}`;

        const payEntity = new PayEntity(key);
        payEntity.bp = pay.bp;
        payEntity.transactionId = pay.trxId;
        payEntity.type = findPay(bpsProto.vpays, key) ? "VPAY" : "VPAY";
        payEntity.quantity = pay.quantity;
        payEntity.value = toBigDecimal(pay.value);
        payEntity.blockNum = BigInt.fromU64(pay.blockNum);
        payEntity.timestamp = timestampToString(pay.timestamp!);
        payEntity.save();

        let bpEntity = BpEntity.load(pay.bp)
        if(!bpEntity) {
            bpEntity = new BpEntity(pay.bp);
        }
        if(bpEntity.paidCount == 0) {
            bpEntity.paidValue = toBigDecimal(pay.value);
            bpEntity.paidQuantity = pay.quantity;
            bpEntity.paidCount = 1;
        }
        else {
            bpEntity.paidValue + toBigDecimal(pay.value);
            bpEntity.paidCount += 1;
            bpEntity.paidQuantity = addAssets(bpEntity.paidQuantity!, pay.quantity);
        }
        bpEntity.save();
    }
}