var m = require("mithril");
var CurrentUser = require("../models/current_user");
var LoginButton = require("./login_button");
var LoginIndicator = {
    view: function () {
        if (CurrentUser.user != null) {
            return m("b", ["Logged in as ", m("a", {href: "/me"}, CurrentUser.user.name)]);
        } else {
            return m(LoginButton);
        }
    }
};
module.exports = LoginIndicator;