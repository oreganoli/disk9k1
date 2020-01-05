import React from 'react';
import {Link} from "react-router-dom";
import {connect} from "react-redux";
import {loadInstance} from "../reducers/instanceReducer";
import LoginIndicator from "./LoginIndicator";

class TopBar extends React.Component {
    componentDidMount() {
        loadInstance().then(
            (result) => {
                console.log(result);
                this.props.load(result);
            }
        );
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
const loadActionCreator = (instance) => ({type: "LOAD_INSTANCE", payload: instance});
const mapDispatchToProps = (dispatch) => ({load: (instance) => dispatch(loadActionCreator(instance))});

export default connect(mapStateToProps, mapDispatchToProps)(TopBar);