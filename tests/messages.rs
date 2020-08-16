use announcer::messages::{load_config, MessagesConfig};

#[test]
fn does_file_exist() {
  let result = load_config("wrong/path.json");
  assert_eq!(result.is_err(), true);
}

#[test]
fn file_exists() {
  let result = load_config("message_config_test_data.json");
  
  let expected = MessagesConfig {};
  assert_eq!(result, expected);
}
