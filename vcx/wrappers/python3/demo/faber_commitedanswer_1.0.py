#!/usr/bin/env python3

from ctypes import cdll

from demo.demo_utils import download_message
from vcx.api.vcx_init import vcx_init, vcx_init_with_config
from vcx.api.utils import vcx_agent_provision, vcx_messages_download
from vcx.api.connection import Connection
from vcx.api.proof import Proof
from vcx.error import VcxError
from vcx.state import State
from multiprocessing import Process, Queue
from time import sleep

import shutil
import logging
import asyncio
import sys
import os
import json
import base64
import datetime
import time


# import qrcode

async def main():
    # Show the public DID for the connection
    # False means use a QR code
    use_public_did = False

    # Message expiration - set to 2 days in the future...
    now = datetime.datetime.today().strftime("%Y-%m-%dT%H:%M:%S+0000")
    future = (datetime.datetime.now() + datetime.timedelta(days=2)).strftime("%Y-%m-%dT%H:%M:%S+0000")

    # Agency and wallet info
    wallet_key = 'provableinfowalletkey'
    genesis_file_location = 'docker.txn'
    enterprise_seed = '000000000000000000000000Trustee1'
    pmt_method = 'null'
    ent_instituion_name = 'Carla'
    ent_instituion_logo = 'http://robohash.org/509'

    # TestNet agency information
    print("\nUse TestNet settings")
    ent_wallet_name = 'ent_provable-wallet'

    # QA-RC environment
    # ent_agency_url = 'https://eas.pqa.evernym.com'
    # ent_agency_did = 'QreyffsPPLCUqetQbahYNu'
    # ent_agency_verkey = 'E194CfHi5GGRiy1nThMorPf3jBEm4tvcAgcb65JFfxc7'

    # Dev-Team1
    ent_agency_url = 'https://eas-team1.pdev.evernym.com'
    ent_agency_did = 'CV65RFpeCtPu82hNF9i61G'
    ent_agency_verkey = '7G3LhXFKXKTMv7XGx1Qc9wqkMbwcU2iLBHL8x1JXWWC2'

    # Prod
    # ent_agency_url = 'https://eas.evernym.com'
    # ent_agency_did = '5YKgVzinHVv5XfudLv5F4k'
    # ent_agency_verkey = '3UX8ZEkpg6ZGPiqdTWdPm5c63z5XotrD7vSKp8DLE9iu'

    # Remove wallet if it exists
    clean_start([ent_wallet_name])

    # Provision first then run the test
    print("\n-- Provision enterprise")

    enterprise_config = {
        'agency_url': ent_agency_url,
        'agency_did': ent_agency_did,
        'agency_verkey': ent_agency_verkey,
        'wallet_name': ent_wallet_name,
        'wallet_key': wallet_key,
        'enterprise_seed': enterprise_seed,
    }

    config = await vcx_agent_provision(json.dumps(enterprise_config))
    config = json.loads(config)

    # Set remaining configuration options specific to the enterprise
    config['payment_method'] = pmt_method
    config['institution_name'] = ent_instituion_name
    config['institution_logo_url'] = ent_instituion_logo
    config['genesis_path'] = genesis_file_location
    config['protocol_type'] = '1.0'
    # config['use_latest_protocol'] = 'true'

    # Init the payment plug-in
    if pmt_method == 'null':
        lib = cdll.LoadLibrary("libnullpay.so")
        lib.nullpay_init()
    else:
        lib = cdll.LoadLibrary("libsovtoken.so")
        lib.sovtoken_init()

    # Init using the config
    try:
        await vcx_init_with_config(json.dumps(config))
        print('\nVcx init complete (enterprise)')
    except VcxError as e:
        print("\nCould not initialize VCX: {0}".format(e))
        print("\nCould not initialize VCX (enterprise): {0}".format(e))

    connection = await create_connection('123', use_public_did)

    send_question = "yes"

    while send_question != "no":
        question = {
            '@type': 'did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/committedanswer/1.0/question',
            '@id': '518be002-de8e-456e-b3d5-8fe472477a86',
            'question_text': 'Alice, are you on the phone with Bob from Faber Bank right now?',
            'question_detail': 'This is optional fine-print giving context to the question and its various answers.',
            'valid_responses': [
                {'text': 'Yes, it is me', 'nonce': '<unique_identifier_a+2018-12-13T17:00:00+0000>'},
                {'text': 'No, that is not me!', 'nonce': '<unique_identifier_b+2018-12-13T17:00:00+0000'},
                {'text': 'Hi!', 'nonce': '<unique_identifier_c+2018-12-13T17:00:00+0000'}],
            '@timing': {
                'expires_time': future
            },
            'external_links': [
                {
                    'text': 'Some external link with so many characters that it can go outside of two lines range from here onwards',
                    'src': '1'},
                {
                    'src': 'Some external link with so many characters that it can go outside of two lines range from here onwards'},
            ]
        }

        msg_id = await connection.send_message(json.dumps(question), "Question", "Answer this question")
        print("\n-- Dynamic message sent")
        print("Dynamic message Id: {}".format(msg_id.decode('utf-8')))

        print("Press enter to start checking response")
        start_checking_response = input()

        try:
            originalMessage = await vcx_messages_download('', "{}".format(msg_id.decode('utf-8')), None)
            originalMessage = json.loads(originalMessage.decode('utf-8'))
            responseMessageId = originalMessage[0]['msgs'][0]['refMsgId']
            messages = await vcx_messages_download('', "{}".format(responseMessageId), None)
            print("-- Enterprise message downloaded")
            messages = json.loads(messages.decode('utf-8'))
            print(messages)
            answer = json.loads(json.loads(messages[0]['msgs'][0]['decryptedPayload'])['@msg'])

            #   {'@type': 'did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/committedanswer/1.0/answer',
            #    'response.@sig': {
            #       'signature': 'wK0/2hGn7Auf831PESB9uOD1YgruPIRjhqfdPH8i2cUcN/YAhaYxN8fAWSLo9bmjILd+1sJCn6FvghmY5+H8CA==',
            #       'sig_data': 'PHVuaXF1ZV9pZGVudGlmaWVyX2ErMjAxOC0xMi0xM1QxNzowMDowMCswMDAwPg==',
            #       'timestamp': '2018-12-13T17:29:34+0000'}
            #   }

            signature = base64.b64decode(answer['response.@sig']['signature'])
            data = answer['response.@sig']['sig_data']
            valid = await connection.verify_signature(data.encode(), signature)
            print("\n-- Signature verified for message...")

            if valid:
                print("-- Answer digitally signed: ", base64.b64decode(data))
            else:
                print("-- Signature was not valid")
        except VcxError as e:
            print("\n::ERROR:: Enterprise message failed to download\n{}".format(e))

        print("Finished")
        print("\n Want to send another question?(yes|no)")
        send_question = input()


