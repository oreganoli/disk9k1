const m = require("mithril");
const CurrentUser = require("../models/current_user");

const MeView = {
    view: () => (
        m(".user_profile", [
            m("h1", "Your profile"),
            m("table", [
                m("tr", [m("th.hor", "ID"), m("td", CurrentUser.user.id)]),
                m("tr", [m("th.hor", "Username"), m("td", CurrentUser.user.name)]),
                m("tr", [m("th.hor", "Email"), m("td", m("a", {href: `mailto:${CurrentUser.user.email}`}, CurrentUser.user.email))]),
                m("tr", [m("th.hor", "Joined"), m("td", CurrentUser.user.joined)]),
                m("tr", [m("th.hor", "Role"), m("td", CurrentUser.user.is_admin ? "Admin" : "User")])])
            ])
    )
};
module.exports = MeView;