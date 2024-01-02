use bank::SavingsAccount;

mod common;

#[test]
fn should_have_a_starting_balance_of_0() {
    let account = SavingsAccount::new();
    assert_eq!(account.get_balance(), 0);

    let mut x = String::from("asdasd");

    let y = &x;
    let z = &mut x;
    println!("{z}");
}

#[test]
fn should_be_able_to_deposit() {
    let mut account = SavingsAccount::new();
    account.deposit(100);
    assert_eq!(account.get_balance(), 100, "Balance should be 100!");
}

#[test]
#[should_panic]
fn should_panic_if_deposit_is_negative() {
    let mut account = SavingsAccount::new();
    account.deposit(-1);
}

#[test]
fn should_transfer_money() -> Result<(), String> {
    let mut account = SavingsAccount::new();
    account.deposit(100);
    account.transfer(123456, 100)?;
    Ok(())
}
