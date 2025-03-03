const path = require("path");
const childProcess = require("child_process");

// Lookup table for all platforms and binary distribution packages
const BINARY_DISTRIBUTION_PACKAGES = {
    'linux-x64': 'paseto_cli-linux-x64',
    'darwin-x64': 'paseto_cli-darwin-x64',
    'darwin-aarch64': 'paseto_cli-darwin-aarch64',
    'win32-x64': 'paseto_cli-win32-x64'
};

// Windows binaries end with .exe so we need to special case them.
const binaryName = process.platform === "win32" ? "paseto_cli.exe" : "paseto_cli";

// Determine package name for this platform
const platformSpecificPackageName = BINARY_DISTRIBUTION_PACKAGES[process.platform];

function getBinaryPath() {
    try {
        // Resolving will fail if the optionalDependency was not installed
        return require.resolve(`${platformSpecificPackageName}/bin/${binaryName}`);
    } catch (e) {
        return path.join(__dirname, "..", binaryName);
    }
}

module.exports.runBinary = function (...args) {
    childProcess.execFileSync(getBinaryPath(), args, {
        stdio: "inherit",
    });
};