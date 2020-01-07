import React from 'react';
import {connect} from "react-redux";

class Home extends React.Component {
    render() {
        let mebibytes = this.props.instance.size_limit / 1048576;
        mebibytes -= mebibytes % 0.01;
        return <div className={"central"}>
            <h1>{this.props.instance.name}</h1>
            <p>{this.props.instance.description}</p>
            <p>{`The size limit is ${mebibytes} MiB.`}</p>
        </div>;
    }
}

const mapStateToProps = (state) => ({
    instance: state.instance
});
const mapDispatchToProps = (dispatch) => ({});

export default connect(mapStateToProps, mapDispatchToProps)(Home);
