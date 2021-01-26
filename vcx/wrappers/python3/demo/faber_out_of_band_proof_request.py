import asyncio
import json
import os
from time import sleep

from vcx.api.connection import Connection
from vcx.api.proof import Proof
from vcx.api.utils import vcx_agent_provision
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
    'enterprise_seed': '000000000000000000000000Trustee1',
    'protocol_type': '3.0',
}

async def main():
    print("#1 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to faber
    config['institution_name'] = 'Frank'
    config['institution_logo_url'] = 'http://robohash.org/234'
    config['genesis_path'] = 'docker.txn'

    print("#2 Initialize libvcx with new configuration")
    await vcx_init_with_config(json.dumps(config))

    proof_attrs = [
        {'name': 'email'},
    ]

    print("#19 Create a Proof object")
    proof = await Proof.create('proof_uuid', 'Person Proving', proof_attrs, {})

    proof_request = await proof.get_proof_request_attach()
    print("Proof Reuqest: " + json.dumps(proof_request))

    print("#5 Create a connection to alice and print out the invite details")
    connection_to_alice = await Connection.create_outofband('alice', 'request-proof', 'Person Proving', True, json.dumps(proof_request))
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

    print("Set connection info into Verifier SM")
    await proof.set_connection(connection_to_alice)

    pw_did = await connection_to_alice.get_my_pw_did()

    # print("Wait for handshake-reuse")
    # while True:
    #     uid, _, _ = await download_message(pw_did, 'handshake-reuse')
    #     if uid:
    #         print("handshake-reuse")
    #         await connection_to_alice.update_state()
    #         break
    # print("Wait for credential-request")

    print("#21 Poll agency and wait for alice to provide proof")
    proof_state = await proof.get_state()
    while proof_state != State.Accepted and proof_state != State.Rejected:
        sleep(2)
        await proof.update_state()
        proof_state = await proof.get_state()
        print(proof_state)

    if proof_state == State.Rejected:
        print("Prof Request has been rejected")
        return

    print("Finished")


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
