import '../module-resolver-helper'

import { assert } from 'chai'
import { connectionCreate, connectionCreateConnect, dataConnectionCreate } from 'helpers/entities'
import { INVITE_ACCEPTED_MESSAGE, INVITE_REDIRECTED_MESSAGE, INVITE_DETAILS, OUTOFBAND_INVITE } from 'helpers/test-constants'
import { initVcxTestMode, shouldThrow, sleep } from 'helpers/utils'
import { Connection, StateType, VCXCode, VCXMock, VCXMockMessage } from 'src'

describe('Connection:', () => {
  before(() => initVcxTestMode())

  describe('create:', () => {
    it('success', async () => {
      await connectionCreate()
    })

    it('success: parallel', async () => {
      const numConnections = 50
      const data = dataConnectionCreate()
      await Promise.all(new Array(numConnections).fill(0).map(() => connectionCreate(data)))
    })
  })

  describe('connect:', () => {
    it('success: without phone', async () => {
      const connection = await connectionCreate()
      const inviteDetails = await connection.connect({ data: '{"connection_type":"QR"}' })
      assert.notEqual(inviteDetails, '')
    })

    it('success: with phone', async () => {
      const connection = await connectionCreate()
      const inviteDetails = await connection.connect({ data: '{"connection_type":"SMS","phone":"7202200000"}' })
      assert.notEqual(inviteDetails, '')
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const err = await shouldThrow(async () => connection.connect({ data: '{"connection_type":"QR"}' }))
      assert.equal(err.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })
  })

  describe('sendMessage:', () => {
    it('success: sends message', async () => {
      const connection = await connectionCreate()
      await connection.connect({ data: '{"connection_type":"QR"}' })
      const error = await shouldThrow(() => connection.sendMessage({ msg: 'msg', type: 'msg', title: 'title' }))
      assert.equal(error.vcxCode, VCXCode.NOT_READY)
    })
  })

  describe('signData:', () => {
    it('success: signs data', async () => {
      const connection = await connectionCreate()
      await connection.connect({ data: '{"connection_type":"QR"}' })
      const signature = await connection.signData(new Buffer('random string'))
      assert(signature)
    })
  })

  describe('verifySignature', () => {
    it('success: verifies the signature', async () => {
      const connection = await connectionCreate()
      await connection.connect({ data: '{"connection_type":"QR"}' })
      const valid = await connection.verifySignature({data: new Buffer('random string'),
        signature: new Buffer('random string')})
      assert(valid)
    })
  })
  describe('serialize:', () => {
    it('success', async () => {
      const connection = await connectionCreate()
      const serialized = await connection.serialize()
      assert.ok(serialized)
      assert.property(serialized, 'version')
      assert.property(serialized, 'data')
      const { data, version } = serialized
      assert.ok(data)
      assert.ok(version)
      assert.equal(data.source_id, connection.sourceId)
      assert.equal(data.state, StateType.Initialized)
    })

    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })

    it('throws: connection deleted', async () => {
      const connection = await connectionCreate()
      await connection.connect({ data: '{"connection_type":"QR"}' })
      await connection.delete()
      const error = await shouldThrow(() => connection.serialize())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })
  })

  describe('deserialize:', () => {
    it('success', async () => {
      const connection1 = await connectionCreate()
      const data1 = await connection1.serialize()
      const connection2 = await Connection.deserialize(data1)
      assert.equal(connection2.sourceId, connection1.sourceId)
      const data2 = await connection2.serialize()
      assert.deepEqual(data1, data2)
    })

    it('throws: incorrect data', async () => {
      const error = await shouldThrow(async () => Connection.deserialize({ data:
        { source_id: 'Invalid' } } as any))
      assert.equal(error.vcxCode, VCXCode.INVALID_JSON)
    })
  })

  describe('updateState:', () => {
    it('throws: not initialized', async () => {
      const connection = new (Connection as any)()
      const error = await shouldThrow(() => connection.updateState())
      assert.equal(error.vcxCode, VCXCode.INVALID_CONNECTION_HANDLE)
    })

    it(`returns ${StateType.Initialized}: not connected`, async () => {
      const connection = await connectionCreate()
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Initialized)
    })

    it(`returns ${StateType.OfferSent}: connected`, async () => {
      const connection = await connectionCreateConnect()
      VCXMock.setVcxMock(VCXMockMessage.AcceptInvite)
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Accepted)
    })

    it(`returns ${StateType.Accepted}: mocked accepted`, async () => {
      const connection = await connectionCreateConnect()
      VCXMock.setVcxMock(VCXMockMessage.GetMessages)
      await connection.updateState()
      assert.equal(await connection.getState(), StateType.Accepted)
    })

    it(`returns ${StateType.Accepted}: mocked accepted`, async () => {
      const connection = await connectionCreateConnect()
      await connection.updateStateWithMessage(INVITE_ACCEPTED_MESSAGE)
      assert.equal(await connection.getState(), StateType.Accepted)
    })

    it(`returns ${StateType.Redirected}: redirected with message`, async () => {
      const connection = await connectionCreateConnect()
      await connection.updateStateWithMessage(INVITE_REDIRECTED_MESSAGE)
      assert.equal(await connection.getState(), StateType.Redirected)
    })

    it.skip(`returns ${StateType.Accepted}: mocked accepted in parallel`, async () => {
      const numConnections = 50
      const interval = 50
      const sleepTime = 100
      const connectionsWithTimers = await Promise.all(new Array(numConnections).fill(0).map(async () => {
        const connection = await connectionCreate()
        const timer = setInterval(() => connection.updateState(), interval)
        return { connection, timer }
      }))
      let cond = false
      while (cond) {
        const states = await Promise.all(connectionsWithTimers.map(({ connection }) => connection.getState()))
        cond = states.every((state) => state === StateType.Accepted)
        VCXMock.setVcxMock(VCXMockMessage.GetMessages)
        await sleep(sleepTime)
      }
      connectionsWithTimers.forEach(({ timer }) => clearInterval(timer))
    })
  })

  describe('inviteDetails:', () => {
    it('success: with abbr', async () => {
      const connection = await connectionCreateConnect()
      const details = await connection.inviteDetails(true)
      assert.include(details, '"dp":')
    })

    it('success: without abbr', async () => {
      const connection = await connectionCreateConnect()
      const details = await connection.inviteDetails()
      assert.include(details, '"senderAgencyDetail":')
    })
  })

  describe('sendPing:', () => {
    it('success: send ping', async () => {
      const connection = await connectionCreate()
      const error = await shouldThrow(() => connection.sendPing('ping'))
      assert.equal(error.vcxCode, VCXCode.ACTION_NOT_SUPPORTED)
    })
  })

  describe('sendDiscoveryFeatures:', () => {
    it('success: send discovery features', async () => {
      const connection = await connectionCreate()
      const error = await shouldThrow(() => connection.sendDiscoveryFeatures('*', 'comment'))
      assert.equal(error.vcxCode, VCXCode.ACTION_NOT_SUPPORTED)
    })
  })
  describe('redirect:', () => {
    it('success', async () => {
      // create an connection.
      const old_connection = await connectionCreateConnect()
      await old_connection.updateStateWithMessage(INVITE_ACCEPTED_MESSAGE)
      assert.equal(await old_connection.getState(), StateType.Accepted)

      const connection = await Connection.createWithInvite({
        'id': 'new',
        'invite': INVITE_DETAILS
      })
      await connection.connectionRedirect(old_connection)
      assert.equal(await connection.getState(), StateType.Redirected)
    })
  })

  describe('getRedirectDetails:', () => {
    it('success', async () => {
      const connection = await connectionCreateConnect()
      await connection.updateStateWithMessage(INVITE_REDIRECTED_MESSAGE)
      const details = await connection.getRedirectDetails()
      assert.include(details, '"DID":')
    })
  })

  describe('createWithOutofbandInvite:', () => {
    it('success: create with out-of-band invitation', async () => {
      await Connection.createWithOutofbandInvite({
        id: 'new',
        invite: OUTOFBAND_INVITE
      })
    })
  })

  describe('sendReuse:', () => {
    it('success: send reuse', async () => {
      const connection = await connectionCreate()
      const error = await shouldThrow(() => connection.sendReuse(OUTOFBAND_INVITE))
      assert.equal(error.vcxCode, VCXCode.ACTION_NOT_SUPPORTED)
    })
  })

  describe('create out-of-band:', () => {
    it('success', async () => {
      const data = {
        goal: 'Foo Goal',
        handshake: true,
        id: 'foobar123'
      }
      const error = await shouldThrow(() => Connection.createOutofband(data))
      assert.equal(error.vcxCode, VCXCode.ACTION_NOT_SUPPORTED)

    })
  })

  describe('sendAnswer:', () => {
    it('success: send answer', async () => {
      const connection = await connectionCreate()
      const data = {
        answer: {
          text: 'Yes, it\'s me'
        },
        question: {
          '@id': '518be002-de8e-456e-b3d5-8fe472477a86',
          '@type': 'did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/questionanswer/1.0/question',
          'question_text': 'Alice, are you on the phone with Bob from Faber Bank right now?',
          'valid_responses' : [
                  { text: 'Yes, it\'s me' },
                  { text: 'No, that\'s not me!' }
          ],
          '~timing': {
            expires_time: '2018-12-13T17:29:06+0000'
          }
        }
      }
      const error = await shouldThrow(() => connection.sendAnswer(data))
      assert.equal(error.vcxCode, VCXCode.ACTION_NOT_SUPPORTED)
    })
  })

  describe('sendInviteAction:', () => {
    it('success: send invite action', async () => {
      const connection = await connectionCreate()
      const error = await shouldThrow(() => connection.sendInviteAction({goal_code: 'automotive.inspect.tire'}))
      assert.equal(error.vcxCode, VCXCode.NOT_READY)
    })
  })
})
