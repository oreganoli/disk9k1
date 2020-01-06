import React from 'react';
import {connect} from 'react-redux';

class ErrorWindow extends React.Component {
    render() {
        if (this.props.error != null) {
            return <div className={"modal_overlay"}/>;
        } else {
            return <div/>
        }
    }
}

const mapStateToProps = (state) => ({
    error: state.error
});
const mapDispatchToProps = (dispatch) => ({});
export default connect(mapStateToProps, mapDispatchToProps)(ErrorWindow);
