var m = require("mithril");
var CurrentUser = require("../models/current_user");

var LogoutButton = {
    view: function () {
        return m("button", {
            onclick: function () {
                CurrentUser.logout();
            }
        }, "Log out");
    }
};
module.exports = LogoutButton;