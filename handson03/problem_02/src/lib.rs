use std::error::Error;

pub fn count_reverse_flags(houses: &str) -> Result<u32, Box<dyn Error>> {
    let mut r: u32 = 0; // #of 'R' substrings
    let mut rw: u32 = 0; // #of 'RW' substrings
    let mut rwg: u32 = 0; // # of 'RWG' substrings (full flags)
    let mut x: u32 = 0; // #of 'X'

    // We perform a single linear scan of the array of houses
    for house in houses.trim().chars() {
        match house {
            'R' => {
                r += 1;
            }
            'W' => {
                // All the previous 'R' strings can be promoted to 'RW'
                rw += r;
            }
            'G' => {
                // All the previous 'RW' strings can be promoted to 'RWG'
                rwg += rw;
            }
            'X' => {
                // We need to count the number of flags found up to now three times
                //  (once for each possible value of this X).
                //  We then add the number of flags that end in the current X.
                rwg = rwg * 3 + rw;

                // RWGR <- rw = 1
                // RWGW <- r2 = 2 (one from RWG and the other due to X being counted as W)
                // RWGG <- r2 = 1
                // We count the previous rw three times and we promote all R to RW using
                //   the current X as W.
                rw = rw * 3 + r;

                // RWGR <- r = 2 (one from RWG and the other due to X being counted as R)
                // RWGW <- r = 1
                // RWGG <- r = 1
                // We count the previous r three times, then we add 3^x, because if we fix
                //  the current as R, every previous X can vary among the three possible values,
                //  so that's 3 * 3 * 3 *...* 3 x times.
                r = r * 3 + u32::pow(3, x);

                x += 1;
            }
            _ => return Err(format!("{} is not a valid character!", house))?,
        }
    }

    Ok(rwg)
}
