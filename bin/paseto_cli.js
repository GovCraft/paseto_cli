#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

const binaryPath = path.join(__dirname, '..', 'native', process.platform, process.arch, 'paseto_cli');

const child = spawn(binaryPath, process.argv.slice(2), { stdio: 'inherit' });

child.on('error', (err) => {
    console.error('Failed to start subprocess:', err);
});

child.on('close', (code) => {
    process.exit(code);
});