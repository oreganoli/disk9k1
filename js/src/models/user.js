import {store} from "../index";

export async function loadMe() {
    let request = new Request("/me", {method: "GET"});
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        let json = await response.json();
        console.log("Got user profile.");
        console.log(json);
        return json;
    } else {
        return null;
    }
}

export async function logOut() {
    let request = new Request("/logout", {method: "POST"});
    await fetch(request).catch((err) => alert(err));
}

export async function signIn({username, password}) {
    let request = new Request("/login", {
        method: "POST",
        body: JSON.stringify({username: username, password: password})
    });
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        return true;
    } else {
        store.dispatch({type: "SET_ERROR", payload: await response.json()});
        return false;
    }
}

export async function putPassword({password, passCon}) {
    let request = new Request("/change_password", {
        method: "PUT",
        body: JSON.stringify({"new": password, con: passCon})
    });
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        return true;
    } else {
        store.dispatch({type: "SET_ERROR", payload: await response.json()});
        return false;
    }
}

export async function putUsername({username}) {
    let request = new Request("/change_username", {
        method: "PUT",
        body: JSON.stringify({"new": username})
    });
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        await response.text();
        return true;
    } else {
        store.dispatch({type: "SET_ERROR", payload: await response.json()});
        return false;
    }
}

