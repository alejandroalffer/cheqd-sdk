# Introduction

This document explains how to use API provided by VCX library to handle [Out-of-Band](https://github.com/hyperledger/aries-rfcs/tree/master/features/0434-outofband) Aries protocol.

## Limitations

- In the current state library is not support cases when Invitation contains `request~attach` field.
    - Sender: You can use simple messages like Question to insert into Out-of-Band Invitation but you can not use Credential Offer to 
    issue a credential using `vcx_issuer_credential` API or Presentation Request to request presentation using `vcx_proof` APIs.
    Their state objects are not compatible with out-of-band flow in the current state. The support for this cases will come in next releases.
    - Receiver: The user has to analyze the value of "request~attach" field and make a decision regarding its follow processing.
      For example: If out-of-band invitation contains Credential Offer message the used has to create Credential state object once connection is established.

## Tips on Out-of-Band Invitation processing

## Receiver

* On received Out-of-Band Invitation user must take the first recipientKey and check whether Connection already exists. 
The next steps depend on three factors:
    * existence of Connection
    * `handshake_protocols` field 
    * `goal_code` or `request~attach` fields - Note, that `goal_code` marked as an optional field.
    In the table below we will use `request~attach` field but we can use `goal_code` as well (if it present).

<table>
    <tr>  
      <th>`handshake_protocols` Present?</th>
      <th>`request~attach` Present?</th>
      <th>Connection exists?</th>
      <th>action</th>
    </tr>
    <tr>
      <td><b>No</b></td>
      <td><b>No</b></td>
      <td><b>No</b></td>
      <td>
        <pre>
throw error
        </pre>
    </tr>
    </tr>
    <tr>
      <td><b>Yes</b></td>
      <td><b>No</b></td>
      <td><b>No</b></td>
      <td>
        <pre>
1. Call `create_connection_with_outofband_invitation` function to process invite.
2. Complete connection with regular flow.
        </pre>
      </td>
    </tr>
    <tr>
      <td><b>No</b></td>
      <td><b>Yes</b></td>
      <td><b>No</b></td>
      <td>
        <pre>
1. Store `request~attach`.
2. Call `create_connection_with_outofband_invitation` function to process invite.
3. Process message from the `request~attach` using created connection object.
        </pre>
      </td>
    </tr>
    <tr>
      <td><b>No</b></td>
      <td><b>No</b></td>
      <td><b>No</b></td>
      <td>
        <pre>
throw error
        </pre>
    </tr>
    <tr>
      <td><b>Yes</b></td>
      <td><b>Yes</b></td>
      <td><b>No</b></td>
      <td>
        <pre>
1. Store `request~attach`.
2. Call `create_connection_with_outofband_invitation` function to process invite.
3. Complete connection with regular flow.
4. Process message from the `request~attach` using created connection object.
        </pre>
    </tr>
    <tr>
      <td><b>Yes</b></td>
      <td><b>No</b></td>
      <td><b>Yes</b></td>
      <td>
        <pre>
1. Call `send_reuse_message` using exisiting connection.
2. Wait until `handshake-reuse-accepted` message is received
        </pre>
    </tr>
    <tr>
      <td><b>No</b></td>
      <td><b>Yes</b></td>
      <td><b>Yes</b></td>
      <td>
        <pre>
1. Process message from the `request~attach` using existing connection.
        </pre>
    </tr>
    <tr>
      <td><b>Yes</b></td>
      <td><b>Yes</b></td>
      <td><b>Yes</b></td>
      <td>
        <pre>
1. Process message from the `request~attach` using existing connection.
        </pre>
    </tr>
  </table>
 
 
## Sender

In order to create Out-of-Band invitation you need to use `vcx_connection_create_outofband` function.

This function accepts following parameters:
* source_id: institution's personal identification for the Connection. It'll be used as a label in Invitation.

* goal_code: Optional<string> - a self-attested code the receiver may want to display to
                               the user or use in automatically deciding what to do with the out-of-band message.

* goal:  Optional<string> - a self-attested string that the receiver may want to display to the user about
                           the context-specific goal of the out-of-band message.

* handshake: whether Inviter wants to establish regular connection using `connections` handshake protocol.
            if false, one-time connection channel will be created.

* request_attach: Optional<string> - An additional message as JSON that will be put into attachment decorator
                                    that the receiver can using in responding to the message (for example Question message).

Please note that you can use simple messages like Question to insert into Out-of-Band Invitation.
But you can not use Credential Offer to issue a credential using `vcx_issuer_credential` API or Presentation Request to request presentation using `vcx_proof` APIs.
Their state objects are not compatible with out-of-band flow in the current state. The support for this cases will come in next releases.

<table>
    <tr>  
      <th>`handshake` requested</th>
      <th>`request~attach` passed?</th>
      <th>action</th>
    </tr>
    <tr>
      <td><b>No</b></td>
      <td><b>No</b></td>
      <td>
        <pre>
throw error
        </pre>
    </tr>
    </tr>
    <tr>
      <td><b>Yes</b></td>
      <td><b>No</b></td>
      <td>
        <pre>
1. Call `vcx_connection_create_outofband` function to create invite.
2. Complete connection with regular flow.
        </pre>
      </td>
    </tr>
    <tr>
      <td><b>No</b></td>
      <td><b>Yes</b></td>
      <td>
        <pre>
1. Call `vcx_connection_create_outofband` function to create invite.
2. Wait until response on `request~attach` is received.
3. Precess response.
        </pre>
      </td>
    </tr>
    <tr>
      <td><b>Yes</b></td>
      <td><b>Yes</b></td>
      <td>
        <pre>
1. Call `vcx_connection_create_outofband` function to create invite.
2. Complete connection with regular flow.
3. Wait until response on `request~attach` is received.
4. Precess response.
        </pre>
    </tr>
  </table>