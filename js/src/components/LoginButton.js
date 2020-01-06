import React from 'react';
import {Redirect} from "react-router";

class LoginButton extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            toLogin: false
        };
    }

    render() {
        if (!this.state.toLogin) {
            return <button onClick={() => {
                this.setState(() => ({toLogin: true}))
            }}>Sign in</button>;
        } else {
            return <div>
                <button>Sign in</button>
                <Redirect to={"/login"}/></div>;
        }
    }
}

export default LoginButton;