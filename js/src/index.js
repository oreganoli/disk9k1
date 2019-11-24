const m = require("mithril");
const InstanceView = require("./views/instance");
const LoginView = require("./views/login");
const PanelView = require("./views/panel");
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