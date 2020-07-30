import { Callback } from 'ffi'

import { VCXInternalError } from '../errors'
import { initRustAPI, rustAPI } from '../rustlib'
import { createFFICallbackPromise } from '../utils/ffi-helpers'
import { IInitVCXOptions } from './common'

/**
 * Initializes VCX with config file.
 * An example config file is at libvcx/sample_config/config.json
 * The list of available options see here: https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md
 *
 * Example:
 * ```
 * await initVcx('/home/username/vcxconfig.json')
 * ```
 */
export async function initVcx (configPath: string, options: IInitVCXOptions = {}): Promise<void> {
  initRustAPI(options.libVCXPath)
  let rc = null
  try {
    return await createFFICallbackPromise<void>(
      (resolve, reject, cb) => {
        rc = rustAPI().vcx_init(0, configPath, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback(
        'void',
        ['uint32', 'uint32'],
        (xhandle: number, err: number) => {
          if (err) {
            reject(err)
            return
          }
          resolve()
        })
    )
  } catch (err) {
    throw new VCXInternalError(err)
  }
}

/**
 * Initializes VCX with config file.
 * The list of available options see here: https://github.com/hyperledger/indy-sdk/blob/master/docs/configuration.md
 *
 * Example:
 * ```
 * config = {
 *   "agency_did": "VsKV7grR1BUE29mG2Fm2kX",
 *   "agency_verkey": "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR",
 *   "agency_endpoint": "http://localhost:8080",
 *   "genesis_path":"/var/lib/indy/verity-staging/pool_transactions_genesis",
 *   "institution_name": "institution",
 *   "institution_logo_url": "http://robohash.org/234",
 *   "institution_did": "EwsFhWVoc3Fwqzrwe998aQ",
 *   "institution_verkey": "8brs38hPDkw5yhtzyk2tz7zkp8ijTyWnER165zDQbpK6",
 *   "remote_to_sdk_did": "EtfeMFytvYTKnWwqTScp9D",
 *   "remote_to_sdk_verkey": "8a7hZDyJK1nNCizRCKMr4H4QbDm8Gg2vcbDRab8SVfsi",
 *   "sdk_to_remote_did": "KacwZ2ndG6396KXJ9NDDw6",
 *   "sdk_to_remote_verkey": "B8LgZGxEPcpTJfZkeqXuKNLihM1Awm8yidqsNwYi5QGc"
 *  }
 * await initVcxWithConfig(JSON.stringify(config))
 * ```
 */
export async function initVcxWithConfig (config: string, options: IInitVCXOptions = {}): Promise<void> {
  initRustAPI(options.libVCXPath)
  let rc = null
  try {
    return await createFFICallbackPromise<void>(
      (resolve, reject, cb) => {
        rc = rustAPI().vcx_init_with_config(0, config, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback(
        'void',
        ['uint32', 'uint32'],
        (xhandle: number, err: number) => {
          if (err) {
            reject(err)
            return
          }
          resolve()
        })
    )
  } catch (err) {
    throw new VCXInternalError(err)
  }
}

export function initMinimal (config: string): number {
  return rustAPI().vcx_init_minimal(config)
}

/**
 * Connect to a Pool Ledger
 *
 * You can deffer connecting to the Pool Ledger during library initialization (vcx_init or vcx_init_with_config)
 * to decrease the taken time by omitting `genesis_path` field in config JSON.
 * Next, you can use this function (for instance as a background task) to perform a connection to the Pool Ledger.
 *
 * Note: Pool must be already initialized before sending any request to the Ledger.
 *
 * EXPERIMENTAL
 *
 * config: string - the configuration JSON containing pool related settings:
 *                  {
 *                     genesis_path: string - path to pool ledger genesis transactions,
 *                     pool_name: Optional[string] - name of the pool ledger configuration will be created.
 *                                                   If no value specified, the default pool name pool_name will be used.
 *                     pool_config: Optional[string] - runtime pool configuration json:
 *                             {
 *                                 "timeout": int (optional), timeout for network request (in sec).
 *                                 "extended_timeout": int (optional), extended timeout for network request (in sec).
 *                                 "preordered_nodes": array<string> -  (optional), names of nodes which will have a priority during request sending:
 *                                         ["name_of_1st_prior_node",  "name_of_2nd_prior_node", .... ]
 *                                         This can be useful if a user prefers querying specific nodes.
 *                                         Assume that `Node1` and `Node2` nodes reply faster.
 *                                         If you pass them Libindy always sends a read request to these nodes first and only then (if not enough) to others.
 *                                         Note: Nodes not specified will be placed randomly.
 *                                 "number_read_nodes": int (optional) - the number of nodes to send read requests (2 by default)
 *                                         By default Libindy sends a read requests to 2 nodes in the pool.
 *                             }
 *                  }
 *
 * Example:
 * ```
 * config = {
 *   "genesis_path":"/var/lib/indy/verity-staging/pool_transactions_genesis",
 * }
 * await initPool(JSON.stringify(config))
 * ```
 */
export async function initPool (poolConfig: string): Promise<void> {
  try {
    return await createFFICallbackPromise<void>(
      (resolve, reject, cb) => {
        const rc = rustAPI().vcx_init_pool(0, poolConfig, cb)
        if (rc) {
          reject(rc)
        }
      },
      (resolve, reject) => Callback(
        'void',
        ['uint32', 'uint32'],
        (xhandle: number, err: number) => {
          if (err) {
            reject(err)
            return
          }
          resolve()
        })
    )
  } catch (err) {
    throw new VCXInternalError(err)
  }
}
