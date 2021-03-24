import asyncio
import json
from time import sleep

from vcx.api.connection import Connection
from vcx.api.credential import Credential
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State

# logging.basicConfig(level=logging.DEBUG) uncomment to get logs

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
    connection_to_faber = None
    credential_o = None
    while True:
        answer = input(
            "Would you like to do? \n "
            "0 - establish connection \n "
            "1 - check for credential offer \n "
            "2 - check for proof request \n "
            "3 - propose proof\n "
            "else finish \n") \
            .lower().strip()
        if answer == '0':
            connection_to_faber = await connect()
        elif answer == '1':
            print("Check agency for a credential offer")
            offers = await Credential.get_offers(connection_to_faber)
            print("Offer: " + json.dumps(offers[0]))
            credential = await Credential.create('credential', offers[0])
            accept_offer_answer = input(
                "Would you like to accept offer? \n "
                "0 - accept \n "
                "1 - reject \n "
                "else finish \n") \
                .lower().strip()
            if accept_offer_answer == '0':
                credential_o = await accept_offer(connection_to_faber, credential)
            elif accept_offer_answer == '1':
                await reject_offer(connection_to_faber, credential)
            else:
                break
        elif answer == '2':
            print("Check agency for a proof request")
            requests = await DisclosedProof.get_requests(connection_to_faber)
            print("#23 Create a Disclosed proof object from proof request")
            print("Proof Request: " + json.dumps(requests[0]))
            proof = await DisclosedProof.create('proof', requests[0])
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
            else:
                break
        elif answer == '3':
            connection_to_faber = await propose_proof(credential_o)
        else:
            break

    print("Finished")

async def init():
    print("#7 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    print("#8 Initialize libvcx with new configuration")
    await vcx_init_with_config(config)


async def connect():
    print("#9 Input faber.py invitation details")
    details = input('invite details: ')

    print("#10 Convert to valid json and string and create a connection to faber")
    jdetails = json.loads(details)
    connection_to_faber = await Connection.accept_connection_invite('faber', json.dumps(jdetails))
    connection_state = await connection_to_faber.update_state()
    while connection_state != State.Accepted:
        sleep(2)
        await connection_to_faber.update_state()
        connection_state = await connection_to_faber.get_state()

    print("Connection is established")
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


async def reject_offer(connection_to_faber, credential):
    print("#15 Reject credential offer")
    await credential.reject(connection_to_faber)

    print("#16 Check credential offer state")
    credential_state = await credential.get_state()
    assert credential_state == State.Rejected


async def propose_proof(credential):
    proposal = await credential.get_presentation_proposal()
    print("Presentation Proposal")
    print(proposal)

    new_connection = await Connection.create_outofband("Connection Proposal", "Presentation", None, True,
                                                       json.dumps(proposal))
    await new_connection.connect('{"use_public_did": true}')
    details = await new_connection.invite_details(False)
    print("**invite details**")
    print(json.dumps(details))
    print("******************")

    print("#6 Poll agency and wait for alice to accept the invitation (start alice.py now)")
    connection_state = await new_connection.get_state()
    while connection_state != State.Accepted:
        sleep(2)
        await new_connection.update_state()
        connection_state = await new_connection.get_state()

    print("Connection is established")
    return new_connection


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
