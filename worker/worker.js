addEventListener('fetch', (event) => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  try {
    const { wasm_main } = wasm_bindgen

    // noinspection JSUnresolvedVariable
    await wasm_bindgen(wasm)

    const headers = {}
    for (let [key, value] of request.headers.entries()) {
      headers[key] = value
    }

    // noinspection JSUnresolvedVariable
    const context = {
      request: {
        method: request.method,
        url: request.url,
        headers,
        body: await request.text()
      },
      env: {
        PUBLIC_KEY
      }
    }

    const { status, body } = await wasm_main(context, DEVNET_PYTH_PRODUCTS)

    // TODO: workaround for "{\"foo\": \"bar\"}"
    return new Response(body, {
      status,
      headers: {
        'Content-Type': 'application/json'
      }
    })
  } catch (e) {
    return new Response(e.toString(), {
      status: 500,
      headers: {
        'Content-Type': 'text/plain'
      }
    })
  }
}
