//$(which true); dst=/var/tmp/sut; out=${dst}/$0.bin; 
//$(which mkdir) -p ${dst}; 
//$(which rustc) -o "${out}" 1>&2 "$0" && "${out}" "$@"; exit $?

struct _MasterCard {
    number: u8,
    verification: u8,
}

struct _Visa {
    number: u32,
}

struct _WesternUnion {
    name: String,
    verification: u8,
}

struct BitCredit {
    btcnumber: u32,
}

trait CreditCharge {
    fn charge_with_id(&self, id: u32) -> bool;
}

impl CreditCharge for _WesternUnion {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 3 == (self.verification + 1) as u32
    }
}

impl CreditCharge for BitCredit {
    fn charge_with_id(&self, id: u32) -> bool {
        id % 2 == self.btcnumber % 2
    }
}

fn transact<Q: CreditCharge>(card: Q) {
    let id = 4096;
    if card.charge_with_id(id) {
        println!("transaction succeeded!");
    } else {
        println!("Failed!");
    }
}

fn main() {
    let card = BitCredit { btcnumber: 1024};
    let code = 4096;
    if card.charge_with_id(code) {
        println!("Payment succeeded!");
    } else {
        println!("Failed!");
    }

    transact(card);
}