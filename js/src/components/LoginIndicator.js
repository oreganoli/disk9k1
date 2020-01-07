import React from 'react';
import LoginButton from './LoginButton';
import {LogoutButton} from "./LogoutButton";
import {connect} from "react-redux";
import {Link} from "react-router-dom";

const PanelLink = (is_admin) => {
    if (is_admin) {
        return <Link to={"/panel"}>🔧 <strong>Global settings</strong></Link>;
    } else {
        return null;
    }
};

class LoginIndicator extends React.Component {
    render() {
        if (this.props.user == null) {
            return <LoginButton/>;
        } else {
            return <span>
                {PanelLink(this.props.user.is_admin)}
                <span className={"top_item"}><Link to={"/drive"}><b>My files</b></Link></span>
                    <span className={"top_item"}>{"Logged in as "}<strong>
                        <Link to={"/me"}>{this.props.user.name.concat(this.props.user.is_admin ? " 👑" : "")}</Link>
                    </strong>
                    <LogoutButton/>
                </span>
            </span>;
        }
    }
}

const mapStateToProps = (state) => ({
    user: state.user
});
const mapDispatchToProps = (dispatch) => ({});

export default connect(mapStateToProps, mapDispatchToProps)(LoginIndicator);

