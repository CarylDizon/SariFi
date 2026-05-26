#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Address};

#[contracttype]
#[derive(Clone, Debug)]
pub struct LoanRecord {
    pub store_owner: Address,
    pub supplier: Address,
    pub loan_amount: i128,
    pub amount_repaid: i128,
    pub active: bool,
}

#[contract]
pub struct SariFiContract;

#[contractimpl]
impl SariFiContract {
    // Initializes a micro-loan record for inventory restocking
    pub fn create_loan(
        env: Env, 
        store_owner: Address, 
        supplier: Address, 
        amount: i128
    ) -> LoanRecord {
        
        LoanRecord {
            store_owner,
            supplier,
            loan_amount: amount,
            amount_repaid: 0,
            active: true,
        }
    }

    // Records a micro-payment installment made by the store owner
    pub fn pay_installment(env: Env, mut loan: LoanRecord, payment: i128) -> LoanRecord {
        loan.amount_repaid += payment;
        
        if loan.amount_repaid >= loan.loan_amount {
            loan.active = false; // Loan fully paid off!
        }
        
        loan
    }
}