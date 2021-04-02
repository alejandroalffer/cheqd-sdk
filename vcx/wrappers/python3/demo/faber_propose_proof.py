import asyncio
import json
from time import sleep

from demo.demo_utils import download_message
from vcx.api.connection import Connection
from vcx.api.proof import Proof
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State, ProofState

provisionConfig = {
    'agency_url': 'https://eas01.pps.evernym.com',
    'agency_did': 'UNM2cmvMVoWpk6r3pG5FAq',
    'agency_verkey': 'FvA7e4DuD2f9kYHq6B3n7hE7NQvmpgeFRrox3ELKv9vX',
    'wallet_name': 'faber_wallet',
    'wallet_key': '123',
    'enterprise_seed': '000000000000000000000000Trustee1',
    'protocol_type': '3.0',
    'name': 'Matt',
    'logo': 'https://s3.us-east-2.amazonaws.com/public-demo-artifacts/demo-icons/cbFaber.png',
    'path': 'docker.txn',
}


async def main():
    print("#1 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))

    print("#2 Initialize libvcx with new configuration")
    await vcx_init_with_config(config)

    print("#3 Make a Connection")
    connection_to_alice = await connect()

    input('Press enter to start checking Presentation Proposal')
    print("#4 Accept Presentation Proposal and send Presentation Request")
    await accept_proposal(connection_to_alice)

    print("Finished")


async def connect():
    print("#9 Input alice invitation details")
    details = input('invite details: ')

    print("#10 Convert to valid json and string and create a connection to faber")
    jdetails = json.loads(details)

    connection_to_alice = await Connection.create_with_outofband_invite('faber', json.dumps(jdetails))
    await connection_to_alice.connect('{"use_public_did": true}')

    print("#6 Poll agency and wait for alice to accept the invitation (start alice.py now)")
    connection_state = await connection_to_alice.get_state()
    while connection_state != State.Accepted:
        sleep(2)
        connection_state = await connection_to_alice.update_state()
    print("Connection is established")
    return connection_to_alice


async def accept_proposal(connection_to_alice):

    pw_did = await connection_to_alice.get_my_pw_did()
    uid, presentation_proposal, _ = await download_message(pw_did, 'propose-presentation')

    print("#20 Request proof from alice")
    proof = await Proof.create_with_proposal('proof_uuid', presentation_proposal, 'Credential Proving')
    await proof.request_proof(connection_to_alice)

    print("#21 Poll agency and wait for alice to provide proof")
    proof_state = await proof.get_state()
    while proof_state != State.Accepted:
        sleep(2)
        await proof.update_state()
        proof_state = await proof.get_state()

    print("#27 Process the proof provided by alice")
    await proof.get_proof(connection_to_alice)

    print("#28 Check if proof is valid")
    if proof.proof_state == ProofState.Verified:
        print("proof is verified!!")
    else:
        print("could not verify proof :(")


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
