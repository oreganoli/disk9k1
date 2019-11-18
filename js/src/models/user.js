var m = require("mithril");
var User = {
    load: function (id) {
        m.request({
            method: "GET",
            url: `/user/${id}`,
            withCredentials: false
        }).then(function (result) {
            return result;
        })
    }
};
module.exports = User;