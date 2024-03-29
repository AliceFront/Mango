// Copyright (c) The Mango Blockchain Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::vm_validator::TransactionValidation;
use anyhow::Result;
use diem_state_view::StateView;
use diem_vm::VMValidator;
use mango_types::{
    account_address::AccountAddress,
    on_chain_config::OnChainConfigPayload,
    transaction::{GovernanceRole, SignedTransaction, VMValidatorResult},
    vm_status::StatusCode,
};

#[derive(Clone)]
pub struct MockVMValidator;

impl VMValidator for MockVMValidator {
    fn validate_transaction(
        &self,
        _transaction: SignedTransaction,
        _state_view: &dyn StateView,
    ) -> VMValidatorResult {
        VMValidatorResult::new(None, 0, GovernanceRole::NonGovernanceRole)
    }
}

impl TransactionValidation for MockVMValidator {
    type ValidationInstance = MockVMValidator;
    fn validate_transaction(&self, txn: SignedTransaction) -> Result<VMValidatorResult> {
        let txn = match txn.check_signature() {
            Ok(txn) => txn,
            Err(_) => {
                return Ok(VMValidatorResult::new(
                    Some(StatusCode::INVALID_SIGNATURE),
                    0,
                    GovernanceRole::NonGovernanceRole,
                ))
            }
        };

        let sender = txn.sender();
        let account_dne_test_add = AccountAddress::new([0_u8; AccountAddress::LENGTH]);
        let invalid_sig_test_add = AccountAddress::new([1_u8; AccountAddress::LENGTH]);
        let insufficient_balance_test_add = AccountAddress::new([2_u8; AccountAddress::LENGTH]);
        let seq_number_too_new_test_add = AccountAddress::new([3_u8; AccountAddress::LENGTH]);
        let seq_number_too_old_test_add = AccountAddress::new([4_u8; AccountAddress::LENGTH]);
        let txn_expiration_time_test_add = AccountAddress::new([5_u8; AccountAddress::LENGTH]);
        let invalid_auth_key_test_add = AccountAddress::new([6_u8; AccountAddress::LENGTH]);
        let ret = if sender == account_dne_test_add {
            Some(StatusCode::SENDING_ACCOUNT_DOES_NOT_EXIST)
        } else if sender == invalid_sig_test_add {
            Some(StatusCode::INVALID_SIGNATURE)
        } else if sender == insufficient_balance_test_add {
            Some(StatusCode::INSUFFICIENT_BALANCE_FOR_TRANSACTION_FEE)
        } else if sender == seq_number_too_new_test_add {
            Some(StatusCode::SEQUENCE_NUMBER_TOO_NEW)
        } else if sender == seq_number_too_old_test_add {
            Some(StatusCode::SEQUENCE_NUMBER_TOO_OLD)
        } else if sender == txn_expiration_time_test_add {
            Some(StatusCode::TRANSACTION_EXPIRED)
        } else if sender == invalid_auth_key_test_add {
            Some(StatusCode::INVALID_AUTH_KEY)
        } else {
            None
        };
        Ok(VMValidatorResult::new(
            ret,
            0,
            GovernanceRole::NonGovernanceRole,
        ))
    }

    fn restart(&mut self, _config: OnChainConfigPayload) -> Result<()> {
        unimplemented!();
    }
}
