//User State
pub fn is_logged_in(is_true: i32) -> bool {
    //TODO: once Session is working, check if active session, THEN return true
    //This is currently only for testing purposes
    if is_true == 0 {
        return false;
    } else if is_true == 1 {
        return true;
    } else {
        return false;
    }
} 