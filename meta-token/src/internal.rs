use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{AccountId, Balance, PromiseResult};

use crate::*;

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
pub const MIN_TRANSFER_UNIT: u128 = 1000; // to make sibyl attacks more expensive in terms of tokens

pub fn default_ft_metadata() -> FungibleTokenMetadata {
    FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "Meta Token".to_string(),
        symbol: "$META".to_string(),
        icon: Some(String::from(
            r###"data:image/svg+xml,%3Csvg viewBox='0 0 67.79 67.79' xmlns='http://www.w3.org/2000/svg'%3E%3Cellipse style='fill:%23000;fill-opacity:1' cx='34.277' cy='33.937' rx='31.899' ry='31.984'%3E%3C/ellipse%3E%3Cpath style='fill:%23797981;fill-opacity:1;stroke-width:.385777' d='M31.006 57.549c.857-.546.364-1.785-.711-1.785-.626 0-1 .25-1.091.727-.12.626.479 1.389 1.09 1.389.106 0 .426-.15.712-.331zm8.138-2.559c7.554-1.742 7.31-1.658 7.31-2.525 0-.548-.501-.882-1.993-1.327-2.254-.673-3.008-.741-3.008-.27 0 .178.952.554 2.116.835 1.164.28 2.116.62 2.116.756 0 .205-2.688.872-11.928 2.958-.794.179 2.57-2.32 5-3.714.315-.18.46-.442.324-.58-.27-.27-1.255.238-4.554 2.35-1.164.746-2.34 1.49-2.613 1.654-.701.422-.143 1.675.639 1.433.345-.107 3.311-.813 6.591-1.57zm-10.445.424c-.415-.33-11.447-6.032-11.67-6.032-.146 0-.203.188-.127.418.195.59 11.631 6.457 11.797 6.054.075-.183.075-.38 0-.44zm1.147-.534c-1.636-3.873-3.014-6.029-3.014-4.713 0 .754 2.32 5.017 2.73 5.017.227 0 .354-.137.284-.304zm19.11-2.404c0-.708-.232-1-.867-1.09-.52-.075-.96.119-1.1.483-.383 1.004.154 1.833 1.1 1.698.635-.09.867-.382.867-1.09zM41.233 50.4c.322-.528.256-.793-.298-1.207-.94-.703-1.79-.333-1.79.778 0 1.41 1.324 1.682 2.088.43zm10.33-2.512c1.437-2.206 2.137-3.642 1.98-4.055-.135-.352-.38-.64-.545-.64-.598 0-11.16 5.9-11.16 6.233 0 .487.478.425 1.357-.176 1.36-.93 9.801-5.304 9.801-5.08 0 .12-.871 1.517-1.936 3.103-2.092 3.116-2.564 4.046-2.049 4.037.181-.004 1.33-1.543 2.553-3.422zm-12.803 2.31c0-.443-.79-.624-6.097-1.397-3.865-.563-5.181-.57-4.64-.026.142.143 8.884 1.616 10.256 1.728.264.022.48-.116.48-.305zM16.25 48.99c0-.686-.234-.995-.816-1.079-.978-.14-1.515.938-.861 1.73.69.836 1.677.452 1.677-.651zm10.966-.188c.382-.463.382-.698 0-1.16-.263-.32-.696-.58-.962-.58-.736 0-1.389.985-1.12 1.69.302.79 1.45.817 2.082.05zm-2.693-.473c0-.344-.225-.49-.555-.363-.305.118-1.955.248-3.666.29-1.712.041-3.208.232-3.326.423-.122.2 1.448.322 3.667.287 3.153-.05 3.88-.169 3.88-.637zm13.063-5.338c-1.106-2.654-2.161-4.826-2.343-4.826-.479 0-.495-.052 1.444 4.72 2.183 5.373 2.535 6.069 2.75 5.424.089-.27-.744-2.663-1.85-5.318zm7.489.415c3.014-4.746 4.2-7.144 2.777-5.614-.567.61-6.784 10.367-6.784 10.647 0 .887 1.224-.65 4.007-5.033zm-28.243.85c.349-1.65.562-3.071.474-3.16-.088-.088.201-.355.643-.592.964-.519 1.056-1.771.157-2.118-.978-.377-1.956.767-1.408 1.648.237.383.296.946.13 1.25-.166.306-.513 1.6-.771 2.876-.258 1.277-.562 2.477-.676 2.667-.114.19-1.203-2.333-2.42-5.608-1.217-3.275-2.152-6.02-2.076-6.098.075-.08 1.257.752 2.626 1.847s2.593 1.822 2.72 1.615c.214-.347-4.552-4.286-5.185-4.286-1.407 0-.86 2.175 3.046 12.114.316.807.647 1.059 1.267.967.713-.105.935-.576 1.473-3.122zm5.354-.51c-1.752-1.772-3.347-3.121-3.543-3-.197.123 1.077 1.672 2.83 3.443 1.752 1.77 3.347 3.12 3.543 2.998.197-.122-1.077-1.67-2.83-3.442zm8.303-.786c3.06-4.27 3.587-5.181 3.002-5.181-.192 0-1.683 1.909-3.312 4.243-2.638 3.779-3.338 5.04-2.796 5.04.092 0 1.49-1.846 3.106-4.102zm25.16.476c.635-.769.104-1.79-.93-1.79-1.025 0-1.386.67-.888 1.644.415.812 1.215.876 1.818.146zm-1.883-2.038c0-.568-2.894-4.006-3.372-4.006-.43 0-.433.12-.017.695 2.003 2.77 3.389 4.124 3.389 3.311zm1.71-6.636c.198-3.315.156-4.72-.138-4.72-.23 0-.426.654-.437 1.451-.01.798-.162 3.191-.336 5.319-.313 3.802-.134 5.547.373 3.64.142-.533.385-3.094.539-5.69zm-28.74 3.544c3.438-.568 6.252-1.18 6.252-1.36 0-.18-.273-.328-.607-.328-1.376 0-12.696 2.058-13.033 2.37-.587.542.71.423 7.387-.682zm-8.619-2.558c.634-5.748.705-7.252.339-7.252-.212 0-.486.913-.61 2.03-.634 5.748-.704 7.253-.338 7.253.212 0 .486-.914.61-2.031zm17.372.865c0-.762-.2-.966-.962-.984-.53-.012-1.395-.302-1.924-.644-3.132-2.025-12.026-7.263-12.333-7.263-.2 0-.36.13-.353.29.014.356 12.462 7.833 13.04 7.833.229 0 .416.333.416.74 0 .786.585 1.19 1.539 1.065.338-.044.577-.474.577-1.037zm14.205.544c.335-.13.609-.58.609-1.002 0-.422.952-1.921 2.116-3.33 2.04-2.47 2.443-3.17 1.828-3.168-.412.002-4.156 4.734-4.04 5.105.052.17-.343.309-.878.309-1.134 0-1.63.917-.946 1.745.545.66.507.651 1.311.341zm-5.491-.539c2.014 0 3.406-.158 3.406-.387 0-.228-1.392-.386-3.406-.386-4.46 0-8.137.399-8.137.883 0 .243.88.295 2.365.139 1.301-.137 3.898-.249 5.772-.249zm-8.717-6.672c.168-2.823.132-4.545-.097-4.545-.36 0-1.217 9.184-.888 9.515.475.478.739-.853.985-4.97zm12.829 4.807c-.31-.768-11.152-10.135-11.489-9.926-.206.128 2.154 2.485 5.244 5.238 5.011 4.465 6.651 5.696 6.245 4.688zM10.64 33.97c.44-.827-.123-1.606-1.159-1.606-.836 0-1.262 1.1-.693 1.79.611.74 1.401.661 1.852-.184zm38.286-3.734a477.765 477.765 0 0 1-.533-4.042c-.008-.095-.164-.008-.347.194-.251.277.148 5.08.688 8.277.016.096.183.009.37-.193.2-.215.125-1.98-.178-4.236zm-34.306.027c2.297-1.69 3.086-2.54 2.357-2.54-.129 0-1.384.836-2.79 1.858-1.404 1.023-2.756 1.982-3.004 2.131-.247.15.937-2.701 2.633-6.335s2.98-6.71 2.855-6.837c-.35-.352-.369-.317-3.624 6.669-2.726 5.85-2.987 6.605-2.43 7.015.344.253.802.344 1.018.203.215-.141 1.559-1.115 2.985-2.164zm41.799-1.138c.632-.766.105-1.79-.922-1.79-.484 0-.988.283-1.12.63-.27.705.383 1.691 1.12 1.691.266 0 .68-.239.922-.531zm-36.74-1.645c.278-.73-.392-1.692-1.18-1.692-.654 0-1.068.942-.763 1.74.296.776 1.641.742 1.943-.048zm34.757-1.493c-.568-1.07-4.955-8.476-5.14-8.676-.08-.089-.259-.05-.395.088-.136.136.99 2.327 2.501 4.869 1.512 2.54 2.748 4.723 2.748 4.85 0 .125-1.03-.294-2.288-.933-1.376-.698-2.384-1.007-2.527-.775-.13.213.842.9 2.162 1.527 2.217 1.055 2.435 1.091 2.876.485.35-.482.367-.862.063-1.435zm-27.028-.233c3.598-.657 6.614-1.266 6.702-1.354.626-.622-1.14-.471-6.774.578-3.557.662-6.598 1.202-6.757 1.2-.159-.002-.289.171-.289.384 0 .514-.323.55 7.118-.808zm-4.556-2.7c1.448-1.505 2.692-3.028 2.764-3.385.141-.7-4.117 3.301-5.381 5.056-1.417 1.966-.039 1.086 2.617-1.672zm26.16 2.15c.263-.693-.5-1.737-1.27-1.737-.655 0-1.068.943-.763 1.741.288.754 1.744.75 2.032-.005zM18.608 23.95c-.116-.798-.32-2.364-.455-3.481-.134-1.117-.421-2.03-.637-2.03-.372 0-.232 2.386.336 5.704.303 1.773 1.014 1.591.756-.193zm27.846.71c0-.195-1.342-.462-2.982-.593-1.64-.132-3.718-.331-4.617-.444-.984-.123-1.636-.047-1.636.189 0 .472 1.63.777 5.002.935 1.376.065 2.891.151 3.367.192.476.041.866-.085.866-.28zm-9.747-.643c.309-.81-.718-1.806-1.536-1.49-.355.137-.645.665-.645 1.174 0 .733.203.926.974.926.536 0 1.079-.275 1.207-.61zm-2.566-1.284c0-.342-5.431-3.908-5.952-3.908-.728 0 .246.903 2.682 2.487 2.77 1.802 3.27 2.019 3.27 1.42zM42.368 20c2.678-1.694 5.098-2.991 5.378-2.883.684.264 1.547-.837 1.255-1.602-.352-.92-1.897-.54-2.146.529-.114.49-.315.839-.446.773-.32-.16-9.547 5.603-9.562 5.973-.024.612.803.194 5.521-2.79zm6.01.372c0-1.547-.164-2.707-.384-2.707s-.385 1.16-.385 2.707.165 2.708.385 2.708.385-1.16.385-2.708zM35.675 21.63c-.012-.429-.716-5.508-.966-6.962-.119-.691-.395-1.257-.614-1.257-.228 0-.308.453-.188 1.064.116.585.409 2.5.652 4.254.243 1.755.596 3.191.783 3.191.187 0 .337-.13.333-.29zm-7.816-3.027c.309-.81-.718-1.806-1.535-1.49-.644.248-.877 1.352-.39 1.842.48.482 1.691.26 1.925-.352zm-2.566-.524c0-.461-.804-.69-3.576-1.017-1.227-.145-2.17-.324-2.096-.398.074-.074 2.722-.946 5.885-1.937 3.163-.992 6.011-1.902 6.329-2.023.317-.12-.462.756-1.732 1.948s-2.174 2.304-2.01 2.471c.267.273 4.52-3.584 4.505-4.086-.034-1.191-.86-1.09-7.133.876-5.527 1.732-6.52 2.159-6.52 2.801 0 .64.423.827 2.693 1.196 3.582.582 3.655.585 3.655.169zm-7.118-1.192c0-.686-.234-.995-.817-1.079-.978-.14-1.514.938-.86 1.73.69.836 1.677.452 1.677-.651zm28.28-1.504c0-.192-1.254-.726-2.789-1.185-6.815-2.043-8.213-2.519-8.784-2.991-1.374-1.138-2.95.148-1.709 1.395.496.498.731.517 1.377.112.428-.27.964-.374 1.19-.234.504.313 9.564 3.156 10.235 3.212.265.022.48-.117.48-.31z'%3E%3C/path%3E%3Cpath d='M53.54 54.643h-4.55V28.435q0-3.105.382-7.6h-.109q-.654 2.642-1.171 3.786l-13.35 30.022H32.51L19.187 24.84q-.573-1.307-1.172-4.004h-.109q.218 2.342.218 7.655v26.153h-4.413V15.577h6.048L31.746 42.82q1.39 3.133 1.798 4.686h.163q1.172-3.215 1.88-4.795L47.82 15.577h5.721zm3.049-44.488a33.15 34.668 0 1 0 0 49.025 33.15 34.668 0 0 0 0-49.025zm-23.782 55.11a29.267 30.608 0 1 1 29.267-30.597 29.267 30.608 0 0 1-29.267 30.597z' transform='scale(1.02265 .97786)' aria-label='M' style='font-size:55.7943px;line-height:1.25;font-family:Ebrima;-inkscape-font-specification:Ebrima;fill:%23fff;stroke-width:1.39485'%3E%3C/path%3E%3C/svg%3E"###,
        )),
        reference: Some("https://metapool.app".into()),
        reference_hash: None,
        decimals: 24,
    }
}

