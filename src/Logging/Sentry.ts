import * as Sentry from "@sentry/node";
import { getAppdataPath } from "../utils/PathHelper";
import { createAppDataDir } from "../utils/FileHelper";
import { v4 as uuidv4 } from "uuid";
import fs from "fs";
import path from "path";

/**
 * Initializes the Sentry SDK for error tracking.
 */
Sentry.init({
  dsn: "https://fabd7303311f3aff8d0e0c157c6e6724@o1177043.ingest.us.sentry.io/4510432129843200",
  sendDefaultPii: true
});

/**
 * Initializes Sentry user tracking and adds useful application metadata.
 * Should be called after Sentry.init().
 *
 * @returns {Promise<void>}
 */
export async function initializeSentry(): Promise<void> {
  const userId = await getOrCreateSentryUser();
  if (userId) {
    Sentry.setUser({ id: userId });
  }

  Sentry.setContext("application", {
    name: "TidaLuna Installer",
    platform: process.platform,
  });
}

/**
 * Retrieves the stored Sentry user ID, or creates one if it does not exist.
 * This ID persists across application launches.
 *
 * @returns {Promise<string | null>} A stable UUID or null if something failed.
 */
async function getOrCreateSentryUser(): Promise<string | null> {
  try {
    const appDataPath = await getAppdataPath();
    const userFilePath = path.join(appDataPath, "sentry_user_id.txt");

    if (fs.existsSync(userFilePath)) {
      return fs.readFileSync(userFilePath, "utf-8").trim();
    }

    await createAppDataDir();

    const newUserId = uuidv4();
    fs.writeFileSync(userFilePath, newUserId, "utf-8");

    return newUserId;
  } catch (error) {
    console.error("Error accessing or creating Sentry user ID file:", error);
    return null;
  }
}
