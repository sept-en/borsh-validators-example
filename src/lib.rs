use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ValidatorStakeV1 {
    pub account_id: String,
    pub stake: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ValidatorStakeV2 {
    pub account_id: String,
    pub stake: u128,
    pub is_chunk_only: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum ValidatorStake {
    V1(ValidatorStakeV1),
    V2(ValidatorStakeV2),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borshify_stake() {
        let account_id = String::from("test_stake.near");
        let stake = 3_400_500u128;

        let stake_v1 = ValidatorStakeV1 {
            account_id: account_id.clone(),
            stake: stake,
        };
        let stake_v2 = ValidatorStakeV2 {
            account_id: account_id.clone(),
            stake: stake,
            is_chunk_only: false,
        };

        let borshified_stake_v1 = stake_v1.try_to_vec().unwrap();
        let borshified_stake_v2 = stake_v2.try_to_vec().unwrap();
        println!("ValidatorStakeV1 (borshified): {:?}", borshified_stake_v1);
        println!("ValidatorStakeV2 (borshified): {:?}", borshified_stake_v2);

        let expected_borshified_stake_v1_len = 35;
        let expected_borshified_stake_v2_len = 36;
        assert_eq!(borshified_stake_v1.len(), expected_borshified_stake_v1_len);
        assert_eq!(borshified_stake_v2.len(), expected_borshified_stake_v2_len);

        let stake_enum_v1 = ValidatorStake::V1(stake_v1);
        let stake_enum_v2 = ValidatorStake::V2(stake_v2);
        let borshified_enum_stake_v1 = stake_enum_v1.try_to_vec().unwrap();
        let borshified_enum_stake_v2 = stake_enum_v2.try_to_vec().unwrap();
        println!("ValidatorStake::V1 (borshified): {:?}", borshified_enum_stake_v1);
        println!("ValidatorStake::V2 (borshified): {:?}", borshified_enum_stake_v2);

        // Enum adds 1 byte prefix to the structure
        let expected_borshified_enum_stake_v1_len = expected_borshified_stake_v1_len + 1;
        let expected_borshified_enum_stake_v2_len = expected_borshified_stake_v2_len + 1;
        assert_eq!(borshified_enum_stake_v1.len(), expected_borshified_enum_stake_v1_len);
        assert_eq!(borshified_enum_stake_v2.len(), expected_borshified_enum_stake_v2_len);
    }
}
