const fs = require('fs');
const path = require('path');
const zlib = require('zlib');
const https = require('https');

const BINARY_DISTRIBUTION_PACKAGES = {
    'linux-x64': 'paseto_cli-linux-x64',
    'darwin-x64': 'paseto_cli-darwin-x64',
    'darwin-aarch64': 'paseto_cli-darwin-aarch64',
    'win32-x64': 'paseto_cli-win32-x64'
};

const BINARY_DISTRIBUTION_VERSION = process.env.BINARY_DISTRIBUTION_VERSION || '1.0.0-alpha.11';
const binaryName = process.platform === 'win32' ? 'paseto_cli.exe' : 'paseto_cli';
const platformSpecificPackageName = BINARY_DISTRIBUTION_PACKAGES[`${process.platform}-${process.arch}`];
const fallbackBinaryPath = path.join(__dirname, binaryName);

if (process.env.CI) {
    console.log('CI environment detected. Skipping binary download.');
    process.exit(0);
}

function makeRequest(url) {
    return new Promise((resolve, reject) => {
        https.get(url, (response) => {
            console.log(`Using binary distribution version: ${BINARY_DISTRIBUTION_VERSION}`);
            console.log(`Attempting to download dependency from ${url}`);
            if (response.statusCode >= 200 && response.statusCode < 300) {
                const chunks = [];
                response.on('data', (chunk) => chunks.push(chunk));
                response.on('end', () => {
                    resolve(Buffer.concat(chunks));
                });
            } else if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
                makeRequest(response.headers.location).then(resolve, reject);
            } else {
                reject(new Error(`npm responded with status code ${response.statusCode} when downloading the package!`));
            }
        }).on('error', (error) => {
            reject(error);
        });
    });
}

function extractFileFromTarball(tarballBuffer, filepath) {
    let offset = 0;
    while (offset < tarballBuffer.length) {
        const header = tarballBuffer.subarray(offset, offset + 512);
        offset += 512;
        const fileName = header.toString('utf-8', 0, 100).replace(/\0.*/g, '');
        const fileSize = parseInt(header.toString('utf-8', 124, 136).replace(/\0.*/g, ''), 8);
        if (fileName === filepath) {
            return tarballBuffer.subarray(offset, offset + fileSize);
        }
        offset = (offset + fileSize + 511) & ~511;
    }
}

async function downloadBinaryFromNpm() {
    try {
        const tarballDownloadBuffer = await makeRequest(
            `https://registry.npmjs.org/@govcraft/${platformSpecificPackageName}/-/${platformSpecificPackageName}-${BINARY_DISTRIBUTION_VERSION}.tgz`
        );
        const tarballBuffer = zlib.unzipSync(tarballDownloadBuffer);
        fs.writeFileSync(
            fallbackBinaryPath,
            extractFileFromTarball(tarballBuffer, `./package/bin/${binaryName}`),
            { mode: 0o755 }
        );
    } catch (error) {
        console.error('Failed to download and install binary:', error);
        process.exit(1);
    }
}

function isPlatformSpecificPackageInstalled() {
    try {
        require.resolve(`./${platformSpecificPackageName}/bin/${binaryName}`);
        return true;
    } catch (e) {
        return false;
    }
}

if (!platformSpecificPackageName) {
    throw new Error('Platform not supported!');
}

if (!isPlatformSpecificPackageInstalled()) {
    console.log('Platform specific package not found. Will manually download binary.');
    downloadBinaryFromNpm();
} else {
    console.log('Platform specific package already installed.');
}
