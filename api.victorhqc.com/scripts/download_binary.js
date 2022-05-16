const fs = require('fs');
const path = require('path');
const { Readable } = require('stream');

main().catch((e) => {
  console.error(e);
  process.exit(1);
});

async function main() {
  const asset = await getAsset();
  console.log('ASSET', asset);
  downloadFile(asset);
}

async function getAsset() {
  const headers = new Headers();
  headers.append(
    'Authorization',
    `token ${process.env.TOKEN ?? 'TOKEN is not defined'}`
  );

  const response = await fetch(
    process.env.GH_TAGS ?? 'GH_TAGS is not defined',
    { headers }
  );

  if (!response.ok) {
    throw new Error('Failed to fetch Github Data');
  }

  const json = await response.json();

  const asset = json.assets.find(
    (asset) => asset.name.match(/unknown-linux-musl\.zip$/g) !== null
  );

  if (!asset) {
    throw new Error('Failed to find an asset.');
  }

  return asset;
}

async function downloadFile(asset) {
  const headers = new Headers();
  headers.append(
    'Authorization',
    `token ${process.env.TOKEN ?? 'TOKEN is not defined'}`
  );
  headers.append('Accept', 'application/octet-stream');

  console.log('attempting to download', asset.url);
  const file = await fetch(asset.url, { headers });

  if (!file.ok) {
    throw new Error('Failed to get binary');
  }

  const desiredPath = path.join(__dirname, '..', 'api_victorhqc_com.zip');

  const fileStream = fs.createWriteStream(desiredPath);
  await new Promise((resolve, reject) => {
    const readable = toReadable(file);
    readable.pipe(fileStream);
    readable.on('error', reject);
    fileStream.on('finish', resolve);
  });
}

function toReadable(response) {
  const reader = response.body.getReader();
  const rs = new Readable();

  rs._read = async () => {
    const result = await reader.read();
    if (!result.done) {
      rs.push(Buffer.from(result.value));
    } else {
      rs.push(null);
      return;
    }
  };

  return rs;
}
