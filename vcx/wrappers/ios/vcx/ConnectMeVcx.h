//
//  init.h
//  vcx
//
//  Created by GuestUser on 4/30/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#ifndef init_h
#define init_h
#import "libvcx.h"

extern void VcxWrapperCommonCallback(vcx_command_handle_t xcommand_handle,
                                     vcx_error_t err);

extern void VcxWrapperCommonHandleCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           vcx_command_handle_t pool_handle);

extern void VcxWrapperCommonStringCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           const char *const arg1);

extern void VcxWrapperCommonBoolCallback(vcx_command_handle_t xcommand_handle,
                                         vcx_error_t err,
                                         unsigned int arg1);

extern void VcxWrapperCommonStringStringCallback(vcx_command_handle_t xcommand_handle,
                                                 vcx_error_t err,
                                                 const char *const arg1,
                                                 const char *const arg2);

extern void VcxWrapperCommonStringOptStringCallback(vcx_command_handle_t xcommand_handle,
                                                    vcx_error_t err,
                                                    const char *const arg1,
                                                    const char *const arg2);

extern void VcxWrapperCommonDataCallback(vcx_command_handle_t xcommand_handle,
                                         vcx_error_t err,
                                         const uint8_t *const arg1,
                                         uint32_t arg2);

extern void VcxWrapperCommonStringStringStringCallback(vcx_command_handle_t xcommand_handle,
                                                       vcx_error_t err,
                                                       const char *const arg1,
                                                       const char *const arg2,
                                                       const char *const arg3);

extern void VcxWrapperCommonStringDataCallback(vcx_command_handle_t xcommand_handle,
                                               vcx_error_t err,
                                               const char *const arg1,
                                               const uint8_t *const arg2,
                                               uint32_t arg3);

extern void VcxWrapperCommonNumberCallback(vcx_command_handle_t xcommand_handle,
                                           vcx_error_t err,
                                           vcx_command_handle_t handle);

extern void VcxWrapperCommonStringOptStringOptStringCallback(vcx_command_handle_t xcommand_handle,
                                                             vcx_error_t err,
                                                             const char *const arg1,
                                                             const char *const arg2,
                                                             const char *const arg3);

extern void VcxWrapperCommonStringStringLongCallback(vcx_command_handle_t xcommand_handle,
                                                     vcx_error_t err,
                                                     const char *arg1,
                                                     const char *arg2,
                                                     unsigned long long arg3);

extern void VcxWrapperCommonNumberStringCallback(vcx_command_handle_t xcommand_handle,
                                                 vcx_error_t err,
                                                 vcx_command_handle_t handle,
                                                 const char *const arg2);

@interface ConnectMeVcx : NSObject

- (int)initSovToken;

//- (int)initNullPay;

- (void)initWithConfig:(NSString *)config
            completion:(void (^)(NSError *error))completion;

- (void)initPool:(NSString *)poolConfig
            completion:(void (^)(NSError *error))completion;

- (void)agentProvisionAsync:(NSString *)config
                 completion:(void (^)(NSError *error, NSString *config))completion;

/// Provision an agent in the agency, populate configuration and wallet for this agent.
///
/// config: Configuration JSON. See: https://github.com/evernym/mobile-sdk/blob/master/docs/Configuration.md#agent-provisioning-options
/// token: {
///          This can be a push notification endpoint to contact the sponsee or
///          an id that the sponsor uses to reference the sponsee in its backend system
///          "sponseeId": String,
///          "sponsorId": String, //Persistent Id of the Enterprise sponsoring the provisioning
///          "nonce": String,
///          "timestamp": String,
///          "sig": String, // Base64Encoded(sig(nonce + timestamp + id))
///          "sponsorVerKey": String,
///          "attestationAlgorithm": Optional<String>, // device attestation signature algorithm. Can be one of: SafetyNet | DeviceCheck
///          "attestationData": Optional<String>, // device attestation signature matching to specified algorithm
///        }
///
/// #Returns
/// Configuration
- (const char *)agentProvisionWithToken:(NSString *)config
                          token:(NSString *)token;

