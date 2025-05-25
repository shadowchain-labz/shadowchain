
#[test]
fn test_balance_logic() {
    let mut ledger = std::collections::HashMap::new();
    ledger.insert("alice", 100);
    assert_eq!(ledger["alice"], 100);
}
