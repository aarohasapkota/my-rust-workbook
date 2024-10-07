impl Solution{
    fn merge_alternately(word1: String, word2: String) -> String{
        //Declare merged, iter1, iter2
        let mut merged = String::with_capacity(word1.len() + word2.len());
        let mut iter1 = word1.chars();
        let mut iter2 = word2.chars();

        // start  loop
        loop{
            // match next character
            match(iter1.next(), iter2.next()){

                //exhaust all the cases
                (Some(c1),Some(c2)) => {
                    merged.push(c1);
                    merged.push(c2);
                }
                (Some(c1), None) => {
                    merged.push(c1);
                    merged.extend(iter1);
                    break;
                }
                (None, Some(c2)) => {
                    merged.push(c2);
                    merged.extend(iter2);
                    break;
                }
                (None,None) => {break;}
            }
            

        }
        merged
        
    }
    
}



// impl Solution{
// fn merge_alternately(word1: String, word2: String) -> String {
//         let mut merged = String::with_capacity(word1.len()+word2.len());
//         let mut iter1 = word1.chars();
//         let mut iter2 = word2.chars();

//         loop{
//             match (iter1.next(), iter2.next()){
//                 (Some(c1), Some(c2)) => {
//                     merged.push(c1);
//                     merged.push(c2);
//                 },
//                 (None, Some(c2)) => {
//                     merged.push(c2);
//                     merged.extend(iter2);
//                     break Default::default();
//                 },
//                 (Some(c1), None) => {
//                     merged.push(c1);
//                     merged.extend(iter1);
//                     break Default::default();
            
//                 },
//                 (None, None) => break Default::default(),

                
//             }
//         }
//         merged
//     }
// }




    
        

   
    
