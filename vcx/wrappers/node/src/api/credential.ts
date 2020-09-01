import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { ISerializedData } from './common'
import { Connection } from './connection'
import { VCXBaseWithState } from './vcx-base-with-state'
import { PaymentManager } from './vcx-payment-txn'

/**
 *    The object of the VCX API representing a Holder side in the credential issuance process.
 *    Assumes that pairwise connection between Issuer and Holder is already established.
 *
 *    # State
 *
 *    The set of object states and transitions depends on communication method is used.
 *    The communication method can be specified as config option on one of *_init function. The default communication method us `proprietary`.
 *
 *        proprietary:
 *            VcxStateType::VcxStateRequestReceived - once `vcx_credential_create_with_offer` (create Credential object) is called.
 *
 *            VcxStateType::VcxStateOfferSent - once `vcx_credential_send_request` (send `CRED_REQ` message) is called.
 *
 *            VcxStateType::VcxStateAccepted - once `CRED` messages is received.
 *                                             use `vcx_credential_update_state` or `vcx_credential_update_state_with_message` functions for state updates.
 *
 *        aries:
 *            VcxStateType::VcxStateRequestReceived - once `vcx_credential_create_with_offer` (create Credential object) is called.
 *
 *            VcxStateType::VcxStateOfferSent - once `vcx_credential_send_request` (send `CredentialRequest` message) is called.
 *
 *            VcxStateType::VcxStateAccepted - once `Credential` messages is received.
 *            VcxStateType::None - 1) once `ProblemReport` messages is received.
 *                                    use `vcx_credential_update_state` or `vcx_credential_update_state_with_message` functions for state updates.
 *                                 2) once `vcx_credential_reject` is called.
 *
 *        # Transitions
 *
 *        proprietary:
 *            VcxStateType::None - `vcx_credential_create_with_offer` - VcxStateType::VcxStateRequestReceived
 *
 *            VcxStateType::VcxStateRequestReceived - `vcx_credential_send_request` - VcxStateType::VcxStateOfferSent
 *
 *            VcxStateType::VcxStateOfferSent - received `CRED` - VcxStateType::VcxStateAccepted
 *
 *        aries: RFC - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential
 *            VcxStateType::None - `vcx_credential_create_with_offer` - VcxStateType::VcxStateRequestReceived
 *
 *            VcxStateType::VcxStateRequestReceived - `vcx_issuer_send_credential_offer` - VcxStateType::VcxStateOfferSent
 *            VcxStateType::VcxStateRequestReceived - `vcx_credential_reject` - VcxStateType::None
 *
 *            VcxStateType::VcxStateOfferSent - received `Credential` - VcxStateType::VcxStateAccepted
 *            VcxStateType::VcxStateOfferSent - received `ProblemReport` - VcxStateType::None
 *            VcxStateType::VcxStateOfferSent - `vcx_credential_reject` - VcxStateType::None
 *
 *        # Messages
 *
 *        proprietary:
 *            CredentialOffer (`CRED_OFFER`)
 *            CredentialRequest (`CRED_REQ`)
 *            Credential (`CRED`)
 *
 *        aries:
 *            CredentialProposal - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential#propose-credential
 *            CredentialOffer - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential#offer-credential
 *            CredentialRequest - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential#request-credential
 *            Credential - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0036-issue-credential#issue-credential
 *            ProblemReport - https://github.com/hyperledger/aries-rfcs/tree/7b6b93acbaf9611d3c892c4bada142fe2613de6e/features/0035-report-problem#the-problem-report-message-type
 *            Ack - https://github.com/hyperledger/aries-rfcs/tree/master/features/0015-acks#explicit-acks
 */

export interface ICredentialStructData {
  source_id: string,
}

export type ICredentialOffer = [ object, object ]

/**
 * @description Interface that represents the parameters for `Credential.create` function.
 * @interface
 */
