const reloadDir = (state, action) => {
    if (typeof state == 'undefined') {
        return false;
    }
    switch (action.type) {
        case "SET_RELOAD_DIR":
            return action.payload;
        default:
            return state;
    }
};
export default reloadDir;