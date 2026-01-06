#!/usr/bin/env node

/**
 * Postinstall script that downloads the slvsx binary for the current platform
 * from GitHub releases.
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import https from 'https';
import { createWriteStream } from 'fs';
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const packageRoot = path.resolve(__dirname, '..');

// Map Node.js platform/arch to our release asset naming
function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;
  
  // Map to actual release asset names like: slvsx-macos-arm64.tar.gz, slvsx-linux.tar.gz
  if (platform === 'darwin') {
    if (arch === 'arm64') {
      return { assetPattern: 'macos-arm64', extension: '' };
    } else if (arch === 'x64') {
      return { assetPattern: 'macos-x86_64', extension: '' };
    }
  } else if (platform === 'linux') {
    // Linux release is just "linux" (x86_64)
    if (arch === 'x64') {
      return { assetPattern: 'linux', extension: '' };
    }
  } else if (platform === 'win32') {
    if (arch === 'x64') {
      return { assetPattern: 'windows', extension: '.exe' };
    }
  }
  
  return null;
}

async function getLatestRelease() {
  return new Promise((resolve, reject) => {
    const options = {
      hostname: 'api.github.com',
      path: '/repos/snoble/slvsx-cli/releases/latest',
      headers: {
        'User-Agent': 'slvsx-mcp-server-installer'
      }
    };
    
    https.get(options, (res) => {
      let data = '';
      res.on('data', chunk => data += chunk);
      res.on('end', () => {
        try {
          resolve(JSON.parse(data));
        } catch (e) {
          reject(new Error('Failed to parse release info'));
        }
      });
    }).on('error', reject);
  });
}

async function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const follow = (url) => {
      https.get(url, (res) => {
        if (res.statusCode === 302 || res.statusCode === 301) {
          follow(res.headers.location);
          return;
        }
        
        if (res.statusCode !== 200) {
          reject(new Error(`Download failed with status ${res.statusCode}`));
          return;
        }
        
        const file = createWriteStream(dest);
        res.pipe(file);
        file.on('finish', () => {
          file.close();
          resolve();
        });
      }).on('error', reject);
    };
    
    follow(url);
  });
}

async function extractTarGz(file, dest) {
  await execAsync(`tar -xzf "${file}" -C "${dest}"`);
}

async function extractZip(file, dest) {
  if (process.platform === 'win32') {
    await execAsync(`powershell -Command "Expand-Archive -Path '${file}' -DestinationPath '${dest}'"`);
  } else {
    await execAsync(`unzip -o "${file}" -d "${dest}"`);
  }
}

async function main() {
  const binDir = path.join(packageRoot, 'bin');
  const binaryPath = path.join(binDir, process.platform === 'win32' ? 'slvsx.exe' : 'slvsx');
  
  // Skip if binary already exists
  if (fs.existsSync(binaryPath)) {
    console.log('slvsx binary already installed.');
    return;
  }
  
  const platformInfo = getPlatformInfo();
  if (!platformInfo) {
    console.error(`Unsupported platform: ${process.platform}-${process.arch}`);
    console.error('Please install slvsx manually from https://github.com/snoble/slvsx-cli/releases');
    process.exit(0); // Don't fail install, just warn
  }
  
  console.log(`Installing slvsx for ${process.platform}-${process.arch}...`);
  
  try {
    const release = await getLatestRelease();
    
    if (!release.assets) {
      console.error('No release assets found. Please install slvsx manually.');
      process.exit(0);
    }
    
    // Find the right asset
    const asset = release.assets.find(a => 
      a.name.includes(platformInfo.assetPattern) && 
      (a.name.endsWith('.tar.gz') || a.name.endsWith('.zip'))
    );
    
    if (!asset) {
      console.error(`No binary found for ${process.platform}-${process.arch} (looking for '${platformInfo.assetPattern}')`);
      console.error('Available assets:', release.assets.map(a => a.name).join(', '));
      console.error('Please install slvsx manually from https://github.com/snoble/slvsx-cli/releases');
      process.exit(0);
    }
    
    // Create bin directory
    fs.mkdirSync(binDir, { recursive: true });
    
    // Download
    const tempFile = path.join(binDir, asset.name);
    console.log(`Downloading ${asset.name}...`);
    await downloadFile(asset.browser_download_url, tempFile);
    
    // Extract
    console.log('Extracting...');
    if (asset.name.endsWith('.tar.gz')) {
      await extractTarGz(tempFile, binDir);
    } else {
      await extractZip(tempFile, binDir);
    }
    
    // Clean up archive
    fs.unlinkSync(tempFile);
    
    // Make executable on Unix
    if (process.platform !== 'win32') {
      fs.chmodSync(binaryPath, 0o755);
    }
    
    // Verify
    if (fs.existsSync(binaryPath)) {
      console.log(`✓ slvsx installed successfully to ${binaryPath}`);
    } else {
      // Binary might be in a subdirectory after extraction
      const files = fs.readdirSync(binDir);
      console.log('Extracted files:', files);
      console.error('Binary not found at expected location. Please check extraction.');
    }
    
  } catch (error) {
    console.error('Failed to download slvsx:', error.message);
    console.error('Please install manually from https://github.com/snoble/slvsx-cli/releases');
    process.exit(0); // Don't fail the npm install
  }
}

main();

