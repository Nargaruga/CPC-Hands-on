pub fn plan_holiday(n: usize, days: usize, cities: Vec<Vec<u32>>) -> u32 {
    // Compute itineraries (prefix sums)
    let mut itineraries: Vec<Vec<u32>> = vec![vec!(0; days); n];
    for city in 0..n {
        let mut prefix = 0;
        for day in 0..days {
            prefix += cities[city][day];
            itineraries[city][day] = prefix;
        }
    }

    let mut dp_matrix: Vec<Vec<u32>> = vec![vec!(0; days + 1); n + 1];

    // Note: we always need to do -1 when indexing "itineraries" as,
    // unlike "matrix", it doesn't contain padding 0s.
    for i in 1..n + 1 {
        for d in 1..days + 1 {
            // We consider three options:
            let without_current_city = dp_matrix[i - 1][d];
            let only_current_city = itineraries[i - 1][d - 1];
            let mut with_current_city = 0;

            // Compute the best combination that considers both the current city
            // and the previous ones
            for k in 1..d + 1 {
                with_current_city = u32::max(
                    // Get the best way to spend d-k days without city i
                    // and sum it to the score for spending the remaining k days in city i
                    dp_matrix[i - 1][d - k] + itineraries[i - 1][k - 1],
                    // Current best considering the current city too
                    with_current_city,
                );
            }

            // Store the best out of the three options
            dp_matrix[i][d] = u32::max(
                without_current_city,
                u32::max(with_current_city, only_current_city),
            );
        }
    }

    dp_matrix[n][days]
}
