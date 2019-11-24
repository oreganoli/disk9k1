const m = require("mithril");
const CurrentUser = {
    user: null,
    logout: function () {
        m.request(
            {method: "POST", url: "/logout"}
        ).then(function () {
            CurrentUser.user = null;
        });
    },
    me: function () {
        return m.request({
                method: "GET",
                url: "/me",
                withCredentials: true,
            }
        ).then(function (me) {
            CurrentUser.user = me;
        });
    },
    authenticate: function (username, password) {
        m.request({
            method: "POST",
            url: "/authenticate",
            withCredentials: true,
            body: {username, password},
        }).then(function (result) {
            if (result) {
                CurrentUser.me();
                m.route.set("/index");
            }
        })
            .catch((err) => {
                alert(err);
            });
    }
};
module.exports = CurrentUser;