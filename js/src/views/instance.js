var m = require("mithril");
var Instance = require("../models/instance");
var TopBar = require("./top_bar");
var InstanceView = {
    oninit: function () {
        Instance.load();
    },
    view: function () {
        let name_header = m("h1", {class: "title"}, Instance.name);
        let description = m("p", {class: "description"}, Instance.description);
        let size_limit = m("p", {class: "size_limit"}, `The size limit is ${Instance.size_limit / 1048576} MiB.`);
        return m("main", {class: "main_container"}, [m(TopBar), name_header, description, size_limit]);
    }
};
module.exports = InstanceView;