const https = require('https')
const { v4 } = require('uuid');
const axios = require('axios');
const http = require('http')

// Dev RC
const VASconfig = {
  verityUrl: 'https://vas.pdev.evernym.com/api/',
  domainDID: '32djqLcu9WGsZL4MwyAjVn',
  apiKey: 'C6jtgbRwzTHp1T1mQSFGDf2YTdHeN1kj2sJr7VJbvT5P:4iBetiYFD998So2APxqRRdjYFg7qhjQvLnkwpJ6vDBszoWGuRpj75YKvLJKBhsXSQtPvXTGghCyKaPMLJEVwX6v7'
}
const httpsConfig = {
  timeout: 180000,
  httpsAgent: new https.Agent({}),
  headers: {
    'X-API-KEY': VASconfig['apiKey'],
  },
}

const main = async () => {
  // Start server to listen VAS responses
  let lastVASresponse
  const server = http
    .createServer(function (request, response) {
      const { headers, method, url } = request

      console.log('URL')
      console.log(request.url)

      let body = ''

      if (request.url === '/') {
          request
            .on('error', (err) => {
              console.error(err)
            })
            .on('data', (chunk) => {
              body += chunk.toString()
            })
            .on('end', () => {
              lastVASresponse = body
              console.log(
                '----------------\n',
                `Headers: ${JSON.stringify(headers)}\n`,
                `Method: ${method}\n`,
                `URL: ${url}\n`,
                `Body: ${body}\n`,
                '----------------\n'
              )
            })
      } else if (request.url === '/last-response') {
        response.writeHead(200, {'Content-Type': 'text/html'});
        response.end(lastVASresponse);
      }
    })
    .listen(1338)
  console.log('Server is listening on port 1338...')


  // Register server endpoint in VAS - run it once
  await axios.post(
    `${VASconfig['verityUrl']}${VASconfig['domainDID']}/configs/0.6/${v4()}`,
    {
      '@id': v4(),
      '@type': 'did:sov:123456789abcdefghi1234;spec/configs/0.6/UPDATE_COM_METHOD',
      'comMethod': {
        'id': 'webhook',
        'type': 2,
        'value': 'http://68b282246827.ngrok.io', // it changes everytime you run ngrok
        'packaging': {
          'pkgType': 'plain'
        }
      }
    },
    httpsConfig
  )
    .catch(err => console.error(err))
    .then(res => console.log(res.data))
  // CreateRelationship request :: VAS returns RelationshipCreated
  await axios.post(
    `${VASconfig['verityUrl']}${VASconfig['domainDID']}/relationship/1.0/${v4()}`,
    {
      '@type': 'did:sov:123456789abcdefghi1234;spec/relationship/1.0/create',
      '@id': v4(),
      'label': 'Alex',
      'logoUrl': 'http://robohash.org/235',
    },
    httpsConfig
  )
    .catch(err => console.error(err))
    .then(res => console.log(res.data))
  await new Promise((r) => setTimeout(r, 10000))

  console.log(lastVASresponse)

  let lastResponse = JSON.parse(lastVASresponse)
  const did = lastResponse['did']
  console.log(`DID: ${did}`)

  const THREAD_ID = lastResponse['~thread']['thid']
  console.log(`THREAD ID: ${THREAD_ID}`)

  await axios.post(
    `${VASconfig['verityUrl']}${VASconfig['domainDID']}/relationship/1.0/${THREAD_ID}`,
    {
      '@type':
       'did:sov:123456789abcdefghi1234;spec/relationship/1.0/connection-invitation',
//        'did:sov:123456789abcdefghi1234;spec/relationship/1.0/out-of-band-invitation',
      '@id': v4(),
      '~for_relationship': did,
      "shortInvite": true
    },
    httpsConfig
  )
    .catch(err => console.error(err))
    .then(res => console.log(res.data))

  // Shutdown server
  await new Promise((r) => setTimeout(r, 60000))

/*
    await axios.post(
    `${VASconfig['verityUrl']}${VASconfig['domainDID']}/committedanswer/1.0/${v4()}`,
    {
      '@type': 'did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/committedanswer/1.0/ask-question',
      '@id': v4(),
      '~for_relationship': did,
      'text': 'To be or to not be?',
      'detail': 'The second classic philosophical questions',
      'valid_responses': [ "be", "not be" , "not 2 be" ]
    },
    httpsConfig
    )
    .catch(err => {
      console.error('Error')
      console.error(err)
    })
    .then(res => {
      console.error('Result')
      console.log(res.data)
    })
      await new Promise((r) => setTimeout(r, 5000))*/

  await new Promise((r) => setTimeout(r, 900000))

  server.close()
  console.log('Invitation server has been stopped.')
}

main()