/// Get token which can be used for provisioning an agent
/// NOTE: Can be used only for Evernym's applications
///
/// config:
/// {
///     vcx_config: VcxConfig // Same config passed to agent provision
///                           // See: https://github.com/evernym/mobile-sdk/blob/master/docs/Configuration.md#agent-provisioning-options
///     sponsee_id: String,
///     sponsor_id: String,
///     com_method: {
///         type: u32 // 1 means push notifications, 4 means forward to sponsor app
///         id: String,
///         value: String,
///     },
/// }
///
/// #Returns
/// token: provisioning token as JSON
- (void)getProvisionToken:(NSString *)config
                 completion:(void (^)(NSError *error, NSString *token))completion;

- (void)connectionCreateWithInvite:(NSString *)invitationId
                     inviteDetails:(NSString *)inviteDetails
                        completion:(void (^)(NSError *error, NSInteger connectionHandle))completion;

- (void)connectionCreateOutofband:(NSString *)sourceId
                         goalCode:(NSString *)goalCode
                             goal:(NSString *)goal
                        handshake:(BOOL *)handshake
                    requestAttach:(NSString *)requestAttach
                       completion:(void (^)(NSError *error, NSInteger connectionHandle))completion;

- (void)acceptConnectionWithInvite:(NSString *)invitationId
                     inviteDetails:(NSString *)inviteDetails
                    connectionType:(NSString *)connectionType
                        completion:(void (^)(NSError *error, NSInteger connectionHandle, NSString *serializedConnection))completion;

- (void)connectionCreateWithOutofbandInvite:(NSString *)invitationId
                                     invite:(NSString *)invite
                                 completion:(void (^)(NSError *error, NSInteger connectionHandle))completion;

- (void)connectionConnect:(VcxHandle)connectionHandle
           connectionType:(NSString *)connectionType
               completion:(void (^)(NSError *error, NSString *inviteDetails))completion;

- (void)connectionGetState:(NSInteger)connectionHandle
                completion:(void (^)(NSError *error, NSInteger state))completion;

- (void)connectionUpdateState:(NSInteger) connectionHandle
                   completion:(void (^)(NSError *error, NSInteger state))completion;

- (void)connectionSerialize:(NSInteger)connectionHandle
                 completion:(void (^)(NSError *error, NSString *serializedConnection))completion;

- (void)connectionDeserialize:(NSString *)serializedConnection
                   completion:(void (^)(NSError *error, NSInteger connectionHandle))completion;

- (int)connectionRelease:(NSInteger) connectionHandle;

- (void)deleteConnection:(VcxHandle)connectionHandle
          withCompletion:(void (^)(NSError *error))completion;

- (void)connectionSendMessage:(VcxHandle)connectionHandle
                  withMessage:(NSString *)message
       withSendMessageOptions:(NSString *)sendMessageOptions
               withCompletion:(void (^)(NSError *error, NSString *msg_id))completion;

- (void)connectionSendPing:(VcxHandle)connectionHandle
                   comment:(NSString *)comment
            withCompletion:(void (^)(NSError *error))completion;

- (void)connectionSendReuse:(VcxHandle)connectionHandle
                     invite:(NSString *)invite
             withCompletion:(void (^)(NSError *error))completion;

- (void)connectionSendAnswer:(VcxHandle)connectionHandle
                    question:(NSString *)question
                      answer:(NSString *)answer
             withCompletion:(void (^)(NSError *error))completion;

- (void)connectionSendInviteAction:(VcxHandle)connectionHandle
                              data:(NSString *)data
                    withCompletion:(void (^)(NSError *error, NSString *message))completion;

- (void)connectionSignData:(VcxHandle)connectionHandle
                  withData:(NSData *)dataRaw
            withCompletion:(void (^)(NSError *error, NSData *signature_raw, vcx_u32_t signature_len))completion;

- (void)connectionVerifySignature:(VcxHandle)connectionHandle
                         withData:(NSData *)dataRaw
                withSignatureData:(NSData *)signatureRaw
                   withCompletion:(void (^)(NSError *error, vcx_bool_t valid))completion;

- (void)connectionUpdateState:(VcxHandle) connectionHandle
               withCompletion:(void (^)(NSError *error, NSInteger state))completion;

