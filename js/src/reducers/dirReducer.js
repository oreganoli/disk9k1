export default function dir(state, action) {
    if (typeof state === 'undefined') {
        return null;
    }
    switch (action.type) {
        case 'SET_DIR':
            return action.payload;
        default:
            return state;
    }
}