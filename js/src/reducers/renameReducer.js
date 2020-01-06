const rename = (state, action) => {
    if (typeof state == 'undefined') {
        return null;
    } else {
        if (action.type === "SET_RN_ITEM") {
            return action.payload;
        } else {
            return state;
        }
    }
};

export default rename;