- (void)connectionUpdateStateWithMessage:(VcxHandle) connectionHandle
                                 message:(NSString *)message
                          withCompletion:(void (^)(NSError *error, NSInteger state))completion;

- (void)connectionGetState:(VcxHandle) connectionHandle
            withCompletion:(void (^)(NSError *error, NSInteger state))completion;

- (void)connectionGetProblemReport:(NSInteger) connectionHandle
                        completion:(void (^)(NSError *error, NSString *message))completion;

- (void)connectionCreateInvite:(NSString *)sourceId
             completion:(void (^)(NSError *error, NSInteger connectionHandle)) completion;

- (void) getConnectionInviteDetails:(NSInteger) connectionHandle
                        abbreviated:(BOOL *) abbreviated
         withCompletion:(void (^)(NSError *error, NSString *inviteDetails))completion;

- (void)agentUpdateInfo:(NSString *)config
             completion:(void (^)(NSError *error))completion;

- (void)getCredential:(NSInteger )credentailHandle
           completion:(void (^)(NSError *error, NSString *credential))completion;

- (void)deleteCredential:(NSInteger )credentialHandle
              completion:(void (^)(NSError *error))completion;

- (void)credentialCreateWithOffer:(NSString *)sourceId
                            offer:(NSString *)credentialOffer
                       completion:(void (^)(NSError *error, NSInteger credentailHandle))completion;

- (void)credentialCreateWithMsgid:(NSString *)sourceId
                 connectionHandle:(VcxHandle)connectionHandle
                            msgId:(NSString *)msgId
                       completion:(void (^)(NSError *error, NSInteger credentialHandle, NSString *credentialOffer))completion;

- (void)credentialAcceptCredentialOffer:(NSString *)sourceId
                                  offer:(NSString *)credentialOffer
                       connectionHandle:(VcxHandle)connectionHandle
                             completion:(void (^)(NSError *error, NSInteger credentialHandle, NSString *credentialSerialized))completion;

- (void)credentialSendRequest:(NSInteger)credentialHandle
             connectionHandle:(VcxHandle)connectionHandle
                paymentHandle:(vcx_payment_handle_t)paymentHandle
                   completion:(void (^)(NSError *error))completion;

- (void)credentialGetState:(NSInteger )credentialHandle
                completion:(void (^)(NSError *error, NSInteger state))completion;

- (void)credentialUpdateState:(NSInteger )credentailHandle
                completion:(void (^)(NSError *error, NSInteger state))completion;

- (void)credentialUpdateStateWithMessage:(VcxHandle) credentialHandle
                                 message:(NSString *)message
                          withCompletion:(void (^)(NSError *error, NSInteger state))completion;

- (void)credentialGetOffers:(VcxHandle)connectionHandle
                 completion:(void (^)(NSError *error, NSString *offers))completion;

- (void)credentialReject:(NSInteger)credentialHandle
        connectionHandle:(VcxHandle)connectionHandle
                 comment:(NSString *)comment
              completion:(void (^)(NSError *error))completion;

- (void)credentialGetPresentationProposal:(NSInteger )credentialHandle
                               completion:(void (^)(NSError *error, NSString *presentationProposal))completion;

- (void)credentialSerialize:(NSInteger)credentialHandle
                 completion:(void (^)(NSError *error, NSString *state))completion;

- (void)credentialDeserialize:(NSString *)serializedCredential
                   completion:(void (^)(NSError *error, NSInteger credentialHandle))completion;

- (void)credentialGetProblemReport:(NSInteger) credentialHandle
                        completion:(void (^)(NSError *error, NSString *message))completion;

- (int)credentialRelease:(NSInteger) credentialHandle;

- (void)exportWallet:(NSString *)exportPath
         encryptWith:(NSString *)encryptionKey
          completion:(void (^)(NSError *error, NSInteger exportHandle))completion;

- (void)importWallet:(NSString *)config
           completion:(void (^)(NSError *error))completion;

- (void)addRecordWallet:(NSString *)recordType
            recordId:(NSString *)recordId
            recordValue:(NSString *) recordValue
           completion:(void (^)(NSError *error))completion;

