import React from 'react';
import {useState} from 'react';
import {Link} from "react-router-dom";

const Register = () => {
    let [registered, setRegistered] = useState(false);
    let [user, setUser] = useState({
        name: "",
        email: "",
        password: "",
        pass_con: ""
    });
    if (registered) {
        return <div>
            <h1>Registration successful</h1>
            <p>You can now <Link to={"/login"}><strong> sign in.</strong></Link></p>
        </div>;
    } else {
        return <div>
            <h1>Register</h1>
            <form>
                <label>Username</label>
                <input type={"text"}/>
                <label>Email address</label>
                <input type={"email"}/>
                <label>Password</label>
                <input type={"password"}/>
                <label>Confirm password</label>
                <input type={"password"}/>
                <button className={"centeredButton"}>Register!</button>
            </form>
        </div>;
    }
};

export default Register;