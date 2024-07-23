#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const platform = process.platform;
const arch = process.arch;
const binaryName = platform === 'win32' ? 'paseto_cli.exe' : 'paseto_cli';
const binaryPath = path.join(__dirname, binaryName);

if (!fs.existsSync(binaryPath)) {
    console.error(`Error: PASETO CLI binary not found at ${binaryPath}`);
    console.error('Please ensure the package is installed correctly or try reinstalling.');
    process.exit(1);
}

const child = spawn(binaryPath, process.argv.slice(2), { stdio: 'inherit' });

child.on('error', (err) => {
    console.error('Failed to start PASETO CLI:', err);
    process.exit(1);
});

child.on('close', (code) => {
    process.exit(code);
});
