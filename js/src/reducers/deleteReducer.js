const del = (state, action) => {
    if (typeof state == 'undefined') {
        return null;
    }
    if (action.type === "SET_DEL_ITEM") {
        return action.payload;
    } else {
        return state;
    }
};
export default del;