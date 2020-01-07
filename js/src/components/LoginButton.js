import React from 'react';
import {useHistory} from "react-router";


function LoginButton() {
    const hist = useHistory();
    return <button onClick={() => {
        hist.push("/login");
    }}>Sign in</button>;
}

export default LoginButton;