- (void)updateRecordWallet:(NSString *)recordType
              withRecordId:(NSString *)recordId
           withRecordValue:(NSString *) recordValue
            withCompletion:(void (^)(NSError *error))completion;

- (void)getRecordWallet:(NSString *)recordType
               recordId:(NSString *)recordId
             completion:(void (^)(NSError *error, NSString *walletValue))completion;

- (void)deleteRecordWallet:(NSString *)recordType
            recordId:(NSString *)recordId
           completion:(void (^)(NSError *error))completion;

- (void) proofGetRequests:(NSInteger)connectionHandle
              completion:(void (^)(NSError *error, NSString *requests))completion;

- (void) proofRetrieveCredentials:(vcx_proof_handle_t)proofHandle
                   withCompletion:(void (^)(NSError *error, NSString *matchingCredentials))completion;

- (void) proofGenerate:(vcx_proof_handle_t)proofHandle
withSelectedCredentials:(NSString *)selectedCredentials
 withSelfAttestedAttrs:(NSString *)selfAttestedAttributes
        withCompletion:(void (^)(NSError *error))completion;

- (void) proofCreateWithMsgId:(NSString *)source_id
         withConnectionHandle:(vcx_connection_handle_t)connectionHandle
                    withMsgId:(NSString *)msgId
               withCompletion:(void (^)(NSError *error, vcx_proof_handle_t proofHandle, NSString *proofRequest))completion;

- (void) requestProof:(vcx_proof_handle_t)proof_handle
 withConnectionHandle:(vcx_connection_handle_t)connection_handle
       requestedAttrs:(NSString *)requestedAttrs
  requestedPredicates:(NSString *)requestedPredicates
            proofName:(NSString *)proofName
   revocationInterval:(NSString *)revocationInterval
       withCompletion:(void (^)(NSError *error))completion;

- (void)proofGetPresentationProposal:(vcx_proof_handle_t)proof_handle
                          completion:(void (^)(NSError *error, NSString *presentationProposal))completion;

- (void) proofSend:(vcx_proof_handle_t)proof_handle
withConnectionHandle:(vcx_connection_handle_t)connection_handle
    withCompletion:(void (^)(NSError *error))completion;

- (void) proofSendProposal:(vcx_proof_handle_t)proof_handle
      withConnectionHandle:(vcx_connection_handle_t)connection_handle
            withCompletion:(void (^)(NSError *error))completion;

- (void)proofGetState:(NSInteger)proofHandle
           completion:(void (^)(NSError *error, NSInteger state))completion;

- (void)proofUpdateState:(NSInteger) proofHandle
              completion:(void (^)(NSError *error, NSInteger state))completion;

- (void) proofReject: (vcx_proof_handle_t)proof_handle
      withConnectionHandle:(vcx_connection_handle_t)connection_handle
      withCompletion: (void (^)(NSError *error))completion;

- (void) proofDeclinePresentationRequest:(vcx_proof_handle_t)proof_handle
                    withConnectionHandle:(vcx_connection_handle_t)connection_handle
                              withReason:(NSString *)reason
                            withProposal:(NSString *)proposal
                          withCompletion:(void (^)(NSError *error))completion;

- (void) getProofMsg:(vcx_proof_handle_t) proofHandle
      withCompletion:(void (^)(NSError *error, NSString *proofMsg))completion;

- (void) getRejectMsg:(vcx_proof_handle_t) proofHandle
       withCompletion:(void (^)(NSError *error, NSString *rejectMsg))completion;

- (void)connectionRedirect:(vcx_connection_handle_t)redirect_connection_handle
      withConnectionHandle:(vcx_connection_handle_t)connection_handle
            withCompletion:(void (^)(NSError *error))completion;

- (void)getRedirectDetails:(vcx_connection_handle_t)connection_handle
      withCompletion:(void (^)(NSError *error, NSString *redirectDetails))completion;

- (void) proofCreateWithRequest:(NSString *) source_id
               withProofRequest:(NSString *) proofRequest
                 withCompletion:(void (^)(NSError *error, vcx_proof_handle_t proofHandle))completion;

