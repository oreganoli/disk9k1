import React from 'react';
import {connect} from 'react-redux';
import {logOut} from "../models/user";

class LogoutButton extends React.Component {
    render() {
        return <button className={"top_item"} onClick={() => {
            logOut().then(this.props.dispatchLogout);
        }
        }>Log out</button>
    }
}

const mapStateToProps = () => ({});
const logoutAction = {type: "LOGOUT"};
const mapDispatchToProps = (dispatch) => ({
    dispatchLogout: () => dispatch(logoutAction)
});

export default connect(mapStateToProps, mapDispatchToProps)(LogoutButton);