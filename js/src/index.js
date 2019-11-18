var m = require("mithril");
var InstanceView = require("./views/instance");
var LoginView = require("./views/login");
var PanelView = require("./views/panel");

m.route(document.body, "/index", {
    "/index": InstanceView,
    "/login": LoginView,
    "/panel": PanelView
});