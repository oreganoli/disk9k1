var m = require("mithril");

var LoginButton = {
    view: function () {
        return m("button.top_item", {
            onclick: function () {
                m.route.set("/login")
            }
        }, "Sign in");
    }
};
module.exports = LoginButton;