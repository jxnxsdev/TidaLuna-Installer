# TidaLuna-Installer

**Installer for the TidaLuna Tidal Client Mod**

## Important:

If you encounter any errors, please refer to the FAQ section at the bottom of the page.

---

## How to Use

- **Prebuilt Binaries (Windows Only)**:  
  Prebuilt binaries are available for Windows. Simply download and run them from the [Releases Page](http://localhost:3000).  
  A website should open in your browser. If it doesn't, manually open `http://localhost:3000`.

- **Windows Smart Screen Warning**:  
  If Windows Smart Screen pops up warning that the app might be malicious, click on **More Info** and then **Run Anyway**.  
  This app is not malicious - Windows shows this because I don't want to pay hundreds of euros per year to Microsoft for an official signature.

> **ATTENTION:** Make sure **Tidal** is closed before proceeding!

- **Running the App**:  
  When the app is running, select a release channel and click **Install**.

---

## macOS Instructions (Untested)

The installer should also work on Mac. To use it:

1. Clone this GitHub repo.
2. Install **Node.js 20**.
3. Run `npm install`.
4. Build the project with `npm run tsc` and then `node build`.

---

## FAQ

### 1. The App Crashes on Startup

Make sure that **nothing** is running on port 3000.

### 2. I Can’t Open the Webpage

Ensure that you are using **http** (not https) to access it. Additionally, check that your firewall isn’t blocking the connection.

### 3. The Installer Shows an Error When I Click Install

This typically happens if **Tidal** is still running.

- Open **Task Manager** to ensure Tidal is fully closed.
- If the installer doesn’t work immediately, wait a few minutes - Windows sometimes takes time to recognize that files are no longer in use.
- If the issue persists after 2-3 minutes, try running the installer as **Administrator** to grant the app the necessary permissions to edit Tidal’s files.

Other Reasons can be that Tidal is not installed in the default location. A feature to set a custom location will be implemented soon™

### 4. The Installer Says `original.asar` is Missing When Trying to Uninstall

This indicates that your installation is broken. To resolve this:

- Uninstall and then reinstall Tidal.
