import React from 'react';
import LoginButton from './LoginButton';
import {LogoutButton} from "./LogoutButton";
import {connect} from "react-redux";
import {Link} from "react-router-dom";

class LoginIndicator extends React.Component {
    render() {
        if (this.props.user == null) {
            return <LoginButton/>;
        } else {
            return <span className={"top_item"}>{"Logged in as "}<strong><Link
                to={"/me"}>{this.props.user.name.concat(this.props.user.is_admin ? " 👑" : "")}
            </Link></strong><LogoutButton/>
            </span>;
        }
    }
}

const mapStateToProps = (state) => ({
    user: state.user
});
const mapDispatchToProps = (dispatch) => ({});

export default connect(mapStateToProps, mapDispatchToProps)(LoginIndicator);

