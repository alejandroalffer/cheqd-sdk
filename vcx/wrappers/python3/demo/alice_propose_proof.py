import asyncio
import json
from time import sleep

from vcx.api.connection import Connection
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State

provisionConfig = {
    'agency_url': 'https://agency.pps.evernym.com',
    'agency_did': '3mbwr7i85JNSL3LoNQecaW',
    'agency_verkey': '2WXxo6y1FJvXWgZnoYUP5BJej2mceFrqBDNPE3p6HDPf',
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

    print("#2 Propose Proof for the Credential")
    await propose_proof(connection_to_faber)

    input('Press enter to start checking Presentation Request')
    print("3. Create Proof for the Credential")
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


async def connect():
    connection_to_faber = await Connection.create_outofband("Connection Proposal",
                                                            "Presentation Proposal",
                                                            None,
                                                            True,
                                                            None)
    await connection_to_faber.connect('{"use_public_did": true}')
    details = await connection_to_faber.invite_details(False)
    print("**invite details**")
    print(json.dumps(details))
    print("******************")

    print("#6 Poll agency and wait for faber to accept the invitation")
    connection_state = await connection_to_faber.get_state()
    while connection_state != State.Accepted:
        sleep(2)
        connection_state = await connection_to_faber.update_state()

    print("Connection is established")
    return connection_to_faber


async def propose_proof(connection_to_faber):
    presentation_proposal = {
        "attributes": [
            {
                "name": "FirstName",
                "cred_def_id": "V4SGRU86Z58d6TV7PBUe6f:3:CL:194217:tag",
                "value": "Rebecca"
            },
            {
                "name": "MemberID",
                "cred_def_id": "V4SGRU86Z58d6TV7PBUe6f:3:CL:194217:tag",
                "value": "435345"
            },
        ],
        "predicates": []
    }
    print("Presentation Proposal: " + json.dumps(presentation_proposal))
    proof = await DisclosedProof.create_proposal("Presentation Proposal", presentation_proposal, "Share Credentials")
    await proof.send_proposal(connection_to_faber)


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
