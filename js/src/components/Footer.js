import React from 'react';
import {connect} from "react-redux";

class Footer extends React.Component {
    render() {
        return <footer>
            <p><span className="codeFont">disk9k1</span> v{this.props.instance.data.version}</p>
        </footer>
    }
}

const mapStateToProps = (state) => ({instance: state.instance});
const mapDispatchToProps = (dispatch) => ({});

export default connect(mapStateToProps, mapDispatchToProps)(Footer)