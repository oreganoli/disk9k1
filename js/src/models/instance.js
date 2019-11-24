const m = require("mithril");

const Instance = {
    name: "",
    description: "",
    size_limit: 0,
    load: () => {
        m.request({
            method: "GET",
            url: "/instance",
            withCredentials: false
        }).then((result) => {
            Instance.name = result.name;
            Instance.description = result.description;
            Instance.size_limit = result.size_limit;
        });
    },
    load_then: (func) => {
        m.request({
            method: "GET",
            url: "/instance",
            withCredentials: false
        }).then((result) => {
            Instance.name = result.name;
            Instance.description = result.description;
            Instance.size_limit = result.size_limit;
            func();
        }).catch((err) => alert(err));
    }
};
module.exports = Instance;