import React from 'react';
import {connect} from "react-redux";
import {loadInstance} from "../reducers/instanceReducer";

class Home extends React.Component {
    componentDidMount() {
        loadInstance().then(
            (result) => {
                console.log(result);
                this.props.load(result);
            }
        )
    }

    render() {
        let mebibytes = this.props.instance.data.size_limit / 1048576;
        mebibytes -= mebibytes % 0.01;
        return <div className={"central"}>
            <h1>{this.props.instance.data.name}</h1>
            <p>{this.props.instance.data.description}</p>
            <p>{`The size limit is ${mebibytes} MiB.`}</p>
        </div>;
    }
}

const mapStateToProps = (state) => ({
    instance: state.instance
});
const loadActionCreator = (instance) => ({type: "LOAD_INSTANCE", payload: instance});
const mapDispatchToProps = (dispatch) => ({load: (instance) => dispatch(loadActionCreator(instance))});

const VisibleHome = connect(mapStateToProps, mapDispatchToProps)(Home);

export default VisibleHome;