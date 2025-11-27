/**
 * @fileoverview Application constants and enumerations
 */

export const Steps = {
  SETUP: "SETUP",
  DOWNLOADING_LUNA: "DOWNLOADING_LUNA",
  EXTRACTING_LUNA: "EXTRACTING_LUNA",
  COPYING_ASAR_INSTALL: "COPYING_ASAR_INSTALL",
  INSERTING_LUNA: "INSERTING_LUNA",
  UNINSTALLING: "UNINSTALLING",
  COPYING_ASAR_UNINSTALL: "COPYING_ASAR_UNINSTALL",
  KILLING_TIDAL: "KILLING_TIDAL",
}

export const WebsocketMessageTypes = {
  INSTALL_LOG: "install_log",
  STEP_LOG: "step_log",
  STEP_UPDATE: "step_update",
  INSTALLATION_COMPLETE: "installation_complete",
  INSTALLATION_START: "installation_start",
  INSTALLATION_ERROR: "installation_error",
  POPUP_MESSAGE: "popup_message",
}

export const StepNames = {
  [Steps.SETUP]: "Setup",
  [Steps.KILLING_TIDAL]: "Stopping TIDAL",
  [Steps.DOWNLOADING_LUNA]: "Downloading TidaLuna",
  [Steps.EXTRACTING_LUNA]: "Extracting TidaLuna",
  [Steps.COPYING_ASAR_INSTALL]: "Copying ASAR file",
  [Steps.COPYING_ASAR_UNINSTALL]: "Copying ASAR file",
  [Steps.INSERTING_LUNA]: "Installing TidaLuna",
  [Steps.UNINSTALLING]: "Uninstalling TidaLuna",
}
