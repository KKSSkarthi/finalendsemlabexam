```javascript
document.getElementById("signupForm").addEventListener("submit", function(event) {

    event.preventDefault();

    let username = document.getElementById("username").value;
    let email = document.getElementById("email").value;
    let password = document.getElementById("password").value;

    let message = document.getElementById("message");

    if (username === "" || email === "" || password === "") {
        message.style.color = "red";
        message.innerHTML = "Please fill all fields!";
    }

    else if (password.length < 6) {
        message.style.color = "red";
        message.innerHTML = "Password must contain at least 6 characters!";
    }

    else {
        let user = {
            username: username,
            email: email,
            password: password
        };

        localStorage.setItem("user", JSON.stringify(user));

        message.style.color = "green";
        message.innerHTML = "Signup Successful! Welcome " + username;

        document.getElementById("signupForm").reset();
    }

});
```