- (void) proofCreateProposal:(NSString *) source_id
           withProofProposal:(NSString *) proofProposal
                 withComment:(NSString *) comment
              withCompletion:(void (^)(NSError *error, vcx_proof_handle_t proofHandle))completion;

- (void) proofSerialize:(vcx_proof_handle_t) proofHandle
         withCompletion:(void (^)(NSError *error, NSString *proof_request))completion;

- (void) proofDeserialize:(NSString *) serializedProof
           withCompletion:(void (^)(NSError *error, vcx_proof_handle_t proofHandle)) completion;

- (void)proofUpdateStateWithMessage:(VcxHandle) proofHandle
                            message:(NSString *)message
                     withCompletion:(void (^)(NSError *error, NSInteger state))completion;

- (void)proofGetProblemReport:(VcxHandle) proofHandle
                   completion:(void (^)(NSError *error, NSString *message))completion;

- (int)proofRelease:(NSInteger) proofHandle;

- (int)vcxShutdown:(BOOL *)deleteWallet;

- (void)createPaymentAddress:(NSString *)seed
              withCompletion:(void (^)(NSError *error, NSString *address))completion;

- (void)getTokenInfo:(vcx_payment_handle_t)payment_handle
      withCompletion:(void (^)(NSError *error, NSString *tokenInfo))completion;

- (void)sendTokens:(vcx_payment_handle_t)payment_handle
        withTokens:(NSString *)tokens
     withRecipient:(NSString *)recipient
    withCompletion:(void (^)(NSError *error, NSString *recipient))completion;

- (void)downloadMessages:(NSString *)messageStatus
                    uid_s:(NSString *)uid_s
                  pwdids:(NSString *)pwdids
              completion:(void (^)(NSError *error, NSString* messages))completion;

- (void)downloadMessage:(NSString *)uid
             completion:(void (^)(NSError *error, NSString* message))completion;

- (void)updateMessages:(NSString *)messageStatus
            pwdidsJson:(NSString *)pwdidsJson
            completion:(void (^)(NSError *error))completion;

- (void)downloadAgentMessages:(NSString *)messageStatus
                    uid_s:(NSString *)uid_s
                    completion:(void (^)(NSError *error, NSString* messages))completion;

- (void) getLedgerFees:(void(^)(NSError *error, NSString *fees)) completion;

- (void) getTxnAuthorAgreement:(void(^)(NSError *error, NSString *authorAgreement)) completion;

- (vcx_error_t) activateTxnAuthorAgreement:(NSString *)text
                               withVersion:(NSString *)version
                                  withHash:(NSString *)hash
                             withMechanism:(NSString *)mechanism
                             withTimestamp:(long)timestamp;

/**
 Fetch and Cache public entities from the Ledger associated with stored in the wallet credentials.
 This function performs two steps:
     1) Retrieves the list of all credentials stored in the opened wallet.
     2) Fetch and cache Schemas / Credential Definitions / Revocation Registry Definitions
        correspondent to received credentials from the connected Ledger.

 This helper function can be used, for instance as a background task, to refresh library cache.
 This allows us to reduce the time taken for Proof generation by using already cached entities instead of queering the Ledger.

 NOTE: Library must be already initialized (wallet and pool must be opened).

 Returns: void
*/

- (void)fetchPublicEntities:(void (^)(NSError *error))completion;

/**
 This function allows you to check the health of LibVCX and EAS/CAS instance.
 It will return error in case of any problems on EAS or will resolve pretty long if VCX is thread-hungry.
 WARNING: this call may take a lot of time returning answer in case of load, be careful.
 NOTE: Library must be initialized, ENDPOINT_URL should be set

 Returns: void
*/

- (void)healthCheck:(void (^)(NSError *error))completion;

- (void) createWalletBackup:(NSString *)sourceID
                  backupKey:(NSString *)backupKey
                 completion:(void (^)(NSError *error, NSInteger walletBackupHandle))completion;

- (void) backupWalletBackup:(vcx_wallet_backup_handle_t) walletBackupHandle
                       path:(NSString *)path
                 completion:(void(^)(NSError *error))completion;

