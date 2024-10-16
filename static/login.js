document.addEventListener("DOMContentLoaded", function() {
    const usernameInput = document.getElementById("username");
    const passwordInput = document.getElementById("password");
    const togglePassword = document.getElementById("toggle-password");

    usernameInput.value = "Username";
    passwordInput.value = "Password";

    usernameInput.addEventListener("focus", function() {
        if (usernameInput.value === "Username") {
            usernameInput.value = "";
        }
    });

    usernameInput.addEventListener("blur", function() {
        if (usernameInput.value === "") {
            usernameInput.value = "Username";
        }
    });

    passwordInput.addEventListener("focus", function() {
        if (passwordInput.value === "Password") {
            passwordInput.value = "";
        }
    });

    passwordInput.addEventListener("blur", function() {
        if (passwordInput.value === "") {
            passwordInput.value = "Password";
        }
    });

    // Alternar a visibilidade da senha
    togglePassword.addEventListener("click", function() {
        const type = passwordInput.getAttribute("type") === "password" ? "text" : "password";
        passwordInput.setAttribute("type", type);
        this.src = type === "password" ? "https://i.imgur.com/vBYUojn.png" : "https://i.imgur.com/WrBbapR.png";
    });
});

