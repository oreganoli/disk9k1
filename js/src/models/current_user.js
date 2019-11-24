const m = require("mithril");
const CurrentUser = {
    user: null,
    logout: () => {
        m.request(
            {method: "POST", url: "/logout"}
        ).then(() => {
            CurrentUser.user = null;
            m.route.set("/index");
        }).catch((err) => (alert(err)));
    },
    me: () => {
        return m.request({
                method: "GET",
                url: "/me",
                withCredentials: true,
            }
        ).then((me) => {
            CurrentUser.user = me;
        }).catch((err) => (alert(err)));
    },
    authenticate: (username, password) => {
        m.request({
            method: "POST",
            url: "/authenticate",
            withCredentials: true,
            body: {username, password},
        }).then((result) => {
            if (result) {
                CurrentUser.me();
                m.route.set("/index");
            }
        }).catch((err) => (alert(err)))
    }
};
module.exports = CurrentUser;