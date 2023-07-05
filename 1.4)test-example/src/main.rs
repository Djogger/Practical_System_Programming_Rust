use std::process;

fn main() {
    println!("{}", get_process_id());
}

fn get_process_id() -> u32 {
    process::id()
}

// 1)Тест:

//#[test]
//fn test_if_process_id_is_returned() {
//    assert_ne!(get_process_id(), 0, "There is error in code!");
//}

// 2)Тест, но который будет экономить время и отделять наш код от кодов теста, крч, чёт типо того,
// это чисто круче:))

#[cfg(test)]
mod tests {
    use crate::get_process_id;

    #[test]
    fn test_if_process_id_is_returned() {
        assert_ne!(get_process_id(), 0, "There is error in code!");
    }
}










