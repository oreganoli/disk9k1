import React from 'react';
import {connect} from 'react-redux';

class ErrorWindow extends React.Component {
    render() {
        if (this.props.error != null) {
            return <div className={"overlay"}>
                <div className={"modal_overlay"}/>
                <div className={"modal_window"}>
                    <h1 className={"error_reason"}>Error</h1>
                    <h2 className={"error_reason"}>{this.props.error.name}</h2>
                    <button className={"central centeredButton"} onClick={() => {
                        this.props.acceptError();
                    }}>OK
                    </button>
                </div>
            </div>;
        } else {
            return <div/>
        }
    }
}

const mapStateToProps = (state) => ({
    error: state.error
});
const mapDispatchToProps = (dispatch) => ({acceptError: () => dispatch({type: "SET_ERROR", payload: null})});
export default connect(mapStateToProps, mapDispatchToProps)(ErrorWindow);
