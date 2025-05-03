import { WebsocketMessageTypes } from "../enums/WebsocketMessageTypes"
import { StepLog } from "./StepLog";
import { InstallLog } from "./InstallLog";
import { InstallInfo } from "./InstallInfo";

/**
* @description This type is used to log the installation process of the app.
* @param type The type of the websocket message.
* @param data The data of the websocket message.
*/
export type WebsocketMessage = {
    type: WebsocketMessageTypes;
    data: StepLog | InstallLog | InstallInfo | string;
}