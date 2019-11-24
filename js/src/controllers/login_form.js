const m = require("mithril");
const CurrentUser = require("../models/current_user");

const LoginForm = {
    username: "",
    password: "",
    view: function () {
        return m("form.login_form", {
                onsubmit: function (e) {
                    e.preventDefault();
                    CurrentUser.authenticate(LoginForm.username, LoginForm.password);
                }
            },
            [
                m("h1.title", "Sign in"),
                m("label", "Username"),
                m("input[type=text]", {
                    oninput: function (e) {
                        LoginForm.username = e.target.value;
                    }
                }),
                m("label", "Password"),
                m("input[type=password]", {
                    oninput: function (e) {
                        LoginForm.password = e.target.value;
                    }
                }),
                m("button", "Sign in")
            ]);
    }
};
module.exports = LoginForm;