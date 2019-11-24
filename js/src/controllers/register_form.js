const m = require("mithril");

const RegisterForm = {
    data: {
        username: "",
        email: "",
        password: "",
        password_con: ""
    },
    send: () => {
        m.request({
            method: "POST",
            url: "/register",
            body: RegisterForm.data
        }).then(() => {
                alert("Registration successful. You can now log in.");
                m.route.set("/login");
            }
        ).catch((err) => (alert(err)));
    },
    view: () => (
        m("div", m("form", {
                onsubmit: (e) => {
                    e.preventDefault();
                    RegisterForm.send();
                }
            }, [
                m("h1", "Register"),
                m("label", "Username"),
                m("input[type=text]", {
                    oninput: (e) => {
                        RegisterForm.data.username = e.target.value;
                    }
                }),
                m("label", "Email"),
                m("input[type=email]", {
                    oninput: (e) => {
                        RegisterForm.data.email = e.target.value;
                    }
                }),
                m("label", "Password"),
                m("input[type=password]", {
                    oninput: (e) => {
                        RegisterForm.data.password = e.target.value;
                    }
                }),
                m("label", "Confirm password"),
                m("input[type=password]", {
                    oninput: (e) => {
                        RegisterForm.data.password_con = e.target.value;
                    }
                }),
                m("button[type=submit]", "Register"),
            ])
        )
    )
};

module.exports = RegisterForm;