import React from 'react';
import LoginButton from './LoginButton';
import {connect} from "react-redux";
import {Link} from "react-router-dom";

class LoginIndicator extends React.Component {
    render() {
        if (this.props.user == null) {
            return <LoginButton/>;
        } else {
            return <p>{"Logged in as "}<strong><Link to={"/me"}>{this.props.user.name}</Link></strong></p>;
        }
    }
}

const mapStateToProps = (state) => ({
    user: state.user
});
const mapDispatchToProps = (dispatch) => ({});

export default connect(mapStateToProps, mapDispatchToProps)(LoginIndicator);

