var m = require("mithril");
var InstanceView = require("./views/instance");
var LoginView = require("./views/login");
var PanelView = require("./views/panel");
const TopBar = require("./views/top_bar");

const Layout = {
    view: (vnode) => {
        return m("main", [m(TopBar), m(".main_container", vnode.children)]);
    }
};

m.route(document.body, "/index", {
    "/index": {render: () => (m(Layout, m(InstanceView)))},
    "/login": {render: () => (m(Layout, m(LoginView)))},
    "/panel": {render: () => (m(Layout, m(PanelView)))}
});