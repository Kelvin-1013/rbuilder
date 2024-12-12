use alloy_primitives::{Address, U256};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TreasuryError {
    #[error("Invalid fee percentage: {0}")]
    InvalidFeePercentage(u8),
    #[error("Invalid treasury address")]
    InvalidTreasuryAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryConfig {
    pub address: Address,
    pub fee_percentage: u8,
}

impl TreasuryConfig {
    pub fn new(address: Address, fee_percentage: u8) -> Result<Self, TreasuryError> {
        if fee_percentage == 0 || fee_percentage >= 100 {
            return Err(TreasuryError::InvalidFeePercentage(fee_percentage));
        }
        
        Ok(Self {
            address,
            fee_percentage,
        })
    }

    pub fn calculate_fee_split(&self, total_value: U256) -> (U256, U256) {
        let fee_amount = total_value
            .checked_mul(U256::from(self.fee_percentage))
            .unwrap_or_default()
            .checked_div(U256::from(100))
            .unwrap_or_default();
            
        let remaining = total_value.checked_sub(fee_amount).unwrap_or_default();
        
        (fee_amount, remaining)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fee_calculation() {
        let treasury = TreasuryConfig::new(
            Address::random(),
            10
        ).unwrap();
        
        // Test with 100 ETH
        let total = U256::from(100_000_000_000_000_000_000u128); // 100 ETH in wei
        let (fee, remaining) = treasury.calculate_fee_split(total);
        
        assert_eq!(fee, U256::from(10_000_000_000_000_000_000u128)); // 10 ETH
        assert_eq!(remaining, U256::from(90_000_000_000_000_000_000u128)); // 90 ETH
    }
}
