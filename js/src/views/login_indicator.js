const m = require("mithril");
const CurrentUser = require("../models/current_user");
const LoginButton = require("../controllers/login_button");
const LogoutButton = require("../controllers/logout_button");
const LoginIndicator = {
    view: function () {
        if (CurrentUser.user != null) {
            if (CurrentUser.user.is_admin) {
                return m(".top_item", [m("a", {href: "/#!/me"}, CurrentUser.user.name),
                    m("a.top_item", {href: "/#!/panel"}, "Global settings"),
                    m(LogoutButton)]);
            }
            return m("b.top_item", ["Logged in as ", m("a", {href: "/me"}, CurrentUser.user.name), m(LogoutButton)]);
        } else {
            return m(LoginButton);
        }
    }
};
module.exports = LoginIndicator;