export interface ICredentialCreateWithOffer {
  // Institution's personal identification for the credential, should be unique.
  sourceId: string,
  // Credential offer received via "getOffers"
  offer: string,
  // Pairwise connection object with the issuer.
  connection: Connection
}

/**
 * @description Interface that represents the parameters for `Credential.createWithMsgId` function.
 * @interface
 */
export interface ICredentialCreateWithMsgId {
  // Institution's personal identification for the credential, should be unique.
  sourceId: string,
  // Id of the message that contains the credential offer
  msgId: string,
  // Connection to query for credential offer
  connection: Connection
}

/**
 * @description Interface that represents the parameters for `Credential.sendRequest` function.
 * @interface
 */
export interface ICredentialSendData {
  // Connection to send credential request
  connection: Connection,
  // Fee amount
  payment: number
}

/**
 * @description Interface that represents the parameters for `Credential.reject` function.
 * @interface
 */
export interface ICredentialRejectData {
  // Connection to send credential rejection
  connection: Connection,
  // human-friendly message to insert into Reject message.
  comment?: string
}

export interface ICredentialGetRequestMessageData {
  // Use Connection api (vcx_connection_get_pw_did) with specified connection_handle to retrieve your pw_did
  myPwDid: string,
  // Use Connection api (vcx_connection_get_their_pw_did) with specified connection_handle to retrieve their pw_did
  theirPwDid?: string,
  // Fee amount
  payment: number
}

// tslint:disable max-classes-per-file
export class CredentialPaymentManager extends PaymentManager {
  protected _getPaymentTxnFn = rustAPI().vcx_credential_get_payment_txn
}

/**
 * A Credential Object, which is issued by the issuing party to the prover and stored in the prover's wallet.
 */