- (void) updateWalletBackupState:(vcx_wallet_backup_handle_t) walletBackupHandle
                      completion:(void (^)(NSError *error, NSInteger state))completion;

- (void) updateWalletBackupStateWithMessage:(vcx_wallet_backup_handle_t) walletBackupHandle
                                    message:(NSString *)message
                                 completion:(void (^)(NSError *error, NSInteger state))completion;

// should *walletBackupStr be just data here?
- (void) serializeBackupWallet:(vcx_wallet_backup_handle_t) walletBackupHandle
              completion:(void (^)(NSError *error, NSString *data))completion;

- (void) deserializeBackupWallet:(NSString *) walletBackupStr
                completion:(void (^)(NSError *error, NSInteger walletBackupHandle))completion;


- (void)restoreWallet:(NSString *)config
           completion:(void (^)(NSError *error))completion;

/*
* Verifier API
*/

/// Create a new Proof object that requests a proof for an enterprise
///
/// #Params
/// sourceId: Enterprise's personal identification for the proof, should be unique.
///
/// proofName: Proof name
///
/// requestedAttributes: Describes requested attribute
///     [{
///         "name": Optional<string>, // attribute name, (case insensitive and ignore spaces)
///         "names": Optional<[string, string]>, // attribute names, (case insensitive and ignore spaces)
///                                              // NOTE: should either be "name" or "names", not both and not none of them.
///                                              // Use "names" to specify several attributes that have to match a single credential.
///         "restrictions":  Optional<wql query> - set of restrictions applying to requested credentials. (see below)
///         "non_revoked": {
///             "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///             "to": Optional<(u64)>
///                 //Requested time represented as a total number of seconds from Unix Epoch, Optional
///         }
///     }]
///
/// # Example requested_attrs -> "[{"name":"attrName","restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
/// requestedPredicates: predicate specifications prover must provide claim for
///          [{ // set of requested predicates
///             "name": attribute name, (case insensitive and ignore spaces)
///             "p_type": predicate type (Currently ">=" only)
///             "p_value": int predicate value
///             "restrictions":  Optional<wql query> -  set of restrictions applying to requested credentials. (see below)
///             "non_revoked": Optional<{
///                 "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///                 "to": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///             }>
///          }]
///
/// # Example requested_predicates -> "[{"name":"attrName","p_type":"GE","p_value":9,"restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
/// revocationInterval:  Optional<<revocation_interval>>, // see below,
///                        // If specified, prover must proof non-revocation
///                        // for date in this interval for each attribute
///                        // (can be overridden on attribute level)
///     from: Optional<u64> // timestamp of interval beginning
///     to: Optional<u64> // timestamp of interval beginning
///         // Requested time represented as a total number of seconds from Unix Epoch, Optional
/// # Examples config ->  "{}" | "{"to": 123} | "{"from": 100, "to": 123}"
///
/// #Returns
/// Handle pointing to Proof Verifier object
- (void) createProofVerifier:(NSString *)sourceId
         requestedAttributes:(NSString *)requestedAttributes
         requestedPredicates:(NSString *)requestedPredicates
          revocationInterval:(NSString *)revocationInterval
                   proofName:(NSString *)proofName
                  completion:(void (^)(NSError *error, NSInteger handle))completion;

