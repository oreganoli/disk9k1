const m = require("mithril");
const DirectoryView = require("./views/directory");
const InstanceView = require("./views/instance");
const LoginView = require("./views/login");
const PanelView = require("./views/panel");
const MeView = require("./views/me");
const TopBar = require("./views/top_bar");

const Layout = {
    view: (vnode) => {
        return m("main", [m(TopBar), m(".main_container", vnode.children)]);
    }
};

m.route(document.body, "/index", {
    "/index": {render: () => (m(Layout, m(InstanceView)))},
    "/login": {render: () => (m(Layout, m(LoginView)))},
    "/panel": {render: () => (m(Layout, m(PanelView)))},
    "/me": {render: () => (m(Layout, m(MeView)))},
    "/drive": {render: () => (m(Layout, m(DirectoryView)))},
    "/drive/:id": {render: () => (m(Layout, m(DirectoryView)))}
});