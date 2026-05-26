#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_sarifi_loan_and_repayment() {
    let env = Env::default();

    // Generate dummy addresses for testing
    let store_owner = Address::generate(&env);
    let supplier = Address::generate(&env);
    let loan_amount = 5000; // 5,000 PHP equivalent in tokens

    // 1. Test Loan Creation
    let mut loan = SariFiContract::create_loan(
        env.clone(), 
        store_owner.clone(), 
        supplier.clone(), 
        loan_amount
    );

    assert_eq!(loan.loan_amount, 5000);
    assert_eq!(loan.amount_repaid, 0);
    assert_eq!(loan.active, true);

    // 2. Test a Micro-Installment Payment of 2,500 PHP
    loan = SariFiContract::pay_installment(env.clone(), loan, 2500);
    assert_eq!(loan.amount_repaid, 2500);
    assert_eq!(loan.active, true);

    // 3. Test Final Installment Payment to clear loan
    loan = SariFiContract::pay_installment(env.clone(), loan, 2500);
    assert_eq!(loan.active, false); // Active status should be false (closed)
}