async def create_connection(connection_name, use_public_did):
    connection = await Connection.create(connection_name)

    print("\n--  use public did:{}".format(use_public_did))
    if use_public_did:
        await connection.connect('{"use_public_did":true,"connection_type":"QR"}')
        invite_details = await connection.invite_details(False)
        print("\t-- Send_offer: invite_details:", json.dumps(invite_details))
    else:
        await connection.connect('{"connection_type":"SMS","phone":"19072313240"}')
        invite_details = await connection.invite_details(True)
        print('\n %s \n' % str(json.dumps(invite_details)))
        # img = qrcode.make(str(json.dumps(invite_details)))
        # img.save("qr.png")

    connection_state = await connection.get_state()
    while connection_state != State.Accepted and connection_state != State.Redirected:
        await asyncio.sleep(15)
        print("calling update_state")
        await connection.update_state()
        connection_state = await connection.get_state()
        print(connection_state)

    print("DONE calling update_state" + str(connection_state))

    return connection


def clean_start(wallets_to_remove):
    """
    Erase existing wallets if they exist
    :return:
    """

    print("Remove test wallets...")
    wallet_path = '/Users/evernym/.indy_client/wallet'

    for _ in wallets_to_remove:
        check = wallet_path + os.sep + _
        if os.path.exists(check):
            print("\nRemoving {0}".format(check))
            shutil.rmtree(check, ignore_errors=True)
        else:
            print("Could not find {} or the wallet does not exist".format(check))


if __name__ == '__main__':
    print("If you are on a mac do...")
    print(
        "You MUST copy this script to the /Users/norm/forge/work/code/evernym/indy-sdk.evernym/vcx/wrappers/python3 folder and run it from there or else it will not work")
    print("export DYLD_LIBRARY_PATH=[path_to_folder_containing_libindy.so]:${DYLD_LIBRARY_PATH}")
    print(
        "ENV: export DYLD_LIBRARY_PATH=/Users/norm/forge/tools/evernym/lib/mac/x86_64-apple-darwin:${DYLD_LIBRARY_PATH}")
    print("Usage: python3 ./test_QA_provable_question_answer.py")

    asyncio.get_event_loop().run_until_complete(main())
