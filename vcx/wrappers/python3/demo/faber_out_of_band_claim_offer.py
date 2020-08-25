import asyncio
import json
import random
import os
from ctypes import cdll
from time import sleep

from demo_utils import file_ext

from demo.demo_utils import download_message
from vcx.api.connection import Connection
from vcx.api.credential_def import CredentialDef
from vcx.api.issuer_credential import IssuerCredential
from vcx.api.schema import Schema
from vcx.api.utils import vcx_agent_provision, vcx_messages_download
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State

TAA_ACCEPT = bool(os.getenv("TAA_ACCEPT", "0") == "1")

# logging.basicConfig(level=logging.DEBUG) # uncomment to get logs

# 'agency_url': URL of the agency
# 'agency_did':  public DID of the agency
# 'agency_verkey': public verkey of the agency
# 'wallet_name': name for newly created encrypted wallet
# 'wallet_key': encryption key for encoding wallet
# 'payment_method': method that will be used for payments
provisionConfig = {
    'agency_url': 'https://eas-team1.pdev.evernym.com',
    'agency_did': 'CV65RFpeCtPu82hNF9i61G',
    'agency_verkey': '7G3LhXFKXKTMv7XGx1Qc9wqkMbwcU2iLBHL8x1JXWWC2',
    'wallet_name': 'faber_wallet',
    'wallet_key': '123',
    'payment_method': 'null',
    'enterprise_seed': '000000000000000000000000Trustee1',
    'protocol_type': '3.0',
}

async def main():
    payment_plugin = cdll.LoadLibrary('libnullpay' + file_ext())
    payment_plugin.nullpay_init()

    print("#1 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to faber
    config['institution_name'] = 'Bob'
    config['institution_logo_url'] = 'http://robohash.org/234'
    config['genesis_path'] = 'docker.txn'
    config['payment_method'] = 'null'
    config[
        'author_agreement'] = "{\"taaDigest\":\"3ae97ea501bd26b81c8c63da2c99696608517d6df8599210c7edaa7e2c719d65\",\"acceptanceMechanismType\":\"at_submission\",\"timeOfAcceptance\":" + str(
        1594193805) + "}"
    print("#2 Initialize libvcx with new configuration")
    await vcx_init_with_config(json.dumps(config))
    print("#3 Create a new schema on the ledger")
    version = format("%d.%d.%d" % (random.randint(1, 101), random.randint(1, 101), random.randint(1, 101)))
    schema = await Schema.create('schema_uuid', 'degree schema', version, ['name', 'email'], 0)
    schema_id = await schema.get_schema_id()

    print("#4 Create a new credential definition on the ledger")
    cred_def = await CredentialDef.create('credef_uuid', 'degree', schema_id, 0, "tag")
    cred_def_handle = cred_def.handle

    schema_attrs = {
        'name': 'Artem',
        'email': 'Artem@eventym.com',
    }

    print("#12 Create an IssuerCredential object using the schema and credential definition")
    credential = await IssuerCredential.create('alice_degree', schema_attrs, cred_def_handle, 'Personal Credential', '0')

    offer = await credential.get_offer_msg()
    print("Offer: " + json.dumps(offer))

    print("#5 Create a connection to alice and print out the invite details")
    connection_to_alice = await Connection.create_outofband('alice', 'issue-credential', 'Issue Credential', True, json.dumps(offer))
    await connection_to_alice.connect('{"use_public_did": true}')
    await connection_to_alice.update_state()
    details = await connection_to_alice.invite_details(False)
    print("**invite details**")
    print(json.dumps(details))
    print("******************")

    print("#6 Poll agency and wait for alice to accept the invitation (start alice.py now)")
    connection_state = await connection_to_alice.get_state()
    while connection_state != State.Accepted:
        sleep(2)
        await connection_to_alice.update_state()
        connection_state = await connection_to_alice.get_state()

    print("Set connection info into Issuer SM")
    await credential.set_connection(connection_to_alice)

    print("#14 Poll agency and wait for alice to send a credential request")
    await credential.update_state()
    credential_state = await credential.get_state()

    pw_did = await connection_to_alice.get_my_pw_did()

    # print("Wait for handshake-reuse")
    # while True:
    #     uid, _, _ = await download_message(pw_did, 'handshake-reuse')
    #     if uid:
    #         print("handshake-reuse")
    #         await connection_to_alice.update_state()
    #         break
    # print("Wait for credential-request")

    while credential_state != State.RequestReceived and credential_state != State.Undefined:
        messages = await vcx_messages_download("MS-103", None, pw_did)
        messages = json.loads(messages.decode())[0]['msgs']
        print(messages)

        sleep(2)
        await credential.update_state()
        credential_state = await credential.get_state()

    if credential_state == State.Undefined:
        print("Credential Offer has been rejected")
        return

    print("#17 Issue credential to alice")
    await credential.send_credential(connection_to_alice)

    print("#18 Wait for alice to accept credential")
    await credential.update_state()
    credential_state = await credential.get_state()
    while credential_state != State.Accepted and credential_state != State.Undefined:
        sleep(2)
        await credential.update_state()
        credential_state = await credential.get_state()

    if credential_state == State.Accepted:
        print("Credential has been issued")
    elif credential_state == State.Undefined:
        print("Credential has been rejected")

    print("Finished")


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)