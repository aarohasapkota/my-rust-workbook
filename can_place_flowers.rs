impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        // if no flowers to be planted return -> true
        if n == 0{
            return true;
        }

        let mut count = 0;
        let mut flowerbed = flowerbed;// make mutable copy of flowerbed

        for i in 0..flowerbed.len(){
            if flowerbed[i] == 0 // check if spot is empty
            && (i == 0 || flowerbed[i-1] == 0) // check for left boundary
            && (i == flowerbed.len() - 1 || flowerbed[i+1] == 0) // check for right boundary
            {
                flowerbed[i] = 1; // plant the flower
                count += 1; // Increase the count
                if n <= count{ // Check if we reached the target or not
                    return true;
                }
            }
        }
        false
        
    }

}
