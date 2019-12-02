const m = require("mithril");
const CurrentDirectory = require("../models/current_directory");
const DirectoryView = {
    oninit: () => {
    },
    onupdate: () => {
        CurrentDirectory.load(m.route.param("id"));
    },
    view: () => (
        m("div", [
            m("h1", CurrentDirectory.info == null ? "Your drive" : CurrentDirectory.info.name),
            m("table", [
                CurrentDirectory.info == null ? m("tr", m("td", m("i", "reached top of file system"))) : m("tr", m("td", m("a", {
                    href: CurrentDirectory.info.parent == null ? "#!/drive" : `#!/drive/${CurrentDirectory.info.parent}`
                }, `📁 ../`))),
                CurrentDirectory.directories.map((x) => (m("tr", m("td", m("a", {href: `#!/drive/${x.id}`}, `📁 ${x.name}/`)))))
            ])])
    )
};
module.exports = DirectoryView;