/// Create a new Proof object based on the given Presentation Proposal message
///
/// #Params
/// sourceId: Enterprise's personal identification for the proof, should be unique..
///
/// name: Proof name
///
/// presentationProposal: Message sent by the Prover to the verifier to initiate a proof presentation process:
///     {
///         "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/present-proof/1.0/propose-presentation",
///         "@id": "<uuid-propose-presentation>",
///         "comment": "some comment",
///         "presentation_proposal": {
///             "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/present-proof/1.0/presentation-preview",
///             "attributes": [
///                 {
///                     "name": "<attribute_name>", - name of the attribute.
///                     "cred_def_id": "<cred_def_id>", - maps to the credential definition identifier of the credential with the current attribute
///                     "mime-type": Optional"<type>", - optional type of value. if mime-type is missing (null), then value is a string.
///                     "value": "<value>", - value of the attribute to reveal in presentation
///                 },
///                 // more attributes
///               ],
///              "predicates": [
///                 {
///                     "name": "<attribute_name>", - name of the attribute.
///                     "cred_def_id": "<cred_def_id>", - maps to the credential definition identifier of the credential with the current attribute
///                     "predicate": "<predicate>", - predicate operator: "<", "<=", ">=", ">"
///                     "threshold": <threshold> - threshold value for the predicate.
///                 },
///                 // more predicates
///             ]
///         }
///     }
///
/// #Returns
/// Handle pointing to Proof Verifier object
- (void) createProofVerifierWithProposal:(NSString *)sourceId
                    presentationProposal:(NSString *)presentationProposal
                                    name:(NSString *)name
                              completion:(void (^)(NSError *error, NSInteger handle))completion;

/// Query the agency for the received messages.
/// Checks for any messages changing state in the object and updates the state attribute.
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// The most current state of the Proof Object
- (void) proofVerifierUpdateState:(NSInteger) proofHandle
                       completion:(void (^)(NSError *error, NSInteger state))completion;

/// Update the state of the proof based on the given message.
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// message: message to process for state changes
///
/// #Returns
/// The most current state of the Proof Object
- (void) proofVerifierUpdateStateWithMessage:(NSInteger)proofHandle
                                    message:(NSString *)message
                                 completion:(void (^)(NSError *error, NSInteger state))completion;

/// Get the current state of the proof object
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// The most current state of the Proof Object
- (void) proofVerifierGetState:(NSInteger) proofHandle
                    completion:(void (^)(NSError *error, NSInteger state))completion;

/// Takes the proof object and returns a json string of all its attributes
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// json string of the state object
- (void) proofVerifierSerialize:(NSInteger) proofHandle
                     completion:(void (^)(NSError *error, NSString* serialized))completion;

/// Takes a json string representing a proof object and recreates an object matching the json
///
/// #Params
/// serialized: json string representing a proof object
///
/// #Returns
/// Handle pointing to Proof Verifier object
- (void) proofVerifierDeserialize:(NSString *) serialized
                       completion:(void (^)(NSError *error, NSInteger proofHandle))completion;

/// Sends a proof request to pairwise connection
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// Null
- (void) proofVerifierSendRequest:(NSInteger) proofHandle
                 connectionHandle:(NSInteger) connectionHandle
                       completion:(void (^)(NSError *error))completion;

/// Request a new proof after receiving a proof proposal (this enables negotiation)
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// connectionHandle: Connection handle that identifies pairwise connection
///
/// requestedAttributes: Describes requested attribute
///     [{
///         "name": Optional<string>, // attribute name, (case insensitive and ignore spaces)
///         "names": Optional<[string, string]>, // attribute names, (case insensitive and ignore spaces)
///                                              // NOTE: should either be "name" or "names", not both and not none of them.
///                                              // Use "names" to specify several attributes that have to match a single credential.
///         "restrictions":  Optional<wql query> - set of restrictions applying to requested credentials. (see below)
///         "non_revoked": {
///             "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///             "to": Optional<(u64)>
///                 //Requested time represented as a total number of seconds from Unix Epoch, Optional
///         }
///     }]
///
/// # Example requested_attrs -> "[{"name":"attrName","restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
/// requestedPredicates: predicate specifications prover must provide claim for
///          [{ // set of requested predicates
///             "name": attribute name, (case insensitive and ignore spaces)
///             "p_type": predicate type (Currently ">=" only)
///             "p_value": int predicate value
///             "restrictions":  Optional<wql query> -  set of restrictions applying to requested credentials. (see below)
///             "non_revoked": Optional<{
///                 "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///                 "to": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///             }>
///          }]
///
/// # Example requested_predicates -> "[{"name":"attrName","p_type":"GE","p_value":9,"restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
/// revocationInterval:  Optional<<revocation_interval>>, // see below,
///                        // If specified, prover must proof non-revocation
///                        // for date in this interval for each attribute
///                        // (can be overridden on attribute level)
///     from: Optional<u64> // timestamp of interval beginning
///     to: Optional<u64> // timestamp of interval beginning
///         // Requested time represented as a total number of seconds from Unix Epoch, Optional
/// # Examples config ->  "{}" | "{"to": 123} | "{"from": 100, "to": 123}"
///
/// #Returns
/// Null
- (void) proofVerifierRequestForProposal:(NSInteger) proofHandle
                        connectionHandle:(NSInteger) connectionHandle
                     requestedAttributes:(NSString *)requestedAttributes
                     requestedPredicates:(NSString *)requestedPredicates
                      revocationInterval:(NSString *)revocationInterval
                               proofName:(NSString *)proofName
                              completion:(void (^)(NSError *error))completion;

