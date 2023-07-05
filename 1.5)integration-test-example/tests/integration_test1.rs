use integration_test_example;

#[test]
fn files_test1() {
    assert_ne!(integration_test_example::get_process_id(), 0, "Error in code");
}

#[test]
#[ignore]    /* 1) Чтобы протестить только заигноренные файлы, 
                нужно использовать эту команду: cargo test -- --ignored */
             /* 2) Чтобы упорядочить тесты, как в этом файле, есть такая команда:
                cargo test -- --test-threads=1 */
fn files_test2() {
    assert_eq!(1+4, 5);
}

#[test]
fn process_test1() {
    assert!(true);
}