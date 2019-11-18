var m = require("mithril");
var CurrentUser = require("../models/current_user");
var LoginButton = require("./login_button");
var LogoutButton = require("./logout_button");
var LoginIndicator = {
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