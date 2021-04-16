#!/usr/bin/env python3

from time import sleep

from demo_utils import download_message

from demo.faber import connect, provisionConfig
from vcx.api.vcx_init import vcx_init_with_config
from vcx.api.utils import vcx_agent_provision
from vcx.error import VcxError

import asyncio
import json


async def main():
    print("#1 Provision an agent and wallet, get back configuration details")
    config = await vcx_agent_provision(json.dumps(provisionConfig))

    config = json.loads(config)

    # Set remaining configuration options specific to the enterprise
    config['institution_name'] = 'Carl'
    config['institution_logo_url'] = 'http://robohash.org/512'
    config['protocol_type'] = '3.0'

    await vcx_init_with_config(json.dumps(config))

    connection = await connect()

    send_question = "yes"

    while send_question != "no":
        question = {
          "@type": "did:sov:BzCbsNYhMrjHiqZDTUASHg;spec/questionanswer/1.0/question",
          "@id": "518be002-de8e-456e-b3d5-8fe472477a86",
          "question_text": "Alice, are you on the phone with Bob from Faber Bank right now?",
          "question_detail": "This is optional fine-print giving context to the question and its various answers.",
          "nonce": "<valid_nonce>",
          "signature_required": True,
          "valid_responses" : [
            {"text": "Yes, it's me"},
            {"text": "No, that's not me!"}],
          "~timing": {
            "expires_time": "2018-12-13T17:29:06+0000"
          }
        }

        msg_id = await connection.send_message(json.dumps(question), "Question", "Answer this question")
        print("\n-- Dynamic message sent")
        print("Dynamic message Id: {}".format(msg_id.decode('utf-8')))

        print("Press enter to start checking response")
        start_checking_response = input()

        try:
            pw_did = await connection.get_my_pw_did()
            uid, answer, _ = await download_message(pw_did, 'answer')

            print("-- Enterprise message downloaded")
            answer = json.loads(answer)
            print("Answer")
            print(answer)
        except VcxError as e:
            print("\n::ERROR:: Enterprise message failed to download\n{}".format(e))

        print("Finished")
        print("\n Want to send another question?(yes|no)")
        send_question = input()


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    sleep(1)
