var m = require("mithril");

var LoginButton = {
    view: function () {
        return m("button.login_button", {
            onclick: function () {
                m.route.set("/login")
            }
        }, "Sign in");
    }
};
module.exports = LoginButton;