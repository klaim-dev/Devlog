fn main() {
    let age: u32 = 21;
    let has_ticket: bool = false;
    let is_banned: bool = false;
    let wants_recording: bool = true;


    let status = if age > 18 {
        if is_banned == false {
            if has_ticket == true {
                "Access granted"
            } else if wants_recording {
               "Denied: ticket required for recording"
            } else {
                "Denied: no ticket"
            }
        } else {
            "Denied: banned user"
        }
    } else {
        "Denied: age restriction"
    };

    println!("{}", status);

}