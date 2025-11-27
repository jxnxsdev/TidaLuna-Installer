import { sendMessageToFrontend } from "..";
import { Popup, PopupButton } from "../types/Popup";
import { getOrCreateSentryUser } from "../Logging/Sentry";
import { WebsocketMessageTypes } from "../enums/WebsocketMessageTypes";


export async function sendErrorHelpPopup() {
    const sentryUserId = await getOrCreateSentryUser();

    const popup: Popup = {
        title: "Installation Error",
        message: "<h3>Unfortunately, an error occurred during the installation process.</h3>\n\nPlease try running the installer again.\nIn case the problem persists, please join the TidaLuna Discord server and open an issue thread there. Make sure to include the error log from the installer.\n\nIn your error report, tag <em>jxnxsdev</em> and send them the following error ID: <em>{sentry:" + sentryUserId + "}</em>",
        type: "error",
        buttons: [
            {
                label: "Join Discord Server",
                action: "open_url",
                value: "https://discord.gg/jK3uHrJGx4",
                color: "primary"
            }
        ]
    };

    await sendMessageToFrontend({
        type: WebsocketMessageTypes.POPUP_MESSAGE,
        data: popup,
    });
}