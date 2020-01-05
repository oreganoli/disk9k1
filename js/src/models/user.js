export async function loadMe() {
    let request = new Request("/me", {method: "GET"});
    let response = await fetch(request).catch((err) => alert(err));
    if (response.status === 200) {
        return await response.json();
    } else {
        return null;
    }
}