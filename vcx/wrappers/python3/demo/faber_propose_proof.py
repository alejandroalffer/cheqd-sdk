import asyncio
import json
import random
import time
from time import sleep

from vcx.api.connection import Connection
from vcx.api.credential_def import CredentialDef
from vcx.api.issuer_credential import IssuerCredential
from vcx.api.proof import Proof
from vcx.api.schema import Schema
from vcx.api.utils import vcx_agent_provision, vcx_get_ledger_author_agreement, \
    vcx_set_active_txn_author_agreement_meta
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State, ProofState
import base64

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
    print("#4 Issue Credential")
    await issue_credential(connection_to_alice)
    print("#5 Accept Presentation Proposal and send Presentation Request")
    await accept_proposal()

    print("Finished")


async def complete_connection(connection):
    print("#6 Poll agency and wait for alice to accept the invitation (start alice.py now)")
    connection_state = await connection.get_state()
    while connection_state != State.Accepted:
        sleep(2)
        await connection.update_state()
        connection_state = await connection.get_state()
    print("Connection is established")


async def connect():
    print("#5 Create a connection to alice and print out the invite details")
    connection_to_alice = await Connection.create('alice')
    await connection_to_alice.connect('{"use_public_did": false}')
    details = await connection_to_alice.invite_details(False)
    print("**invite details**")
    print(json.dumps(details))
    print("******************")

    await complete_connection(connection_to_alice)
    return connection_to_alice


async def accept_proposal():
    print("#9 Input alice invitation details")
    details = input('invite details: ')

    print("#10 Convert to valid json and string and create a connection to faber")
    jdetails = json.loads(details)
    presentation_proposal = base64.b64decode(jdetails['request~attach'][0]['data']['base64']).decode()

    connection_to_alice = await Connection.create_with_outofband_invite('faber', json.dumps(jdetails))
    await connection_to_alice.connect('{"use_public_did": true}')
    await complete_connection(connection_to_alice)

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


async def accept_taa():
    # To support ledger which transaction author agreement accept needed
    print("#2.1 Accept transaction author agreement")
    txn_author_agreement = await vcx_get_ledger_author_agreement()
    txn_author_agreement_json = json.loads(txn_author_agreement)
    first_acc_mech_type = list(txn_author_agreement_json['aml'].keys())[0]
    vcx_set_active_txn_author_agreement_meta(text=txn_author_agreement_json['text'],
                                             version=txn_author_agreement_json['version'],
                                             hash=None,
                                             acc_mech_type=first_acc_mech_type, time_of_acceptance=int(time.time()))


async def issue_credential(connection_to_alice):
    await accept_taa()

    print("#3 Create a new schema on the ledger")
    version = format("%d.%d.%d" % (random.randint(1, 101), random.randint(1, 101), random.randint(1, 101)))
    schema = await Schema.create('schema_uuid', 'degree schema', version, [
        'FirstName',
        'MemberID',
        'Lastname',
        'Age',
        'Sex',
        'Male',
        'Hobby',
    ], 0)
    schema_id = await schema.get_schema_id()
    print('schema_id')
    print(schema_id)

    print("#4 Create a new credential definition on the ledger")
    cred_def = await CredentialDef.create('credef_uuid', 'degree', schema_id, 0, "tag")
    cred_def_handle = cred_def.handle
    cred_def_id = await cred_def.get_cred_def_id()
    print('cred_def_id')
    print(cred_def_id)

    schema_attrs = {
        'FirstName': 'Rebecca',
        'MemberID': '435345',
        'Lastname': 'Greaves',
        'Age': '27',
        'Sex': 'Male',
        'Male': 'F',
        'Hobby': 'Dance',
    }

    print("#12 Create an IssuerCredential object using the schema and credential definition")
    credential = await IssuerCredential.create('alice_degree', schema_attrs, cred_def_handle, 'Demo Credential 4', '0')

    print("#13 Issue credential offer to alice")
    await credential.send_offer(connection_to_alice)
    await credential.update_state()

    print("#14 Poll agency and wait for alice to send a credential request")
    credential_state = await credential.get_state()
    while credential_state != State.RequestReceived and credential_state != State.Rejected:
        sleep(2)
        await credential.update_state()
        credential_state = await credential.get_state()

    if credential_state == State.Rejected:
        problem_report = await credential.get_problem_report()
        print("Credential Offer has been rejected")
        print(problem_report)
        return

    print("#17 Issue credential to alice")
    await credential.send_credential(connection_to_alice)

    print("#18 Wait for alice to accept credential")
    await credential.update_state()
    credential_state = await credential.get_state()
    while credential_state != State.Accepted and credential_state != State.Rejected:
        sleep(2)
        await credential.update_state()
        credential_state = await credential.get_state()
        print(credential_state)

    if credential_state == State.Accepted:
        print("Credential has been issued")
    elif credential_state == State.Rejected:
        problem_report = await credential.get_problem_report()
        print("Credential has been rejected")
        print(problem_report)


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
