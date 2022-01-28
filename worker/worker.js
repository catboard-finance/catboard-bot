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
        PUBLIC_KEY,
        SYMBOLS
      }
    }

    const { status, body } = await wasm_main(context, DEVNET_PYTH_PRODUCTS)

    const embeds = [
      {
        type: 'rich',
        title: 'Solana',
        description: `0️⃣⤴️ 24h +7.26%\n0️⃣⤴️ BB 20 2 = 87.04 ← 128.69 → 170.34 \n0️⃣⤴️ MACD 12 26 close 9 = -3.39 -18.80 -15.40\n0️⃣⤴️ Stoch RSI 14 14 3 3  = K 23.23 D 16.577`,
        color: 0x8400ff
        // image: {
        //   url: `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAgAAAAIAQMAAAD+wSzIAAAABlBMVEX///+/v7+jQ3Y5AAAADklEQVQI12P4AIX8EAgALgAD/aNpbtEAAAAASUVORK5CYII`,
        //   height: 0,
        //   width: 0
        // }
      }
    ]

    const components = [
      {
        type: 1,
        components: [
          {
            style: 1,
            label: `SWAP`,
            custom_id: `row_0_button_0`,
            disabled: false,
            type: 2
          }
        ]
      }
    ]

    const _body = JSON.parse(body)
    _body.data.embeds = embeds
    _body.data.components = components

    console.log(' _body.data:', _body.data)

    let body2 = JSON.stringify(_body)
    console.log('body2:', body2)

    // TODO: workaround for "{\"foo\": \"bar\"}"
    return new Response(body2, {
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
