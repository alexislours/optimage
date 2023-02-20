const os = require('node:os');
const fs = require('node:fs');
const exec = require('node:child_process').execSync;
const { version } = require('./package.json');

const platform = os.platform();
const arch = os.arch();
const url = `https://github.com/alexislours/optimage/releases/download/v${version}/${platform}-${arch}.node`;

console.log(`Installing for ${platform}-${arch} (v${version})`);

const fetchBinary = async () => {
  console.log("Attempting to fetch prebuilt package from GitHub...");
  await fetch(url)
    .then((res) => {
      if (res.status === 404) {
        console.log(`No prebuilt binary for: ${platform}-${arch}
Attempting to build from source... 
If this step fails, you probably need to install the Rust toolchain: https://www.rust-lang.org/tools/install
This step can take a minute or two...`);
        exec('npm run build-release');
      } else {
        res.arrayBuffer().then((res) => {
          fs.writeFileSync('index.node', Buffer.from(res));
          console.log("Prebuilt binary fetched successfully!");
        });
      }
    })
};

fetchBinary();