/// Get the proof request message that can be sent to the specified connection
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// # Example proof_request -> "{'@topic': {'tid': 0, 'mid': 0}, '@type': {'version': '1.0', 'name': 'PROOF_REQUEST'}, 'proof_request_data': {'name': 'proof_req', 'nonce': '118065925949165739229152', 'version': '0.1', 'requested_predicates': {}, 'non_revoked': None, 'requested_attributes': {'attribute_0': {'name': 'name', 'restrictions': {'$or': [{'issuer_did': 'did'}]}}}, 'ver': '1.0'}, 'thread_id': '40bdb5b2'}"
///
/// #Returns
/// Message as JSON
- (void) proofVerifierGetProofRequestMessage:(NSInteger) proofHandle
                                 completion:(void (^)(NSError *error, NSString* message))completion;

/// Get the proof proposal received for deciding whether to accept it
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// Message as JSON
- (void) proofVerifierGetProofProposalMessage:(NSInteger) proofHandle
                                 completion:(void (^)(NSError *error, NSString* message))completion;

/// Get the proof request attachment that you send along the out of band credential
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to access proof object
///
/// # Example presentation_request_attachment -> "{"@id": "8b23c2b6-b432-45d8-a377-d003950c0fcc", "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/present-proof/1.0/request-presentation", "comment": "Person Proving", "request_presentations~attach": [{"@id": "libindy-request-presentation-0", "data": {"base64": "eyJuYW1lIjoiUGVyc29uIFByb3ZpbmciLCJub25fcmV2b2tlZCI6bnVsbCwibm9uY2UiOiI2MzQxNzYyOTk0NjI5NTQ5MzA4MjY1MzQiLCJyZXF1ZXN0ZWRfYXR0cmlidXRlcyI6eyJhdHRyaWJ1dGVfMCI6eyJuYW1lIjoibmFtZSJ9LCJhdHRyaWJ1dGVfMSI6eyJuYW1lIjoiZW1haWwifX0sInJlcXVlc3RlZF9wcmVkaWNhdGVzIjp7fSwidmVyIjpudWxsLCJ2ZXJzaW9uIjoiMS4wIn0="}, "mime-type": "application/json"}]}"
///
/// #Returns
/// Message as JSON
- (void) proofVerifierGetProofRequestAttach:(NSInteger) proofHandle
                                 completion:(void (^)(NSError *error, NSString* message))completion;

/// Get Proof Msg
///
/// #Params
/// proofHandle: Proof handle that was provided during creation. Used to identify proof object
///
/// #Returns
/// Status and Message as JSON
- (void) proofVerifierGetProofMessage:(NSInteger) proofHandle
                                 completion:(void (^)(NSError *error, NSInteger proofState, NSString* message))completion;

/// Get Problem Report message for Proof object in Failed or Rejected state.
///
/// #Params
/// proofHandle: handle pointing to Proof state object.
///
/// #Returns
/// Status and Message as JSON
- (void) proofVerifierGetProblemReportMessage:(NSInteger) proofHandle
                                 completion:(void (^)(NSError *error, NSString* message))completion;

/// Bind proof state object with connection
///
/// #Params
/// proofHandle: handle pointing to Proof state object.
///
/// #Returns
/// Null
- (void) proofVerifierSetConnection:(NSInteger) proofHandle
                            connectionHandle:(NSInteger) connectionHandle
                                 completion:(void (^)(NSError *error))completion;

@end

#endif /* init_h */
