const m = require("mithril");
const User = {
    load: (id) => {
        m.request({
            method: "GET",
            url: `/user/${id}`,
            withCredentials: false
        }).then((result) => {
            return result;
        })
    }
};
module.exports = User;