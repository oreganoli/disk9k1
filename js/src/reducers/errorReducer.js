export default function error(state, action) {
    if (typeof state === "undefined") {
        return {
            name: "Placeholder error for the meantime"
        };
    }
    if (action.type === "SET_ERROR") {
        return action.payload;
    }
    return state;
}