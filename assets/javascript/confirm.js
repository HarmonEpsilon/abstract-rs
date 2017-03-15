function Validate() {
    var password = document.getElementById("password");
    var confirm = document.getElementById("confirm");

    if(password != confirm) {
        alert("Passwords do not match");
        return false;
    }

    return true;
}