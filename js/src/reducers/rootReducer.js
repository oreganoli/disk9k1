import crDir from "./crDirReducer";
import del from "./deleteReducer";
import dir from "./dirReducer";
import error from "./errorReducer";
import instance from "./instanceReducer";
import rename from "./renameReducer";
import reloadDir from "./reloadDir"
import user from "./userReducer";
import {combineReducers} from "redux";

const root = combineReducers({crDir, del, dir, error, instance, reloadDir, rename, user});
export default root;