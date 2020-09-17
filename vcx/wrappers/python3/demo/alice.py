import asyncio
import json
import logging
import os
from time import sleep

from faber import INVITATION_FILE

from demo_utils import update_message_as_read
from vcx.api.connection import Connection
from vcx.api.credential import Credential
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State
from vc_auth_oidc.alice_vc_auth import handle_challenge


# logging.basicConfig(level=logging.DEBUG) uncomment to get logs
logging.basicConfig(level=50)

log = logging.getLogger("faber")
log.setLevel(logging.INFO)
formatter = logging.Formatter("%(asctime)s|%(name)s|%(levelname)s| %(message)s")
ch = logging.StreamHandler()
ch.setLevel(logging.DEBUG)
ch.setFormatter(formatter)
log.addHandler(ch)
log.propagate = False

provisionConfig = {
    'agency_url': 'https://eas-team1.pdev.evernym.com',
    'agency_did': 'CV65RFpeCtPu82hNF9i61G',
    'agency_verkey': '7G3LhXFKXKTMv7XGx1Qc9wqkMbwcU2iLBHL8x1JXWWC2',
    'wallet_name': 'alice_wallet',
    'wallet_key': '123',
    'payment_method': 'null',
    'enterprise_seed': '000000000000000000000000Trustee1',
    'protocol_type': '1.0',
}


async def main():
    await init()
    await asyncio.wait([work(i) for i in range(10)])


async def work(idx):
    idx = str(idx)
    connection_to_faber = await connect(idx)

    log.info("Check agency for a credential offer " + str(idx))
    credential = None

    processed_offers = []

    while True:
        log.info("Check for offer "+ str(idx))
        offers = await Credential.get_offers(connection_to_faber)
        offers = [offer for offer in offers if offer[0]['msg_ref_id'] not in processed_offers]
        print('Offers ' + str(idx))
        print(offers)
        if len(offers) > 0:
            offer = offers[0]
            processed_offers.append(offer[0]['msg_ref_id'])
            credential = await Credential.create('credential', offer)
            await accept_offer(connection_to_faber, credential, idx)

    log.info("Finished")


async def init():
    log.info("#7 Provision an agent and wallet, get back configuration details ")
    config = await vcx_agent_provision(json.dumps(provisionConfig))
    config = json.loads(config)
    # Set some additional configuration options specific to alice
    config['institution_name'] = 'alice'
    config['institution_logo_url'] = 'http://robohash.org/456'
    config['genesis_path'] = 'docker.txn'
    config['payment_method'] = 'null'

    config = json.dumps(config)

    log.info("#8 Initialize libvcx with new configuration ")
    await vcx_init_with_config(config)


async def connect(idx):
    idx = str(idx)
    log.info("#9 Input faber.py invitation details "+ idx)
    details = None

    while True:
        log.info("Check for Connection Invite "+idx)
        if os.path.exists(INVITATION_FILE+idx):
            f = open(INVITATION_FILE+idx)
            details = f.read()
            break
        await asyncio.sleep(2)

    log.info("#10 Convert to valid json and string and create a connection to faber "+ idx)
    jdetails = json.loads(details)
    connection_to_faber = await Connection.create_with_details('faber', json.dumps(jdetails))
    await connection_to_faber.connect('{"use_public_did": true}')
    connection_state = await connection_to_faber.update_state()
    timer = 4
    while connection_state != State.Accepted:
        await asyncio.sleep(timer)
        await connection_to_faber.update_state()
        connection_state = await connection_to_faber.get_state()

    log.info("Connection is established "+ idx)
    return connection_to_faber


async def accept_offer(connection_to_faber, credential, idx):
    log.info("#15 After receiving credential offer, send credential request "+ idx)
    await credential.send_request(connection_to_faber, 0)

    log.info("#16 Poll agency and accept credential offer from faber "+ idx)
    credential_state = await credential.get_state()
    timer = 4
    while credential_state != State.Accepted:
        await asyncio.sleep(timer)
        await credential.update_state()
        credential_state = await credential.get_state()


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