export class Credential extends VCXBaseWithState<ICredentialStructData> {
  /**
   * Creates a credential with an offer.
   *
   * * Requires a credential offer to be submitted to prover.
   *
   * ```
   * credentialOffer = [
   *   {
   *     claim_id: 'defaultCredentialId',
   *     claim_name: 'Credential',
   *     cred_def_id: 'id',
   *     credential_attrs: {
   *     address1: ['101 Tela Lane'],
   *     address2: ['101 Wilson Lane'],
   *     city: ['SLC'],
   *     state: ['UT'],
   *     zip: ['87121']
   *   },
   *   from_did: '8XFh8yBzrpJQmNyZzgoTqB',
   *   libindy_offer: '{}',
   *   msg_ref_id: '123',
   *   msg_type: 'CLAIM_OFFER',
   *   schema_seq_no: 1487,
   *   to_did: '8XFh8yBzrpJQmNyZzgoTqB',
   *   version: '0.1'
   * },
   * {
   *   payment_addr: 'pov:null:OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j',
   *   payment_required: 'one-time',
   *   price: 5
   * }]
   *
   * {
   *   JSON.stringify(credentialOffer),
   *   'testCredentialSourceId'
   * }
   * credential = Credential.create(data)
   * ```
   *
   */
  public static async create ({ sourceId, offer }: ICredentialCreateWithOffer): Promise<Credential> {
    const credential = new Credential(sourceId)
    try {
      await credential._create((cb) => rustAPI().vcx_credential_create_with_offer(
        0,
        sourceId,
        offer,
        cb
        )
      )
      return credential
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Create a Credential object based off of a known message id for a given connection.
   *
   * ```
   * credential = Credential.createWithMsgId({
   *   connection,
   *   msgId: 'testCredentialMsgId',
   *   sourceId: 'testCredentialSourceId'
   * })
   * ```
   */
  public static async createWithMsgId (
    { connection, sourceId, msgId }: ICredentialCreateWithMsgId
  ): Promise<Credential> {
    try {
      return await createFFICallbackPromise<Credential>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_create_with_msgid(0, sourceId, connection.handle, msgId, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'uint32', 'string'],
            (xHandle: number, err: number, handleNum: number, credOffer: string) => {
              if (err) {
                reject(err)
                return
              }
              const newObj = new Credential(sourceId)
              newObj._setHandle(handleNum)
              newObj._credOffer = credOffer
              resolve(newObj)
            })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Accept credential for the given offer.
   *
   * This function performs the following actions:
   * 1. Creates Credential state object that requests and receives a credential for an institution.
   *    (equal to `Credential.create` function).
   * 2. Prepares Credential Request and replies to the issuer.
   *    (equal to `credential.sendRequest` function)
   *
   * ```
   * credential = Credential.acceptOffer({
   *   connection,
   *   msgId: 'testCredentialMsgId',
   *   sourceId: 'testCredentialSourceId'
   * })
   * ```
   */
  public static async acceptOffer ({ sourceId, offer, connection }: ICredentialCreateWithOffer): Promise<Credential> {
    try {
      return await createFFICallbackPromise<Credential>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_accept_credential_offer(0, sourceId, offer, connection.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'uint32', 'string'],
            (xHandle: number, err: number, credentialHandle: number, credentialSerialized: string) => {
              if (err) {
                reject(err)
                return
              }
              const credential = new Credential(sourceId)
              credential._setHandle(credentialHandle)
              credential._serialized = credentialSerialized
              resolve(credential)
            })
      )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Create an object from a JSON Structured data produced from the objects serialize method
   *
   * ```
   * data = credential.deserialize()
   * ```
   */
  public static async deserialize (credentialData: ISerializedData<ICredentialStructData>) {
    const credential = await super._deserialize<Credential, {}>(Credential, credentialData)
    return credential
  }
  /**
   * Retrieves all pending credential offers.
   *
   * ```
   * connection = await Connection.create({id: 'foobar'})
   * inviteDetails = await connection.connect()
   * offers = await Credential.getOffers(connection)
   * ```
   */
  public static async getOffers (connection: Connection): Promise<ICredentialOffer[]> {
    try {
      const offersStr = await createFFICallbackPromise<string>(
        (resolve, reject, cb) => {
          const rc = rustAPI().vcx_credential_get_offers(0, connection.handle, cb)
          if (rc) {
            reject(rc)
          }
        },
        (resolve, reject) => Callback(
          'void',
          ['uint32', 'uint32', 'string'],
          (handle: number, err: number, messages: string) => {
            if (err) {
              reject(err)
              return
            }
            resolve(messages)
          })
      )
      const offers: ICredentialOffer[] = JSON.parse(offersStr)
      return offers
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  public paymentManager!: CredentialPaymentManager
  protected _releaseFn = rustAPI().vcx_credential_release
  protected _updateStFn = rustAPI().vcx_credential_update_state
  protected _updateStWithMessageFn = rustAPI().vcx_credential_update_state_with_message
  protected _getStFn = rustAPI().vcx_credential_get_state
  protected _serializeFn = rustAPI().vcx_credential_serialize
  protected _deserializeFn = rustAPI().vcx_credential_deserialize
  protected _credOffer: string = ''
  protected _serialized: string = ''

  /**
   * Approves the credential offer and submits a credential request.
   * The result will be a credential stored in the prover's wallet.
   *
   * ```
   * connection = await Connection.create({id: 'foobar'})
   * inviteDetails = await connection.connect()
   * credential = Credential.create(data)
   * await credential.sendRequest({ connection, 1000 })
   * ```
   *
   */
  public async sendRequest ({ connection, payment }: ICredentialSendData): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_send_request(0, this.handle, connection.handle, payment, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle: number, err: number) => {
            if (err) {
              reject(err)
              return
            }
            resolve()
          })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }
  /**
   * Gets the credential request message for sending to the specified connection.
   *
   * ```
   * connection = await Connection.create({id: 'foobar'})
   * inviteDetails = await connection.connect()
   * credential = Credential.create(data)
   * await credential.getRequestMessage({ '44x8p4HubxzUK1dwxcc5FU', 1000 })
   * ```
   *
   */
  public async getRequestMessage ({ myPwDid, theirPwDid, payment }: ICredentialGetRequestMessageData): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_get_request_msg(0, this.handle, myPwDid, theirPwDid, payment, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (xHandle: number, err: number, message: string) => {
              if (err) {
                reject(err)
                return
              }
              if (!message) {
                reject(`Credential ${this.sourceId} returned empty string`)
                return
              }
              resolve(message)
            })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Retrieve information about a stored credential in user's wallet,
   * including credential id and the credential itself.
   *
   * ```
   * credential = Credential.create(data)
   * await credential.getCredential()
   * ```
   *
   */
  public async getCredential (): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_get_credential(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (xHandle: number, err: number, message: string) => {
              if (err) {
                reject(err)
                return
              }
              if (!message) {
                reject(`Credential ${this.sourceId} returned empty string`)
                return
              }
              resolve(message)
            })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  get credOffer (): string {
    return this._credOffer
  }
  /**
   * Retrieve Payment Transaction Information for this Credential. Typically this will include
   * how much payment is requried by the issuer, which needs to be provided by the prover, before
   * the issuer will issue the credential to the prover. Ideally a prover would want to know
   * how much payment is being asked before submitting the credential request (which triggers
   * the payment to be made).
   * ```
   * EXAMPLE HERE
   * ```
   */
  public async getPaymentInfo (): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_get_payment_info(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32', 'string'],
          (xcommandHandle: number, err: number, info: any) => {
            if (err) {
              reject(err)
            } else {
              resolve(info)
            }
          })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Send a Credential rejection to the connection.
   * It can be called once Credential Offer or Credential messages are received.
   *
   * Note that this function can be used for `aries` communication protocol.
   * In other cases it returns ActionNotSupported error.
   *
   * ```
   * connection = await Connection.create({id: 'foobar'})
   * inviteDetails = await connection.connect()
   * credential = Credential.create(data)
   * await credential.reject({ connection, 'Foo' })
   * ```
   *
   */
  public async reject ({ connection, comment }: ICredentialRejectData): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_reject(0, this.handle, connection.handle, comment, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle: number, err: number) => {
            if (err) {
              reject(err)
              return
            }
            resolve()
          })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Delete a Credential associated with the state object from the Wallet and release handle of the state object.
   *
   * ```
   * credential = Credential.create(data)
   * await credential.delete()
   * ```
   */
  public async delete (): Promise<void> {
    try {
      await createFFICallbackPromise<void>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_delete_credential(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback('void', ['uint32', 'uint32'], (xcommandHandle: number, err: number) => {
            if (err) {
              reject(err)
              return
            }
            resolve()
          })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  /**
   * Build Presentation Proposal message for revealing Credential data.
   *
   * Presentation Proposal is an optional message that can be sent by the Prover to the Verifier to 
   * initiate a Presentation Proof process.
   *
   * Presentation Proposal Format:
   *   https://github.com/hyperledger/aries-rfcs/tree/master/features/0037-present-proof#propose-presentation
   *
   * EXPERIMENTAL
   *
   * ```
   * credential = Credential.create(data)
   * await credential.getPresentationProposal()
   * ```
   *
   */
  public async getPresentationProposal (): Promise<string> {
    try {
      return await createFFICallbackPromise<string>(
          (resolve, reject, cb) => {
            const rc = rustAPI().vcx_credential_get_presentation_proposal_msg(0, this.handle, cb)
            if (rc) {
              reject(rc)
            }
          },
          (resolve, reject) => Callback(
            'void',
            ['uint32', 'uint32', 'string'],
            (xHandle: number, err: number, message: string) => {
              if (err) {
                reject(err)
                return
              }
              if (!message) {
                reject(`Credential ${this.sourceId} returned empty string`)
                return
              }
              resolve(message)
            })
        )
    } catch (err) {
      throw new VCXInternalError(err)
    }
  }

  protected _setHandle (handle: number) {
    super._setHandle(handle)
    this.paymentManager = new CredentialPaymentManager({ handle })
  }
}
