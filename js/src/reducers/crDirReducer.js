const crDir = (state, action) => {
    if (typeof state == 'undefined') {
        return null;
    }
    if (action.type === "SET_CREATE_DIR") {
        return action.payload;
    } else {
        return state;
    }
};
export default crDir;