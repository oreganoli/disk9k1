var m = require("mithril");
var CurrentUser = require("../models/current_user");

var LogoutButton = {
    view: function () {
        return m("button.top_item", {
            onclick: function () {
                CurrentUser.logout();
                m.route.set("/index");
            }
        }, "Log out");
    }
};
module.exports = LogoutButton;