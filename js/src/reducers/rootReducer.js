import del from "./deleteReducer";
import dir from "./dirReducer";
import error from "./errorReducer";
import instance from "./instanceReducer";
import reloadDir from "./reloadDir"
import user from "./userReducer";
import {combineReducers} from "redux";

const root = combineReducers({del, dir, error, instance, reloadDir, user});
export default root;