beforeAll(async function () {
  // NOTE: nearlib and nearConfig are made available by near-cli/test_environment
  const near = await nearlib.connect(nearConfig)
  window.accountId = nearConfig.contractName
  window.contract = await near.loadContract(nearConfig.contractName, {
    viewMethods: ['get_a', 'get_aaaa', 'get_content_hash', 'get_txt'],
    changeMethods: ['set_a', 'set_aaaa', 'set_content_hash', 'set_txt'],
    sender: window.accountId
  })

  window.walletConnection = {
    requestSignIn() {
    },
    signOut() {
    },
    isSignedIn() {
      return true
    },
    getAccountId() {
      return window.accountId
    }
  }
})

test('set_then_get_a', async () => {
  await window.contract.set_a({ a_record: "127.0.0.1" })
  const message = await window.contract.get_a({ account_id: window.accountId })
  expect(message).toEqual("127.0.0.1")
})

test('set_then_get_aaaa', async () => {
  await window.contract.set_aaaa({ aaaa_record: "::1" })
  const message = await window.contract.get_aaaa({ account_id: window.accountId })
  expect(message).toEqual("::1")
})

test('set_then_get_content_hash', async () => {
  await window.contract.set_content_hash({ content_hash: "ipfs_cid" })
  const message = await window.contract.get_content_hash({ account_id: window.accountId })
  expect(message).toEqual("ipfs_cid")
})

test('set_then_get_txt', async () => {
  await window.contract.set_txt({ txt_record: "txt" })
  const message = await window.contract.get_txt({ account_id: window.accountId })
  expect(message).toEqual("txt")
})

