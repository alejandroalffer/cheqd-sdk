#!/usr/bin/env python3

from ctypes import cdll

from vcx.api.vcx_init import vcx_init_with_config
from vcx.api.utils import vcx_agent_provision
from vcx.api.connection import Connection
from vcx.error import VcxError
from vcx.state import State

import shutil
import asyncio
import os
import json


async def main():
    # Show the public DID for the connection
    # False means use a QR code
    use_public_did = True

    # Agency and wallet info
    wallet_key = 'provableinfowalletkey'
    genesis_file_location = 'docker.txn'
    enterprise_seed = '000000000000000000000000Trustee1'
    pmt_method = 'null'
    ent_instituion_name = 'Richard'
    ent_instituion_logo = 'http://robohash.org/514'

    # TestNet agency information
    ent_wallet_name = 'ent_provable-wallet'

    # Staging
    ent_agency_url = 'https://eas01.pps.evernym.com'
    ent_agency_did = 'UNM2cmvMVoWpk6r3pG5FAq'
    ent_agency_verkey = 'FvA7e4DuD2f9kYHq6B3n7hE7NQvmpgeFRrox3ELKv9vX'

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

    # creating duplicate connection to check connection redirection functionality
    duplicate_connection = await create_connection('456', use_public_did)


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
