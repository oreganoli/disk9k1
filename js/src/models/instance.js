const m = require("mithril");

const Instance = {
    name: "",
    description: "",
    size_limit: 0,
    load: function () {
        m.request({
            method: "GET",
            url: "/instance",
            withCredentials: false
        }).then(function (result) {
            Instance.name = result.name;
            Instance.description = result.description;
            Instance.size_limit = result.size_limit;
        });
    }
};
module.exports = Instance;