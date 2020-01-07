export default function user(state, action) {
    if (typeof state === 'undefined') {
        return null;
    }
    switch (action.type) {
        case 'SET_USER':
            return action.payload;
        case 'LOGOUT':
            return null;
        default:
            return state;
    }
}

const nullUser = {
    id: null,
    name: null,
    email: null,
    is_admin: null,
};