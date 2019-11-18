var m = require("mithril");
var CurrentUser = {
    user: null,
    authenticate: function (username, password) {
        m.request({
            method: "POST",
            url: "/authenticate",
            withCredentials: true,
            body: {username, password}
        }).then(function (result) {
            CurrentUser.user = result;
        });
    }
};
module.exports = CurrentUser;