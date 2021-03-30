import sys
import json
import random
from ctypes import cdll
from time import sleep
import platform

from vcx.api.credential_def import CredentialDef
from vcx.api.issuer_credential import IssuerCredential
from vcx.api.credential import Credential
from vcx.api.proof import Proof
from vcx.api.disclosed_proof import DisclosedProof
from vcx.api.schema import Schema
from vcx.api.utils import vcx_messages_download, vcx_messages_update_status
from vcx.state import State, ProofState


EXTENSION = {"darwin": ".dylib", "linux": ".so", "win32": ".dll", 'windows': '.dll'}


def file_ext():
    your_platform = platform.system().lower()
    return EXTENSION[your_platform] if (your_platform in EXTENSION) else '.so'


async def download_message(pw_did: str, msg_type: str):
    messages = await vcx_messages_download("MS-103", None, pw_did)
    messages = json.loads(messages.decode())[0]['msgs']
    print(messages)
    if msg_type:
        message = [message for message in messages if json.loads(message["decryptedPayload"])["@type"]["name"] == msg_type]
        if len(message):
            decryptedPayload = message[0]["decryptedPayload"]
            return message[0]["uid"], json.loads(decryptedPayload)["@msg"], json.dumps(message[0])
        else:
            return None, None, None
    else:
        return None, None, None


async def update_message_as_read(pw_did: str, uid: str):
    messages_to_update = [{
        "pairwiseDID": pw_did,
        "uids": [uid]
    }]
    await vcx_messages_update_status(json.dumps(messages_to_update))


def load_payment_library():
    payment_plugin = cdll.LoadLibrary('libnullpay' + file_ext())
    payment_plugin.nullpay_init()
