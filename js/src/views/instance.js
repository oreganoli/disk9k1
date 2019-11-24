const m = require("mithril");
const Instance = require("../models/instance");
const InstanceView = {
    oninit: () => {
        Instance.load();
    },
    view: () => {
        let name_header = m("h1", {class: "title"}, Instance.name);
        let description = m("p", {class: "description"}, Instance.description);
        let size_limit = m("p", {class: "size_limit"}, `The size limit is ${Instance.size_limit / 1048576} MiB.`);
        return m("div.main_container", [name_header, description, size_limit]);
    }
};
module.exports = InstanceView;