impl MetaToken {
    pub fn assert_owner_calling(&self) {
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "can only be called by the owner"
        );
    }

    pub fn assert_minter(&self, account_id: String) {
        assert!(self.minters.contains(&account_id), "not a minter");
    }

    //get stored metadata or default
    pub fn internal_get_ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap_or(default_ft_metadata())
    }

    pub fn internal_unwrap_balance_of(&self, account_id: &AccountId) -> Balance {
        self.accounts.get(&account_id).unwrap_or(0)
    }

    pub fn mint_into(&mut self, account_id: &AccountId, amount: Balance) {
        let balance = self.internal_unwrap_balance_of(account_id);
        self.internal_update_account(&account_id, balance + amount);
        self.total_supply += amount;
    }

    pub fn internal_burn(&mut self, account_id: &AccountId, amount: u128) {
        let balance = self.internal_unwrap_balance_of(account_id);
        assert!(balance >= amount);
        self.internal_update_account(&account_id, balance - amount);
        assert!(self.total_supply >= amount);
        self.total_supply -= amount;
    }

    pub fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: Balance,
        memo: Option<String>,
    ) {
        assert_ne!(
            sender_id, receiver_id,
            "Sender and receiver should be different"
        );

        if self.locked_until_nano > 0 && env::block_timestamp() < self.locked_until_nano {
            panic!(
                "transfers are locked until unix timestamp {}",
                self.locked_until_nano / NANOSECONDS
            );
        }

        let sender_balance = self.internal_unwrap_balance_of(sender_id);
        assert!(
            amount == sender_balance || amount > ONE_NEAR / MIN_TRANSFER_UNIT,
            "The amount should be at least 1/{}",
            MIN_TRANSFER_UNIT
        );

        // remove from sender
        let sender_balance = self.internal_unwrap_balance_of(sender_id);
        assert!(
            amount <= sender_balance,
            "The account doesn't have enough balance {}",
            sender_balance
        );
        let balance_left = sender_balance - amount;
        self.internal_update_account(&sender_id, balance_left);

        // check vesting
        if self.vested_count > 0 {
            match self.vested.get(&sender_id) {
                Some(vesting) => {
                    //compute locked
                    let locked = vesting.compute_amount_locked();
                    if locked == 0 {
                        //vesting is complete. remove vesting lock
                        self.vested.remove(&sender_id);
                        self.vested_count -= 1;
                    } else if balance_left < locked {
                        panic!("Vested account, balance can't go lower than {}", locked);
                    }
                }
                None => {}
            }
        }

        // add to receiver
        let receiver_balance = self.internal_unwrap_balance_of(receiver_id);
        self.internal_update_account(&receiver_id, receiver_balance + amount);

        log!("Transfer {} from {} to {}", amount, sender_id, receiver_id);
        if let Some(memo) = memo {
            log!("Memo: {}", memo);
        }
    }

    /// Inner method to save the given account for a given account ID.
    pub fn internal_update_account(&mut self, account_id: &AccountId, balance: u128) {
        self.accounts.insert(account_id, &balance); //insert_or_update
    }

    // TODO rename
    pub fn int_ft_resolve_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: ValidAccountId,
        amount: U128,
    ) -> (u128, u128) {
        let sender_id: AccountId = sender_id.into();
        let receiver_id: AccountId = receiver_id.into();
        let amount: Balance = amount.into();

        // Get the unused amount from the `ft_on_transfer` call result.
        let unused_amount = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(value) => {
                if let Ok(unused_amount) = near_sdk::serde_json::from_slice::<U128>(&value) {
                    std::cmp::min(amount, unused_amount.0)
                } else {
                    amount
                }
            }
            PromiseResult::Failed => amount,
        };

        if unused_amount > 0 {
            let receiver_balance = self.accounts.get(&receiver_id).unwrap_or(0);
            if receiver_balance > 0 {
                let refund_amount = std::cmp::min(receiver_balance, unused_amount);
                self.accounts
                    .insert(&receiver_id, &(receiver_balance - refund_amount));

                if let Some(sender_balance) = self.accounts.get(&sender_id) {
                    self.accounts
                        .insert(&sender_id, &(sender_balance + refund_amount));
                    log!(
                        "Refund {} from {} to {}",
                        refund_amount,
                        receiver_id,
                        sender_id
                    );
                    return (amount - refund_amount, 0);
                } else {
                    // Sender's account was deleted, so we need to burn tokens.
                    self.total_supply -= refund_amount;
                    log!("The account of the sender was deleted");
                    return (amount, refund_amount);
                }
            }
        }
        (amount, 0)
    }
}
