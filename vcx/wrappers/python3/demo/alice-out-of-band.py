import asyncio
import json
from time import sleep

from vcx.api.connection import Connection
from vcx.api.credential import Credential
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config
from vcx.state import State
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

    offer = base64.b64decode(jdetails['request~attach'][0]['data']['base64']).decode('utf-8')
    print("Offer: " + offer)
    credential = await Credential.create('credential', json.loads(offer))

    connection_to_faber = await Connection.create_with_outofband_invite('faber', json.dumps(jdetails))
    await connection_to_faber.connect('{"use_public_did": true}')
    connection_state = await connection_to_faber.update_state()
    while connection_state != State.Accepted:
        sleep(2)
        await connection_to_faber.update_state()
        connection_state = await connection_to_faber.get_state()

    print("Connection is established")

    print("#15 After receiving credential offer, send credential request")
    await credential.send_request(connection_to_faber, 0)

    print("#16 Poll agency and accept credential offer from faber")
    credential_state = await credential.get_state()
    while credential_state != State.Accepted:
        sleep(2)
        await credential.update_state()
        credential_state = await credential.get_state()

    print("Credential saved")

    print("Finished")


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


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
