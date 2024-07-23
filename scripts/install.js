const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const version = '1.0.0-alpha.1'; // Match this with your package version
const baseUrl = `https://github.com/Govcraft/paseto-cli/releases/download/v${version}`;

const platform = process.platform;
const arch = process.arch;

const filename = `paseto_cli-${platform}-${arch}.tar.gz`;
const url = `${baseUrl}/${filename}`;

const targetDir = path.join(__dirname, '..', 'native', platform, arch);

fs.mkdirSync(targetDir, { recursive: true });

const filePath = path.join(targetDir, filename);

https.get(url, (response) => {
    if (response.statusCode === 302) {
        https.get(response.headers.location, (redirectedResponse) => {
            redirectedResponse.pipe(fs.createWriteStream(filePath))
                .on('finish', () => {
                    console.log('Downloaded successfully');
                    execSync(`tar -xzf ${filePath} -C ${targetDir}`);
                    fs.unlinkSync(filePath);
                    fs.chmodSync(path.join(targetDir, 'paseto_cli'), '755');
                });
        });
    } else {
        response.pipe(fs.createWriteStream(filePath))
            .on('finish', () => {
                console.log('Downloaded successfully');
                execSync(`tar -xzf ${filePath} -C ${targetDir}`);
                fs.unlinkSync(filePath);
                fs.chmodSync(path.join(targetDir, 'paseto_cli'), '755');
            });
    }
}).on('error', (err) => {
    console.error('Error downloading file:', err);
});