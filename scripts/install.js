const BinWrapper = require('bin-wrapper');
const path = require('path');
const fs = require('fs');

const version = '1.0.0-alpha.2'; // Match this with your package version
const baseUrl = `https://github.com/Govcraft/paseto_cli/releases/download/v${version}`;

const binary = new BinWrapper()
    .src(`${baseUrl}/paseto_cli-darwin-x64.tar.gz`, 'darwin', 'x64')
    .src(`${baseUrl}/paseto_cli-linux-x64.tar.gz`, 'linux', 'x64')
    .src(`${baseUrl}/paseto_cli-win32-x64.zip`, 'win32', 'x64')
    .dest(path.join(__dirname, '..', 'bin'))
    .use(process.platform === 'win32' ? 'paseto_cli.exe' : 'paseto_cli');

binary.run(['--version']).then(() => {
    console.log('Binary installed successfully.');

    const binDir = path.join(__dirname, '..', 'bin');
    const binaryName = process.platform === 'win32' ? 'paseto_cli.exe' : 'paseto_cli';
    const binaryPath = path.join(binDir, binaryName);

    if (!fs.existsSync(binaryPath)) {
        console.error('Error: Binary file not found.');
        process.exit(1);
    }

    console.log(`Binary is installed at: ${binaryPath}`);
}).catch(err => {
    console.error('Error installing binary:', err.message);
});
