const fs = require('fs');

const packageJson = require('../package.json');
const newVersion = packageJson.version;

const updatedOptionalDependencies = Object.keys(packageJson.optionalDependencies).reduce((acc, key) => {
    acc[key] = newVersion;
    return acc;
}, {});

packageJson.optionalDependencies = updatedOptionalDependencies;

fs.writeFileSync('../package.json', JSON.stringify(packageJson, null, 2), 'utf8');

console.log(`Updated optionalDependencies to version ${newVersion}`);
