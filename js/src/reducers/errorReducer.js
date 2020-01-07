export default function error(state, action) {
    if (typeof state === "undefined") {
        return null;
    }
    if (action.type === "SET_ERROR") {
        return action.payload;
    }
    return state;
}