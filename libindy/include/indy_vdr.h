#ifndef __indy__vdr__included__
#define __indy__vdr__included__

#include "indy_mod.h"
#include "indy_types.h"

#ifdef __cplusplus
extern "C" {
#endif


extern indy_error_t indy_vdr_register_indy_ledger(indy_handle_t command_handle,
                                                  const char *  namespace_list,
                                                  const char *  genesis_txn_data,
                                                  const char *  taa_config,
                                                  void          (*cb)(indy_handle_t command_handle_, indy_error_t err)
                                                 );
                                                    
extern indy_error_t indy_vdr_register_cheqd_ledger(indy_handle_t command_handle,
                                                   const char *  namespace_list,
                                                   const char *  chain_id,
                                                   const char *  node_addrs_list,
                                                   void          (*cb)(indy_handle_t command_handle_, indy_error_t err)
                                                  );

extern indy_error_t indy_vdr_ping(indy_handle_t command_handle,
                                  const char *  namespace_list,                                              
                                  void          (*cb)(indy_handle_t command_handle_, indy_error_t err, const char *const status_list)
                                 );

extern indy_error_t indy_vdr_cleanup(indy_handle_t command_handle,
                                     void          (*cb)(indy_handle_t command_handle_, indy_error_t err)
                                    );

extern indy_error_t indy_vdr_resolve_did(indy_handle_t command_handle,
                                         const char *  fqdid,
                                         const char *  cache_options,
                                         void          (*fn)(indy_handle_t command_handle_, indy_error_t err, const char *const diddoc)
                                        );

extern indy_error_t indy_vdr_resolve_schema(indy_handle_t command_handle,
                                            const char *  fqschema,
                                            const char *  cache_options,
                                            void          (*fn)(indy_handle_t command_handle_, indy_error_t err, const char *const schema)
                                           );

extern indy_error_t indy_vdr_resolve_cred_def(indy_handle_t command_handle,
                                              const char *  fqcreddef,
                                              const char *  cache_options,
                                              void          (*fn)(indy_handle_t command_handle_, indy_error_t err, const char *const cred_def)
                                             );

#ifdef __cplusplus
}
#endif

#endif