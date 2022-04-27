=============deploy authorized_contract(hasn\'t authorized yet)===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 50000000000 \
--session-path /home/jh/mywork/contractaccess/contract/target/wasm32-unknown-unknown/release/authorized_contract.wasm \
--secret-key /home/jh/keys/test99/secret_key.pem 

=> 
authorized_contract
hash-bc9a09a6fc3ce9589f8e80843fbd2020dbacfffee8bc215e88b2e59a58df4ffd

=============deploy main contract (also authorize above contract)===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 60000000000 \
--session-arg "authorized_contract:String='contract-bc9a09a6fc3ce9589f8e80843fbd2020dbacfffee8bc215e88b2e59a58df4ffd'" \
--session-path /home/jh/mywork/contractaccess/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--secret-key /home/jh/keys/test99/secret_key.pem 

=>my_contract
hash-bd29a4c62605f9c2aa4ce99f92231ef73749301aadfb52d72e41baa8a033d1ed

=============call_access_other from authorized_contract===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 1000000000 \
--session-hash hash-bc9a09a6fc3ce9589f8e80843fbd2020dbacfffee8bc215e88b2e59a58df4ffd \
--session-entry-point "call_access_other" \
--session-arg "main_contract:String='contract-bd29a4c62605f9c2aa4ce99f92231ef73749301aadfb52d72e41baa8a033d1ed'" \
--secret-key /home/jh/keys/test1/secret_key.pem 


=============call test2 from other account directly===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 5000000000 \
--session-hash hash-bd29a4c62605f9c2aa4ce99f92231ef73749301aadfb52d72e41baa8a033d1ed \
--session-entry-point "test2" \
--secret-key /home/jh/keys/test1/secret_key.pem 

=>
"error_message": "Invalid context"

=============deploy unauthorized_contract===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 40000000000 \
--session-path /home/jh/mywork/contractaccess/contract/target/wasm32-unknown-unknown/release/unauthorized_contract.wasm \
--secret-key /home/jh/keys/test99/secret_key.pem 

=> 
unauthorized_contract
hash-15560625d5c9eed5ec3527ab6049b766ee82034888ecff8a0097179d7ba20093

=============call_access_other from unauthorized_contract===========================

casper-client put-deploy \
--chain-name casper-test \
--node-address http://3.208.91.63:7777 \
--payment-amount 5000000000 \
--session-hash hash-15560625d5c9eed5ec3527ab6049b766ee82034888ecff8a0097179d7ba20093 \
--session-entry-point "call_access_other" \
--session-arg "main_contract:String='contract-bd29a4c62605f9c2aa4ce99f92231ef73749301aadfb52d72e41baa8a033d1ed'" \
--secret-key /home/jh/keys/test1/secret_key.pem 

=>
"error_message": "Invalid context"