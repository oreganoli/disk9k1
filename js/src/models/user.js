import {store} from "../index";

export async function loadMe() {
    let request = new Request("/me", {method: "GET"});
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        return await response.json();
    } else {
        return null;
    }
}

export async function logOut() {
    let request = new Request("/logout", {method: "POST"});
    await fetch(request).catch((err) => alert(err));
}

export async function signIn({username, password}) {
    let request = new Request("/authenticate", {method: "POST", body: JSON.stringify({username: username, password: password})});
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        return true;
    } else {
        store.dispatch({type: "SET_ERROR", payload: await response.json()});
        return false;
    }
}