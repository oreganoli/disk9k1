var m = require("mithril");
var CurrentUser = require("../models/current_user");
var LoginButton = require("./login_button");
var LogoutButton = require("./logout_button");
var LoginIndicator = {
    view: function () {
        if (CurrentUser.user != null) {
            return m("b", ["Logged in as ", m("a", {href: "/me"}, CurrentUser.user.name), m(LogoutButton)]);
        } else {
            return m(LoginButton);
        }
    }
};
module.exports = LoginIndicator;