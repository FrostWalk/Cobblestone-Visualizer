"use strict";
document.getElementById('login-form').addEventListener('submit', function (event) {
    event.preventDefault();
    var username = document.getElementById('username').value;
    var password = document.getElementById('password').value;
    // Handle login logic here
    console.log("Username: ".concat(username, ", Password: ").concat(password));
    // Example of form validation or submission
    if (username && password) {
        alert('Login successful');
    }
    else {
        alert('Please enter both username and password');
    }
});
