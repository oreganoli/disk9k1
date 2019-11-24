const m = require("mithril");

const LoginButton = {
    view: () => {
        return m("button.top_item", {
            onclick: () => {
                m.route.set("/login")
            }
        }, "Sign in");
    }
};
module.exports = LoginButton;