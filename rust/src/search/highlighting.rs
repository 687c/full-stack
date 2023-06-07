// use std::vec;

// // Type to mark characters as highlighted
// #[derive(Debug)]
// enum Fragment {
//     Highlighted(char),
//     Normal(char),
// }

// // This is the function doing the highlighting
// // Takes search_query and mpn and returns vector of mpn characters with highlighting
// // TODO: fix errors in this function
// fn highlight_search_query_in_mpn(search_query: &str, mpn: &str) -> Vec<Fragment> {
//     let mut string_fragments: Vec<Fragment> = Vec::new();
//     let mut mpn_chars = mpn.chars();
//     for query_char in search_query.chars() {
//         while let Some(mpn_char) = mpn_chars.next() {
//             if mpn_char == query_char {
//                 string_fragments.push(Fragment::Highlighted(mpn_char));
//                 break;
//             } else {
//                 string_fragments.push(Fragment::Normal(mpn_char));
//             }
//         }
//     }

//     //for the characters not consumed by the query search
//     for mpn_char in mpn_chars {
//         string_fragments.push(Fragment::Normal(mpn_char));
//     }

//     string_fragments
// }

use serde::Serialize;

// ! MY IMPLEMENTATION USING THE STRUCT ASSUMES THE SEARCH ORDER DOES NOT MATTER
#[derive(Debug, Serialize)]
pub struct Fragmented {
    highlighted: Vec<char>,
    normal: Vec<char>,
}

pub fn split_fragments(search_query: &str, mpn: &str) -> Fragmented {
    let mut fragmented_res: Fragmented = Fragmented {
        highlighted: Vec::new(),
        normal: Vec::new(),
    };

    // let mut string_fragments: Vec<Fragment> = Vec::new();
    let mut mpn_chars = mpn.chars();
    for query_char in search_query.chars() {
        while let Some(mpn_char) = mpn_chars.next() {
            if mpn_char == query_char && mpn_char != '-' {
                fragmented_res.highlighted.push(mpn_char);
                break;
            } /*  else {
                  fragmented_res.normal.push(mpn_char);
              } */
        }
        // fragmented_res.normal.push(mpn);

    }

    // //for the characters not consumed by the query search
    // for mpn_char in mpn_chars {
    //     fragmented_res.normal.push(mpn_char);
    // }
    fragmented_res.normal = mpn.chars().collect();

    fragmented_res
}

#[test]
fn test_search_highlighting() {
    let mpn = "FH12-5S-1SH(55)";
    let search_query = "-";
    let res = split_fragments(search_query, mpn);

    assert_eq!(res.highlighted.len(), 0); //since - is ignored
    assert_eq!(res.normal.len(), mpn.len()); //since - is ignored

    let search_query2 = "FH";
    let res = split_fragments(search_query2, mpn);
    assert_eq!(res.highlighted.len(), 2);
    assert_eq!(res.normal.len(), mpn.len());
    assert!(res.highlighted.contains(&'F'));
    assert!(!res.highlighted.contains(&'f')); //fails since 'f' !== 'F'
}
