const m = require("mithril");
const CurrentUser = require("../models/current_user");

const LogoutButton = {
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