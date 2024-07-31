const { MongoClient } = require('mongodb');

async function main() {
    // MongoDB 连接 URI
    const uri = "mongodb://localhost:27017"; // 如果你使用的是远程 MongoDB，请相应更改 URI

    // 创建一个新的 MongoClient
    const client = new MongoClient(uri, { useNewUrlParser: true, useUnifiedTopology: true });

    try {
        // 连接到 MongoDB 服务器
        await client.connect();

        console.log("Connected successfully to server");

      
    } finally {
        // 确保在完成后关闭连接
        await client.close();
    }
}

main().catch(console.error);