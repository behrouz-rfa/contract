use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // SETUP CONTRACT STATE
    password_solution: String
}

#[near_bindgen]

impl Contract {
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            password_solution:solution,
        }
    }

    pub fn get_solution(&self) -> String {
        self.password_solution.clone()
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);
        // what do we need to modify in the logic below
        // to reflect the hash input logic we've created
        if hashed_input_hex == self.password_solution {
            env::log_str("You may enter! This is the right password");
            true
        }  else {
            env::log_str("You shall not pass. Please try again.");
            false
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // debugging and iteration of a unit test

    // TESTS HERE
    #[test]
    fn debug_get_hash() {

        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "tamago clarian";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
    // write a function that will guess the solution
    // get an account ID
    // set up a testing context and test environment
    // set up the contract for the test - call a new method

    fn check_guess_solution() {
        let clarian = AccountId::new_unchecked("clarian.testnet".to_string());
        let context = get_context(clarian);
        testing_env!(context.build());

        let mut contract = Contract::new("e9242595b2be15297c7a9b0433e22c3d95a2c013b0635ec0a37a14b3f9e8146c".to_string(),);

        let mut guess_result = contract.guess_solution("wrong answer".to_string());
        assert!(!guess_result, "Expectation: This is incorrect");
        assert_eq!(get_logs(), ["You shall not pass. Please try again."], "Expected a failure in logs");
        // set the guess_result variable to the correct password and return
        // the assertion so that it checks guess result with the expected string which
        // should say that it passes
        guess_result = contract.guess_solution("tamago clarian".to_string());
        assert!(guess_result, "Expectation: This is the correct answer");
        assert_eq!(get_logs(), ["You shall not pass. Please try again.", "You may enter! This is the right password"], "Expected a successful log
        after previous failed");
    }

    // 1. Compile the updated smart contract
    // 2. clean the state of our subaccount
    // 3. deploy the wasm file

}