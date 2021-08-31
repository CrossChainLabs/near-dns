const IPFS = require('ipfs-core')
const fs = require('fs')

async function store() {
    try {
        const data = fs.readFileSync('./src/ipfs/sample.html', 'utf8')
        //console.log(data)

        const ipfs = await IPFS.create()
        const { cid } = await ipfs.add(data)
        console.info(cid)

    } catch (err) {
        console.error(err)
    }

}

store();