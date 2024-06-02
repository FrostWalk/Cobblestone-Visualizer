document.getElementById('login-form')!.addEventListener('submit', function(event) {
    event.preventDefault();

    const username = (document.getElementById('username') as HTMLInputElement).value;
    const password = (document.getElementById('password') as HTMLInputElement).value;

    // Handle login logic here
    console.log(`Username: ${username}, Password: ${password}`);

    // Example of form validation or submission
    if (username && password) {
        alert('Login successful');
    } else {
        alert('Please enter both username and password');
    }
});
