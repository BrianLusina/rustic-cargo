pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut cumulative_gas = 0;
    let mut cumulative_cost = 0;

    let mut gas_tank = 0;
    let mut start_index = 0;
    for i in 0..gas.len() {
        cumulative_gas += gas[i];
        cumulative_cost += cost[i];
        gas_tank += gas[i] - cost[i];
        if gas_tank < 0 {
            gas_tank = 0;
            start_index = i + 1;
        }
    }

    if cumulative_gas < cumulative_cost {
        return -1;
    }
    start_index as i32
}

#[cfg(test)]
mod can_complete_circut_tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }
}
