import asyncio
import json
import logging
import random
import os
import time
from ctypes import cdll
from time import sleep

from demo_utils import file_ext
from vcx.api.connection import Connection
from vcx.api.credential_def import CredentialDef
from vcx.api.issuer_credential import IssuerCredential
from vcx.api.proof import Proof
from vcx.api.schema import Schema
from vcx.api.utils import vcx_agent_provision, vcx_get_ledger_author_agreement, vcx_set_active_txn_author_agreement_meta
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State, ProofState

TAA_ACCEPT = bool(os.getenv("TAA_ACCEPT", "0") == "1")


logging.basicConfig(level=50)

log = logging.getLogger("faber")
log.setLevel(logging.INFO)
formatter = logging.Formatter("%(asctime)s|%(name)s|%(levelname)s| %(message)s")
ch = logging.StreamHandler()
ch.setLevel(logging.DEBUG)
ch.setFormatter(formatter)
log.addHandler(ch)
log.propagate = False



# logging.basicConfig(
#     format='%(asctime)s %(message)s',
#     level=logging.DEBUG,
#     datefmt='%Y-%m-%d %H:%M:%S')


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
    'protocol_type': '1.0',
}


INVITATION_FILE = "invitation.json"


async def main():
    payment_plugin = cdll.LoadLibrary('libnullpay' + file_ext())
    payment_plugin.nullpay_init()

    for i in range(20):
        try:
            os.remove(INVITATION_FILE + str(i))
        except:
            pass


    log.info("#1 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to faber
    config['institution_name'] = 'Faber'
    config['institution_logo_url'] = 'http://robohash.org/234'
    config['genesis_path'] = 'docker.txn'
    config['payment_method'] = 'null'
    config[
        'author_agreement'] = "{\"taaDigest\":\"3ae97ea501bd26b81c8c63da2c99696608517d6df8599210c7edaa7e2c719d65\",\"acceptanceMechanismType\":\"at_submission\",\"timeOfAcceptance\":" + str(
        1594193805) + "}"

    log.info("#2 Initialize libvcx with new configuration")
    await vcx_init_with_config(json.dumps(config))

    log.info("#3 Create a new schema on the ledger")
    version = format("%d.%d.%d" % (random.randint(1, 101), random.randint(1, 101), random.randint(1, 101)))
    schema = await Schema.create('schema_uuid', 'degree schema', version, ['email', 'first_name', 'last_name'], 0)
    schema_id = await schema.get_schema_id()

    log.info("#4 Create a new credential definition on the ledger")
    cred_def = await CredentialDef.create('credef_uuid', 'degree', schema_id, 0)
    cred_def_handle = cred_def.handle


    await asyncio.wait([work(cred_def_handle, i) for i in range(10)])


async def work(cred_def_handle, idx):
    idx = str(idx)
    # spawn 20 connections
    log.info("#5 Create a connection to alice and print out the invite details "+ idx)
    connection_to_alice = await Connection.create('alice')
    await connection_to_alice.connect('{"use_public_did": true}')
    await connection_to_alice.update_state()
    details = await connection_to_alice.invite_details(False)
    log.info("**invite details** " + idx)
    log.info(json.dumps(details))
    log.info("****************** " + idx)

    f = open(INVITATION_FILE+idx, "w")
    f.write(json.dumps(details))
    f.close()

    connection_state = await connection_to_alice.get_state()
    timer = 4
    while connection_state != State.Accepted:
        log.info("#6 Poll agency and wait for alice to accept the invitation (start alice.py now) " + idx)
        await asyncio.sleep(timer)
        await connection_to_alice.update_state()
        connection_state = await connection_to_alice.get_state()

    log.info("Connection is established " + idx)

    # spawn 20 credentials for each connection
    await asyncio.wait([issue_credential(connection_to_alice, cred_def_handle, idx, str(j)) for j in range(10)])

    log.info("Finished " + idx)
    os.remove(INVITATION_FILE +idx)


async def issue_credential(connection_to_alice, cred_def_handle, idx1, idx2):
    idx = idx1+' - '+idx2
    schema_attrs = {
        'email': 'test',
        'first_name': 'DemoName',
        'last_name': 'DemoLastName',
    }

    log.info("#12 Create an IssuerCredential object using the schema and credential definition " + idx)
    credential = await IssuerCredential.create('alice_degree', schema_attrs, cred_def_handle, 'cred', '0')

    log.info("#13 Issue credential offer to alice "+ idx)

    await credential.send_offer(connection_to_alice)

    credential_state = await credential.get_state()
    timer = 2
    while credential_state != State.RequestReceived:
        log.info("#14 Poll agency and wait for alice to send a credential request " + idx)
        await asyncio.sleep(timer)
        timer = timer + 2
        await credential.update_state()
        credential_state = await credential.get_state()

    log.info("#17 Issue credential to alice "+ idx)
    await credential.send_credential(connection_to_alice)
    end = time.time()

    await credential.update_state()
    credential_state = await credential.get_state()
    timer = 4
    while credential_state != State.Accepted:
        log.info("#18 Poll agency and wait for alice to accept credential " + idx)
        await asyncio.sleep(timer)
        await credential.update_state()
        credential_state = await credential.get_state()
    log.info("#19 Credential accepted "+ idx)


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
