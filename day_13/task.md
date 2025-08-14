# Task — Day 13

## Goal
Build a mini wallet + account management system using `HashMap<K, V>` with clean modular architecture.

## Requirements:
1. **Structs**
   - `User { name: String, age: u32 }`
   - `Account { name: String, balance: i32 }`

2. **Functions**
   - `add_user(user, &mut user_ages)`: Insert a user into a `HashMap<String, u32>`.
   - `print_age(name, &user_ages)`: Print user’s age if found, otherwise print "not found".
   - `add_account(account, &mut user_balances)`: Insert account into a `HashMap<String, i32>`.
   - `add_balance(name, &mut user_balances, delta)`: Increase account balance by `delta`.
   - `count_words(words)`: Return a `HashMap<String, u32>` with the frequency of each word.

3. **Demo Scenario**
   - Create a few users and add them to the `user_ages` map.
   - Print the age of a specific user.
   - Create a few accounts and add them to the `user_balances` map.
   - Update one account’s balance.
   - Count word frequencies from a list and print the result.

4. **Constraints**
   - Use `entry()` API where applicable.
   - No `unwrap()` or `expect()` in production code.
   - All structs and functions should be logically separated and reusable.

