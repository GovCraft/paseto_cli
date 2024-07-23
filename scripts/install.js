const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const version = '1.0.0-alpha.1'; // Match this with your package version
const baseUrl = `https://github.com/Govcraft/paseto_cli/releases/download/v${version}`;
const platform = process.platform;
const arch = process.arch;

const isWindows = platform === 'win32';
const fileExtension = isWindows ? 'zip' : 'tar.gz';
const filename = `paseto_cli-${platform}-${arch}.${fileExtension}`;
const url = `${baseUrl}/${filename}`;

const targetDir = path.join(__dirname, '..', 'native', platform, arch);

console.log(`Installing PASETO CLI v${version} for ${platform}-${arch}`);
console.log(`Downloading from: ${url}`);

fs.mkdirSync(targetDir, { recursive: true });

const filePath = path.join(targetDir, filename);

function extractFile() {
    console.log(`Extracting to ${targetDir}...`);
    if (isWindows) {
        execSync(`powershell -command "Expand-Archive -Path '${filePath}' -DestinationPath '${targetDir}' -Force"`);
    } else {
        execSync(`tar -xzf "${filePath}" -C "${targetDir}"`);
    }
    fs.unlinkSync(filePath);

    const binaryName = isWindows ? 'paseto_cli.exe' : 'paseto_cli';
    const binaryPath = path.join(targetDir, binaryName);

    if (!isWindows) {
        fs.chmodSync(binaryPath, '755');
    }

    console.log('Installation completed successfully.');
}

function handleDownload(response) {
    if (response.statusCode === 200) {
        console.log('Download started...');
        const file = fs.createWriteStream(filePath);
        response.pipe(file);

        file.on('finish', () => {
            file.close();
            console.log('Download completed.');
            try {
                extractFile();
            } catch (error) {
                console.error('Error during extraction:', error.message);
                process.exit(1);
            }
        });
    } else {
        console.error(`Server responded with status code: ${response.statusCode}`);
        process.exit(1);
    }
}

https.get(url, (response) => {
    if (response.statusCode === 302) {
        console.log('Following redirect...');
        https.get(response.headers.location, handleDownload);
    } else {
        handleDownload(response);
    }
}).on('error', (err) => {
    console.error('Error downloading file:', err.message);
    process.exit(1);
});