# DevLog — Day 13

Today was a tough but productive session.  
The main focus was on mastering `HashMap<K, V>` and building a realistic architecture that ties together multiple collections.  

### Key Achievements:
- Implemented separate structs for `User` and `Account` to represent domain data.
- Created functions for:
  - Adding users to a HashMap.
  - Printing user age if available.
  - Adding accounts with initial balances.
  - Updating account balances.
  - Counting word frequencies from a collection.
- Practiced `entry()` API for both insertion and mutation.
- Strengthened pattern matching skills with `if let Some(...)` syntax.

### Lessons Learned:
- `entry().or_insert()` is essential for concise initialization in `HashMap`.
- Always keep function responsibilities focused — one function per clear purpose.
- Variable naming matters for readability.
- Even simple logic benefits from structuring the code into reusable functions.

### Reflection:
While the architecture felt solid, the business logic complexity made the task take longer than expected.  
Next steps will focus on keeping architecture production-like, but simplifying logic to ensure the day’s topic remains the main learning target.

