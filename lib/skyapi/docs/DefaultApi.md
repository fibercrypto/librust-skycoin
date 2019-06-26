# \DefaultApi

All URIs are relative to *http://127.0.0.1:6420*

Method | HTTP request | Description
------------- | ------------- | -------------
[**address_count**](DefaultApi.md#address_count) | **Get** /api/v1/addresscount | Returns the total number of unique address that have coins.
[**address_uxouts**](DefaultApi.md#address_uxouts) | **Get** /api/v1/address_uxouts | 
[**api_v1_rawtx_get**](DefaultApi.md#api_v1_rawtx_get) | **Get** /api/v1/rawtx | 
[**api_v2_metrics_get**](DefaultApi.md#api_v2_metrics_get) | **Get** /api/v2/metrics | 
[**balance_get**](DefaultApi.md#balance_get) | **Get** /api/v1/balance | Returns the balance of one or more addresses, both confirmed and predicted. The predicted balance is the confirmed balance minus the pending spends.
[**balance_post**](DefaultApi.md#balance_post) | **Post** /api/v1/balance | Returns the balance of one or more addresses, both confirmed and predicted. The predicted balance is the confirmed balance minus the pending spends.
[**block**](DefaultApi.md#block) | **Get** /api/v1/block | Returns the balance of one or more addresses, both confirmed and predicted. The predicted balance is the confirmed balance minus the pending spends.
[**blockchain_metadata**](DefaultApi.md#blockchain_metadata) | **Get** /api/v1/blockchain/metadata | Returns the blockchain metadata.
[**blockchain_progress**](DefaultApi.md#blockchain_progress) | **Get** /api/v1/blockchain/progress | Returns the blockchain sync progress.
[**blocks**](DefaultApi.md#blocks) | **Get** /api/v1/blocks | Returns the balance of one or more addresses, both confirmed and predicted. The predicted balance is the confirmed balance minus the pending spends.
[**coin_supply**](DefaultApi.md#coin_supply) | **Get** /api/v1/coinSupply | 
[**csrf**](DefaultApi.md#csrf) | **Get** /api/v1/csrf | Creates a new CSRF token. Previous CSRF tokens are invalidated by this call.
[**data_delete**](DefaultApi.md#data_delete) | **Delete** /api/v2/data | 
[**data_get**](DefaultApi.md#data_get) | **Get** /api/v2/data | 
[**data_post**](DefaultApi.md#data_post) | **Post** /api/v2/data | 
[**default_connections**](DefaultApi.md#default_connections) | **Get** /api/v1/network/defaultConnections | defaultConnectionsHandler returns the list of default hardcoded bootstrap addresses.\\n They are not necessarily connected to.
[**health**](DefaultApi.md#health) | **Get** /api/v1/health | Returns node health data.
[**last_blocks**](DefaultApi.md#last_blocks) | **Get** /api/v1/last_blocks | 
[**network_connection**](DefaultApi.md#network_connection) | **Get** /api/v1/network/connection | This endpoint returns a specific connection.
[**network_connections**](DefaultApi.md#network_connections) | **Get** /api/v1/network/connections | This endpoint returns all outgoings connections.
[**network_connections_disconnect**](DefaultApi.md#network_connections_disconnect) | **Post** /api/v1/network/connection/disconnect | 
[**network_connections_exchange**](DefaultApi.md#network_connections_exchange) | **Get** /api/v1/network/connections/exchange | 
[**network_connections_trust**](DefaultApi.md#network_connections_trust) | **Get** /api/v1/network/connections/trust | trustConnectionsHandler returns all trusted connections.\\n They are not necessarily connected to. In the default configuration, these will be a subset of the default hardcoded bootstrap addresses.
[**outputs_get**](DefaultApi.md#outputs_get) | **Get** /api/v1/outputs | If neither addrs nor hashes are specificed, return all unspent outputs. If only one filter is specified, then return outputs match the filter. Both filters cannot be specified.
[**outputs_post**](DefaultApi.md#outputs_post) | **Post** /api/v1/outputs | If neither addrs nor hashes are specificed, return all unspent outputs. If only one filter is specified, then return outputs match the filter. Both filters cannot be specified.
[**pending_txs**](DefaultApi.md#pending_txs) | **Get** /api/v1/pendingTxs | 
[**resend_unconfirmed_txns**](DefaultApi.md#resend_unconfirmed_txns) | **Post** /api/v1/resendUnconfirmedTxns | 
[**richlist**](DefaultApi.md#richlist) | **Get** /api/v1/richlist | Returns the top skycoin holders.
[**transaction**](DefaultApi.md#transaction) | **Get** /api/v1/transaction | 
[**transaction_inject**](DefaultApi.md#transaction_inject) | **Post** /api/v1/injectTransaction | Broadcast a hex-encoded, serialized transaction to the network.
[**transaction_post**](DefaultApi.md#transaction_post) | **Post** /api/v2/transaction | 
[**transaction_post_unspent**](DefaultApi.md#transaction_post_unspent) | **Post** /api/v2/transaction/unspent | 
[**transaction_raw**](DefaultApi.md#transaction_raw) | **Get** /api/v2/transaction/raw | Returns the hex-encoded byte serialization of a transaction. The transaction may be confirmed or unconfirmed.
[**transaction_verify**](DefaultApi.md#transaction_verify) | **Post** /api/v2/transaction/verify | 
[**transactions_get**](DefaultApi.md#transactions_get) | **Get** /api/v1/transactions | Returns transactions that match the filters.
[**transactions_post**](DefaultApi.md#transactions_post) | **Post** /api/v1/transactions | Returns transactions that match the filters.
[**uxout**](DefaultApi.md#uxout) | **Get** /api/v1/uxout | Returns an unspent output by ID.
[**verify_address**](DefaultApi.md#verify_address) | **Post** /api/v2/address/verify | Verifies a Skycoin address.
[**version**](DefaultApi.md#version) | **Get** /api/v1/version | 
[**wallet**](DefaultApi.md#wallet) | **Get** /api/v1/wallet | Returns a wallet by id.
[**wallet_balance**](DefaultApi.md#wallet_balance) | **Get** /api/v1/wallet/balance | Returns the wallet's balance, both confirmed and predicted.  The predicted balance is the confirmed balance minus the pending spends.
[**wallet_create**](DefaultApi.md#wallet_create) | **Post** /api/v1/wallet/create | 
[**wallet_decrypt**](DefaultApi.md#wallet_decrypt) | **Post** /api/v1/wallet/decrypt | Decrypts wallet.
[**wallet_encrypt**](DefaultApi.md#wallet_encrypt) | **Post** /api/v1/wallet/encrypt | Encrypt wallet.
[**wallet_folder**](DefaultApi.md#wallet_folder) | **Get** /api/v1/wallets/folderName | 
[**wallet_new_address**](DefaultApi.md#wallet_new_address) | **Post** /api/v1/wallet/newAddress | 
[**wallet_new_seed**](DefaultApi.md#wallet_new_seed) | **Get** /api/v1/wallet/newSeed | 
[**wallet_recover**](DefaultApi.md#wallet_recover) | **Post** /api/v2/wallet/recover | Recovers an encrypted wallet by providing the seed. The first address will be generated from seed and compared to the first address of the specified wallet. If they match, the wallet will be regenerated with an optional password. If the wallet is not encrypted, an error is returned.
[**wallet_seed**](DefaultApi.md#wallet_seed) | **Post** /api/v1/wallet/seed | This endpoint only works for encrypted wallets. If the wallet is unencrypted, The seed will be not returned.
[**wallet_seed_verify**](DefaultApi.md#wallet_seed_verify) | **Post** /api/v2/wallet/seed/verify | Verifies a wallet seed.
[**wallet_transaction**](DefaultApi.md#wallet_transaction) | **Post** /api/v1/wallet/transaction | Creates a signed transaction
[**wallet_transaction_sign**](DefaultApi.md#wallet_transaction_sign) | **Post** /api/v2/wallet/transaction/sign | Creates a signed transaction
[**wallet_transactions**](DefaultApi.md#wallet_transactions) | **Get** /api/v1/wallet/transactions | 
[**wallet_unload**](DefaultApi.md#wallet_unload) | **Post** /api/v1/wallet/unload | Unloads wallet from the wallet service.
[**wallet_update**](DefaultApi.md#wallet_update) | **Post** /api/v1/wallet/update | Update the wallet.
[**wallets**](DefaultApi.md#wallets) | **Get** /api/v1/wallets | 



## address_count

> ::models::InlineResponse200 address_count()
Returns the total number of unique address that have coins.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## address_uxouts

> Vec<Value> address_uxouts(address)


Returns the historical, spent outputs associated with an address

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address** | **String**| address to filter by | 

### Return type

[**Vec<Value>**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v1_rawtx_get

> String api_v1_rawtx_get()


### Required Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_v2_metrics_get

> String api_v2_metrics_get()


### Required Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance_get

> Value balance_get(addrs)
Returns the balance of one or more addresses, both confirmed and predicted. The predicted balance is the confirmed balance minus the pending spends.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **addrs** | **String**| command separated list of addresses | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance_post

> Value balance_post(ctx, addrs)
Returns the balance of one or more addresses, both confirmed and predicted. The predicted balance is the confirmed balance minus the pending spends.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **addrs** | **String**| command separated list of addresses | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block

> Vec<::models::BlockSchema> block(optional)
Returns the balance of one or more addresses, both confirmed and predicted. The predicted balance is the confirmed balance minus the pending spends.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hash** | **String**| get block by hash | 
 **seq** | **i32**| get block by sequence number | 

### Return type

[**Vec<::models::BlockSchema>**](blockSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blockchain_metadata

> Value blockchain_metadata()
Returns the blockchain metadata.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blockchain_progress

> Value blockchain_progress()
Returns the blockchain sync progress.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blocks

> ::models::InlineResponse2001 blocks(optional)
Returns the balance of one or more addresses, both confirmed and predicted. The predicted balance is the confirmed balance minus the pending spends.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **start** | **i32**| start seq | 
 **end** | **i32**| end seq | 
 **seq** | [**Vec<i32>**](i32.md)| comma-separated list of block seqs | 

### Return type

[**::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## coin_supply

> ::models::InlineResponse2002 coin_supply()


coinSupplyHandler returns coin distribution supply stats

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## csrf

> ::models::InlineResponse2003 csrf()
Creates a new CSRF token. Previous CSRF tokens are invalidated by this call.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_delete

> data_delete(optional)


### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**| storage type. | 
 **key** | **String**| key of the specific value to get. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_get

> Value data_get(optional)


### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**| storage type. | 
 **key** | **String**| key of the specific value to get. | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## data_post

> data_post(optional)


### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **_type** | **String**| storage type. | 
 **key** | **String**| key of the specific value to get. | 
 **val** | **String**| additional value. | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## default_connections

> Vec<String> default_connections()
defaultConnectionsHandler returns the list of default hardcoded bootstrap addresses.\\n They are not necessarily connected to.

### Required Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health

> Value health()
Returns node health data.

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## last_blocks

> Value last_blocks(num)


Returns the most recent N blocks on the blockchain

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **num** | **i32**| Num of blockss | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connection

> ::models::NetworkConnectionSchema network_connection(addr)
This endpoint returns a specific connection.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **addr** | **String**| Address port | 

### Return type

[**::models::NetworkConnectionSchema**](networkConnectionSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections

> ::models::InlineResponse2004 network_connections(ctx, optional)
This endpoint returns all outgoings connections.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **states** | **String**| Connection status. | 
 **direction** | **String**| Direction of the connection. | 

### Return type

[**::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections_disconnect

> network_connections_disconnect(ctx, id)


This endpoint disconnects a connection by ID or address

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Address id. | 

### Return type

 (empty response body)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections_exchange

> Vec<String> network_connections_exchange()


This endpoint returns all connections found through peer exchange

### Required Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## network_connections_trust

> Vec<String> network_connections_trust()
trustConnectionsHandler returns all trusted connections.\\n They are not necessarily connected to. In the default configuration, these will be a subset of the default hardcoded bootstrap addresses.

### Required Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outputs_get

> Value outputs_get(optional)
If neither addrs nor hashes are specificed, return all unspent outputs. If only one filter is specified, then return outputs match the filter. Both filters cannot be specified.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | [**Vec<String>**](String.md)|  | 
 **hash** | [**Vec<String>**](String.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outputs_post

> Value outputs_post(ctx, optional)
If neither addrs nor hashes are specificed, return all unspent outputs. If only one filter is specified, then return outputs match the filter. Both filters cannot be specified.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **address** | **String**|  | 
 **hash** | **String**|  | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pending_txs

> Vec<::models::InlineResponse20010> pending_txs()


### Required Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<::models::InlineResponse20010>**](inline_response_200_10.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resend_unconfirmed_txns

> Value resend_unconfirmed_txns(ctx, )


Broadcasts all unconfirmed transactions from the unconfirmed transaction pool

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application-json, application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## richlist

> Value richlist(optional)
Returns the top skycoin holders.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **include_distribution** | **bool**| include distribution addresses or not, default value false | 
 **n** | **String**| include distribution addresses or not, default value false | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction

> ::models::Transaction transaction(txid)


Returns a transaction identified by its txid hash with just id

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **txid** | **String**| transaction Id | 

### Return type

[**::models::Transaction**](transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_inject

> String transaction_inject(ctx, rawtx)
Broadcast a hex-encoded, serialized transaction to the network.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **rawtx** | **String**| hex-encoded serialized transaction string. | 

### Return type

**String**

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_post

> ::models::InlineResponse2008 transaction_post(ctx, optional)


### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **transaction_v2_params_address** | [**TransactionV2ParamsAddress**](TransactionV2ParamsAddress.md)|  | 

### Return type

[**::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_post_unspent

> ::models::InlineResponse2008 transaction_post_unspent(ctx, transaction_v2_params_unspent)


### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **transaction_v2_params_unspent** | [**TransactionV2ParamsUnspent**](TransactionV2ParamsUnspent.md)| Unspent parameters | 

### Return type

[**::models::InlineResponse2008**](inline_response_200_8.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_raw

> Value transaction_raw(optional)
Returns the hex-encoded byte serialization of a transaction. The transaction may be confirmed or unconfirmed.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **txid** | **String**| Transaction id hash | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transaction_verify

> Value transaction_verify(ctx, transaction_verify_request)


Decode and verify an encoded transaction

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **transaction_verify_request** | [**TransactionVerifyRequest**](TransactionVerifyRequest.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_get

> Value transactions_get(optional)
Returns transactions that match the filters.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addrs** | **String**| command separated list of addresses | 
 **confirmed** | **String**| Whether the transactions should be confirmed [optional, must be 0 or 1; if not provided, returns all] | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transactions_post

> Value transactions_post(ctx, optional)
Returns transactions that match the filters.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addrs** | **String**| command separated list of addresses | 
 **confirmed** | **String**| Whether the transactions should be confirmed [optional, must be 0 or 1; if not provided, returns all] | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uxout

> Value uxout(optional)
Returns an unspent output by ID.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **uxid** | **String**| uxid to filter by | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_address

> Value verify_address(ctx, address)
Verifies a Skycoin address.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **address** | [**Value**](.md)| Address id. | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## version

> ::models::InlineResponse2005 version()


versionHandler returns the application version info

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet

> Value wallet(id)
Returns a wallet by id.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| tags to filter by | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_balance

> Value wallet_balance(id)
Returns the wallet's balance, both confirmed and predicted.  The predicted balance is the confirmed balance minus the pending spends.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| tags to filter by | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_create

> Value wallet_create(ctx, seed, label, optional)


Loads wallet from seed, will scan ahead N address and load addresses till the last one that have coins.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **seed** | **String**| Wallet seed. | 
  **label** | **String**| Wallet label. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **seed** | **String**| Wallet seed. | 
 **label** | **String**| Wallet label. | 
 **scan** | **i32**| The number of addresses to scan ahead for balances. | 
 **encrypt** | **bool**| Encrypt wallet. | 
 **password** | **String**| Wallet Password | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_decrypt

> Value wallet_decrypt(ctx, id, password)
Decrypts wallet.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Wallet id. | 
  **password** | **String**| Wallet password. | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_encrypt

> Value wallet_encrypt(ctx, id, password)
Encrypt wallet.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Wallet id. | 
  **password** | **String**| Wallet password. | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_folder

> ::models::InlineResponse2007 wallet_folder(addr)


Returns the wallet directory path

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **addr** | **String**| Address port | 

### Return type

[**::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_new_address

> Value wallet_new_address(ctx, id, optional)


Generates new addresses

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Wallet Id | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Wallet Id | 
 **num** | **String**| The number you want to generate | 
 **password** | **String**| Wallet Password | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_new_seed

> Value wallet_new_seed(optional)


Returns the wallet directory path

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **entropy** | **String**| Entropy bitSize. | 

### Return type

[**Value**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_recover

> Value wallet_recover(ctx, id, seed, optional)
Recovers an encrypted wallet by providing the seed. The first address will be generated from seed and compared to the first address of the specified wallet. If they match, the wallet will be regenerated with an optional password. If the wallet is not encrypted, an error is returned.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Wallet id. | 
  **seed** | **String**| Wallet seed. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| Wallet id. | 
 **seed** | **String**| Wallet seed. | 
 **password** | **String**| Wallet password. | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_seed

> Value wallet_seed(ctx, id, password)
This endpoint only works for encrypted wallets. If the wallet is unencrypted, The seed will be not returned.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Wallet Id. | 
  **password** | **String**| Wallet password. | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_seed_verify

> Value wallet_seed_verify(ctx, optional)
Verifies a wallet seed.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters

Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **seed** | **String**| Seed to be verified. | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transaction

> Value wallet_transaction(ctx, wallet_transaction_request)
Creates a signed transaction

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **wallet_transaction_request** | [**WalletTransactionRequest**](WalletTransactionRequest.md)|  | 

### Return type

[**Value**](Value.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transaction_sign

> ::models::InlineResponse2009 wallet_transaction_sign(ctx, wallet_transaction_sign_request)
Creates a signed transaction

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **wallet_transaction_sign_request** | [**WalletTransactionSignRequest**](WalletTransactionSignRequest.md)|  | 

### Return type

[**::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_transactions

> ::models::InlineResponse2006 wallet_transactions(id)


### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| Wallet Id. | 

### Return type

[**::models::InlineResponse2006**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_unload

> wallet_unload(ctx, id)
Unloads wallet from the wallet service.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Wallet Id. | 

### Return type

 (empty response body)

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallet_update

> String wallet_update(ctx, id, label)
Update the wallet.

### Required Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| Wallet Id. | 
  **label** | **String**| The label the wallet will be updated to. | 

### Return type

**String**

### Authorization

[csrfAuth](../README.md#csrfAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wallets

> Vec<Value> wallets()


Returns all loaded wallets

### Required Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<Value>**](Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

