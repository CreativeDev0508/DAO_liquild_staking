export NODE_ENV=testnet
near delete diversifying.pool.testnet asimov.testnet
near create-account diversifying.pool.testnet --masterAccount pool.testnet
. deploy-testnet.sh
near call diversifying.pool.testnet new '{"owner_account_id":"lucio.testnet"}' --accountId pool.testnet
