import asyncio
import json
import random
import os
import time
from ctypes import cdll
from time import sleep

from demo_utils import file_ext

from demo.demo_utils import download_message
from demo.faber import accept_taa, connect
from vcx.api.connection import Connection
from vcx.api.credential_def import CredentialDef
from vcx.api.issuer_credential import IssuerCredential
from vcx.api.proof import Proof
from vcx.api.schema import Schema
from vcx.api.utils import vcx_agent_provision, vcx_get_ledger_author_agreement, vcx_set_active_txn_author_agreement_meta
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State, ProofState

# logging.basicConfig(level=logging.DEBUG) uncomment to get logs

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
    config['institution_name'] = 'Faber'
    config['institution_logo_url'] = 'http://robohash.org/4'
    config['genesis_path'] = 'docker.txn'
    config['payment_method'] = 'null'

    print("#2 Initialize libvcx with new configuration")
    await vcx_init_with_config(json.dumps(config))

    connection_to_alice = None

    print("Connection is established")

    while True:
        answer = input(
            "Would you like to do? \n "
            "0 - establish connection \n "
            "1 - issue credential \n "
            "2 - ask for proof request \n "
            "3 - send ping \n "
            "4 - update connection state \n "
            "5 - establish out-of-band connection \n "
            "else finish \n") \
            .lower().strip()
        if answer == '0':
            connection_to_alice = await connect()
        elif answer == '1':
            await issue_credential(connection_to_alice)
        elif answer == '2':
            await ask_for_proof(connection_to_alice, config['institution_did'])
        else:
            break

    print("Finished")


async def issue_credential(connection_to_alice):
    await accept_taa()

    print("#3 Create a new schema on the ledger")
    version = format("%d.%d.%d" % (random.randint(1, 101), random.randint(1, 101), random.randint(1, 101)))
    schema = await Schema.create('schema_uuid', 'degree schema', version, ['Email', 'First Name', 'Last Name', 'Age', 'Sex', 'DateofBirth', 'Height', 'Width'], 0)
    schema_id = await schema.get_schema_id()

    print("#4 Create a new credential definition on the ledger")
    cred_def = await CredentialDef.create('credef_uuid', 'degree', schema_id, 0, "tag")
    cred_def_handle = cred_def.handle

    schema_attrs = {
        'Email': '003',
        'First Name': 'Faber',
        'Last Name': 'Test',
        'Age': '24',
        'Sex': 'male',
        'DateofBirth':'2000-02-04',
        'Height':'50',
        'Width':'150',
    }

    print("#12 Create an IssuerCredential object using the schema and credential definition")
    credential = await IssuerCredential.create('alice_degree', schema_attrs, cred_def_handle, 'Demo Credential', '0')

    print("#13 Issue credential offer to alice")
    await credential.send_offer(connection_to_alice)
    await credential.update_state()

    print("#14 Poll agency and wait for alice to send a credential request")
    credential_state = await credential.get_state()
    while credential_state != State.RequestReceived and credential_state != State.Undefined:
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


async def ask_for_proof(connection_to_alice, institution_did):
    proof_attrs = [
        {'name': ' email', 'restrictions': {'issuer_did': institution_did}},
        {'names': ['first name', 'Last Name'], 'restrictions': {'issuer_did': institution_did}},
    ]

    proof_predicates = [
        {'name': 'Height', 'p_type': '>=', 'p_value': 30},
        {'name': 'Width', 'p_type': '<', 'p_value': 200},
    ]

    print("#19 Create a Proof object")
    proof = await Proof.create('proof_uuid', 'Person Proving', proof_attrs, {}, proof_predicates)

    print("#20 Request proof of degree from alice")
    await proof.request_proof(connection_to_alice)

    print("#21 Poll agency and wait for alice to provide proof")
    proof_state = await proof.get_state()
    while proof_state != State.Accepted and proof_state != State.Undefined:
        sleep(2)
        await proof.update_state()
        proof_state = await proof.get_state()
        print(proof_state)

    if proof_state == State.Undefined:
        print("Prof Request has been rejected")
        return

    print("#27 Process the proof provided by alice")
    proof_message = await proof.get_proof(connection_to_alice)
    print(proof_message)

    print("#28 Check if proof is valid")
    if proof.proof_state == ProofState.Verified:
        print("proof is verified!!")
    else:
        print("could not verify proof :(")

    print(await proof.serialize())


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
