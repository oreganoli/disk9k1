const upload = (state, action) => {
    if (typeof state == 'undefined') {
        return false;
    }
    if (action.type === "SET_UPLOAD") {
        return action.payload;
    } else {
        return state;
    }
};
export default upload;