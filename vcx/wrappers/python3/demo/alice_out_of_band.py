import asyncio
import json
from time import sleep

from vcx.api.connection import Connection
from vcx.api.credential import Credential
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State
from vcx.api.disclosed_proof import DisclosedProof
import base64


# logging.basicConfig(level=logging.DEBUG) uncomment to get logs

provisionConfig = {
    'agency_url': 'https://eas-team1.pdev.evernym.com',
    'agency_did': 'CV65RFpeCtPu82hNF9i61G',
    'agency_verkey': '7G3LhXFKXKTMv7XGx1Qc9wqkMbwcU2iLBHL8x1JXWWC2',
    'wallet_name': 'alice_wallet',
    'wallet_key': '123',
    'payment_method': 'null',
    'enterprise_seed': '000000000000000000000000Trustee1',
    'protocol_type': '3.0',
}


async def main():
    await init()

    print("#9 Input faber.py invitation details")
    details = input('invite details: ')

    print("#10 Convert to valid json and string and create a connection to faber")
    jdetails = json.loads(details)

    proof_request = base64.b64decode(jdetails['request~attach'][0]['data']['base64']).decode('utf-8')
    print("Proof Request: " + proof_request)
    proof = await DisclosedProof.create('proof', json.loads(proof_request))

    connection_to_faber = await Connection.create_with_outofband_invite('faber', json.dumps(jdetails))
    await connection_to_faber.connect('{"use_public_did": true}')
    connection_state = await connection_to_faber.update_state()
    while connection_state != State.Accepted:
        sleep(2)
        await connection_to_faber.update_state()
        connection_state = await connection_to_faber.get_state()

    print("Connection is established")

    accept_proof_answer = input(
                    "Would you like to accept proof? \n "
                    "0 - accept \n "
                    "1 - reject \n "
                    "else finish \n") \
                    .lower().strip()
    if accept_proof_answer == '0':
        await create_proof(connection_to_faber, proof)
    elif accept_proof_answer == '1':
        await reject_proof(connection_to_faber, proof)
    print("Proof finished")


async def init():
    print("#7 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to alice
    config['institution_name'] = 'alice'
    config['institution_logo_url'] = 'http://robohash.org/456'
    config['genesis_path'] = 'docker.txn'
    config['payment_method'] = 'null'

    config = json.dumps(config)

    print("#8 Initialize libvcx with new configuration")
    await vcx_init_with_config(config)

async def create_proof(connection_to_faber, proof):
    print("#24 Query for credentials in the wallet that satisfy the proof request")
    credentials = await proof.get_creds()

    print(credentials)

    # Use the first available credentials to satisfy the proof request
    for attr in credentials['attrs']:
        credentials['attrs'][attr] = {
            'credential': credentials['attrs'][attr][0]
        }

    print("#25 Generate the proof")
    await proof.generate_proof(credentials, {})

    print("#26 Send the proof")
    await proof.send_proof(connection_to_faber)

    proof_state = await proof.get_state()
    while proof_state != State.Accepted:
        sleep(2)
        await proof.update_state()
        proof_state = await proof.get_state()

    print("proof is verified!!")

async def reject_proof(connection_to_faber, proof):
    print("#15 Reject proof request")
    await proof.reject_proof(connection_to_faber)

    print("#16 Check proof request state")
    proof_state = await proof.get_state()
    assert proof_state == State.Rejected


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)