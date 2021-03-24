import asyncio
import json
import random
import string
from time import sleep

from demo.faber import provisionConfig, connect, issue_credential, ask_for_proof
from vcx.api.utils import vcx_agent_provision
from vcx.api.vcx_init import vcx_init_with_config


def rand_string():
    return ''.join(random.choice(string.ascii_uppercase + string.digits) for _ in range(103))


async def main():
    provisionConfig['name'] = 'Faber 100 attributes'

    print("#1 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))

    print("#2 Initialize libvcx with new configuration")
    await vcx_init_with_config(config)

    connection_to_alice = None

    while True:
        answer = input(
            "Would you like to do? \n "
            "0 - establish connection \n "
            "1 - issue credential \n "
            "2 - ask for proof request \n "
            "else finish \n") \
            .lower().strip()
        if answer == '0':
            connection_to_alice = await connect()
        elif answer == '1':
            await issue_credential(connection_to_alice, schema_attributes(), credential_values(), credential_name())
        elif answer == '2':
            institution_did = json.loads(config)['institution_did']
            await ask_for_proof(connection_to_alice, proof_attrs(institution_did), proof_predicates(institution_did))
        else:
            break

    print("Finished")


def schema_attributes():
    return ["attr" + str(i) for i in range(1, 101)]


def credential_values():
    schema_attrs = {}
    for i in range(1, 101):
        schema_attrs["attr" + str(i)] = rand_string()
    return schema_attrs


def credential_name():
    return 'Demo Credential 100 Attributes'


def proof_attrs(institution_did):
    return [{'name': "attr" + str(i)} for i in range(1, 101)]


def proof_predicates(institution_did):
    return []


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
