import React from 'react';
import {Link} from "react-router-dom";
import {connect} from "react-redux";
import {loadInstance} from "../reducers/instanceReducer";
import {loadMe} from "../models/user";
import LoginIndicator from "./LoginIndicator";

class TopBar extends React.Component {
    componentDidMount() {
        loadInstance().then(
            (result) => {
                console.log(result);
                this.props.loadInstance(result);
            }
        );
        loadMe().then(
            (result) => {
                console.log(result);
                this.props.loadMe(result);
            }
        )
    }

    render() {
        return <nav>
            <div className={"left_half"}>
                <Link to="/"><strong>{this.props.instance.name}</strong></Link>
            </div>
            <div className={"right_half"}>
                <LoginIndicator/>
            </div>
        </nav>;
    }
}

const mapStateToProps = (state) => (
    {instance: state.instance}
);
const instanceLoadAct = (instance) => ({type: "LOAD_INSTANCE", payload: instance});
const meLoadAct = (user) => ({type: "SET_USER", payload: user});
const mapDispatchToProps = (dispatch) => ({
    loadInstance: (instance) => dispatch(instanceLoadAct(instance)),
    loadMe: (user) => dispatch(meLoadAct(user))
});

export default connect(mapStateToProps, mapDispatchToProps)(TopBar);