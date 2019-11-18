var m = require("mithril");
var InstanceView = require("./views/instance");
var LoginView = require("./views/login");

m.route(document.body, "/index", {
    "/index": InstanceView,
    "/login": LoginView
});