const neoLib = require("../node-mod/neon-lab")

const crypto = require('crypto')
const fsPromises = require('fs/promises')
async function digestFile(filename) {
  const hasher = crypto.createHash('SHA256')
  const fin = await fsPromises.open(filename, 'r')
  const buf = new Buffer.alloc(1024 * 64)
  for (;;) {
    const { bytesRead } = await fin.read(buf, 0, buf.length, null)
    if (bytesRead === 0) {
      break
    }
    hasher.update(buf.subarray(0, bytesRead))
  }
  await fin.close()
  return hasher.digest('hex')
}
const files = [
  // "./package.json",
  // "./index.js",
  // "../node-mod/neon-lab/cargo.toml"
  "./20240702.zip",
  "./20240710.zip",
  "./UAP.exe"
]


function digestFile(file) {
  
}

files.forEach( async file => {
  const stat = await fsPromises.stat(file)
  const startTime = Date.now()
  const digestStr = neoLib.digestFile(file)
  const endTime = Date.now()
  const speed = (stat.size / 1024 / 1024) * 1000 / (endTime - startTime)
  console.log(`
    加密的文件为：${file},
    摘要为: ${digestStr},
    速度为: ${speed},
    用时: ${endTime - startTime} ms
  `)
  // console.log(file, neoLib.digestFile(file))
})