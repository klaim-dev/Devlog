## 🧠 Super Task — Online Event Access System (Level 3, Day 2)

### 🎯 Description

You are building an access system for an online event. Each user has the following data:

```rust
let age: u32 = 21;
let has_ticket: bool = true;
let is_banned: bool = false;
let wants_recording: bool = true;

📋 Access Rules
If the user is under 18 → "Denied: age restriction"

If the user is banned → "Denied: banned user"

If the user has no ticket but wants the recording → "Denied: ticket required for recording"

If the user has no ticket at all → "Denied: no ticket"

Otherwise → "Access granted"

           🛠 Requirements
Use only:
if, else
bool, let, &str
The logic must be nested, not flat
Store the final decision in:

let status: &str = ...;
Then print the result using:
println!("Access result: {}", status);

      📦 Example Input

let age = 17;
let has_ticket = true;
let is_banned = false;
let wants_recording = false;

// → "Denied: age restriction"
🔍 Why This Is Hard
Multiple conditions overlap:
!has_ticket && wants_recording
just !has_ticket

== You must respect priority order == :
1.Age check
2.Ban check
3.Recording without ticket
4.No ticket at all
You must avoid duplicate logic (like repeating !has_ticket)
This is a logic tree — not a flat list of if statements
Think like a backend system: fail fast, keep logic clean
