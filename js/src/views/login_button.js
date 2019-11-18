var m = require("mithril");

var LoginButton = {

    view: function () {
        return m("button.login_button", {
            onclick: function () {
                alert("Attempted to log in");
            }
        }, "Sign in");
    }
};
module.exports = LoginButton;