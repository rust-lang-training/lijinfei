const rustModule = require('./neon-lab')
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

// console.log(rustModule);
const files = ['./package.json', './index.js', './neon-lab/cargo.toml']

files.forEach((file) => {
  console.log(file, rustModule.digestFile(file))
})
