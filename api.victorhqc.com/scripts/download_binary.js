const fs = require("fs");
const path = require("path");

main().catch((e) => {
  console.error(e);
  process.exit(1);
});

async function main() {
  const asset = await getAsset();
  console.log("ASSET", asset);
}

async function getAsset() {
  const headers = new Headers();
  headers.append(
    "Authorization",
    `token ${process.env.TOKEN ?? "TOKEN is not defined"}`
  );

  const response = await fetch(
    process.env.GH_TAGS ?? "GH_TAGS is not defined",
    { headers }
  );

  if (!response.ok) {
    throw new Error("Failed to fetch Github Data");
  }

  const json = await response.json();

  const asset = json.assets.find(
    (asset) => asset.name.test(/unknown-linux-musl\.zip$/g) !== null
  );

  if (!asset) {
    throw new Error("Failed to find an asset.");
  }

  return asset;
}

async function downloadFile(asset) {
  const headers = new Headers();
  headers.append(
    "Authorization",
    `token ${process.env.TOKEN ?? "TOKEN is not defined"}`
  );

  const file = await fetch(asset.browser_download_url, { headers });
  const desiredPath = path.join(__dirname, "..", "..", asset.name);

  const fileStream = fs.createWriteStream(desiredPath);
  await new Promise((resolve, reject) => {
    file.body.pipe(fileStream);
    file.body.on("error", reject);
    fileStream.on("finish", resolve);
  });
}
