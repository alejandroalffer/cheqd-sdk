import asyncio
import json
from time import sleep

from vcx.api.connection import Connection
from vcx.api.credential import Credential
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State

provisionConfig = {
    'agency_url': 'https://agency.pstg.evernym.com',
    'agency_did': 'LqnB96M6wBALqRZsrTTwda',
    'agency_verkey': 'BpDPZHLbJFu67sWujecoreojiWZbi2dgf4xnYemUzFvB',
    'wallet_name': 'alice_wallet',
    'wallet_key': '123',
    'payment_method': 'null',
    'enterprise_seed': '000000000000000000000000Trustee1',
    'protocol_type': '3.0',
    'name': 'alice',
    'logo': 'http://robohash.org/456',
    'path': 'docker.txn',
}


async def main():
    await init()
    print("#1 Make a Connection")
    connection_to_faber = await connect()

    input('Press enter to start checking Credential Offers ')
    print("#2 Accept Credential")
    offers = await Credential.get_offers(connection_to_faber)
    print("Offer: " + json.dumps(offers[0]))
    credential = await Credential.create('credential', offers[0])
    await accept_offer(connection_to_faber, credential)

    print("#3 Propose Proof for the Credential")
    connection_to_faber = await propose_proof(credential)

    input('Press enter to start checking Presentation Request')
    print("4. Create Proof for the Credential")
    requests = await DisclosedProof.get_requests(connection_to_faber)
    print("Proof Request: " + json.dumps(requests[0]))
    proof = await DisclosedProof.create('proof', requests[0])
    await create_proof(connection_to_faber, proof)

    print("Finished")

async def init():
    print("#7 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    print("#8 Initialize libvcx with new configuration")
    await vcx_init_with_config(config)


async def complete_connection(connection):
    print("#6 Poll agency and wait for faber to accept the invitation")
    connection_state = await connection.get_state()
    while connection_state != State.Accepted:
        sleep(2)
        await connection.update_state()
        connection_state = await connection.get_state()

    print("Connection is established")


async def connect():
    print("#9 Input faber.py invitation details")
    details = input('invite details: ')

    print("#10 Convert to valid json and string and create a connection to faber")
    jdetails = json.loads(details)
    connection_to_faber = await Connection.accept_connection_invite('faber', json.dumps(jdetails))
    await complete_connection(connection_to_faber)
    return connection_to_faber


async def accept_offer(connection_to_faber, credential):
    print("#15 After receiving credential offer, send credential request")
    await credential.send_request(connection_to_faber, 0)

    print("#16 Poll agency and accept credential offer from faber")
    credential_state = await credential.get_state()
    while credential_state != State.Accepted:
        sleep(2)
        await credential.update_state()
        credential_state = await credential.get_state()

    return credential


async def propose_proof(credential):
    presentation_proposal = await credential.get_presentation_proposal()
    print("Presentation Proposal: " + json.dumps(presentation_proposal))

    connection_to_faber = await Connection.create_outofband("Connection Proposal",
                                                            "Presentation Proposal",
                                                            None,
                                                            True,
                                                            json.dumps(presentation_proposal))
    await connection_to_faber.connect('{"use_public_did": true}')
    details = await connection_to_faber.invite_details(False)
    print("**invite details**")
    print(json.dumps(details))
    print("******************")

    await complete_connection(connection_to_faber)
    return connection_to_faber


async def create_proof(connection_to_faber, proof):
    print("#24 Query for credentials in the wallet that satisfy the proof request")
    credentials = await proof.get_creds()

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


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
