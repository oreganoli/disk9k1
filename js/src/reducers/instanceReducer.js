export default function instance(state, action) {
    if (typeof state === 'undefined') {
        return {
            data: {
                name: "Placeholder, loading...",
                description: "Placeholder, loading...",
                version: "placeholder, loading...",
                size_limit: 0
            }
        };
    }
    switch (action.type) {
        case 'LOAD_INSTANCE':
            return {
                ...state,
                data: action.payload
            };
        default:
            return state;
    }
}

export async function loadInstance() {
    let request = new Request('/instance',
        {method: 'GET'});
    let response = await fetch(request);
    let inst = (await response.json());
    console.log(inst);
    return inst;
}

function defaultInstance() {
    return {
        name: "Placeholder, loading...",
        description: "Placeholder, loading...",
        version: "placeholder, loading...",
        size_limit: 0
    };
}