#![allow(dead_code)]

use std::io::BufWriter;

use assert_approx_eq::assert_approx_eq;

use super::*;

fn rounded(it:f32,places:i32) -> f32{
    let e = 10_f32.powi(places);
    (it * e).round() / e 
}

// #[test]
fn test_sort() {
    let mut sortable:Slots= [2,6,5,1,5].into();
    let presorted:Slots = [1,2,5,5,6].into();
    sortable.sort();
    assert_eq!(sortable.data, presorted.data);
}

// #[test]
fn test_dievals_into(){
    let dv:DieVals = [2,2,5,5,5].into();
    assert_eq!(dv.get(4), 5);
    assert_eq!(dv.get(0), 2);
    eprintln!("{}",dv);
}


// #[test]
fn score_slot_test() {
    assert_eq!(0, score_slot(FULL_HOUSE,[1,2,5,5,5].into() ));
    assert_eq!(25, score_slot(FULL_HOUSE,[2,2,5,5,5].into()));
    assert_eq!(0, score_slot(YAHTZEE,[4,5,5,5,5].into()));
    assert_eq!(50, score_slot(YAHTZEE,[1,1,1,1,1].into()));
}


// #[test]
fn ev_of_yahtzee_in_1_roll() {
// see https://www.yahtzeemanifesto.com/yahtzee-odds.php 
    let game = GameState{   rolls_remaining: 1, 
                            sorted_open_slots: [YAHTZEE].into(),// array_vec!([Dieval;13] => YAHTZEE ), 
                            sorted_dievals: Default::default(), 
                            upper_bonus_deficit: INIT_DEFICIT , yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let _result = best_choice_ev(game, app);
    let in_1_odds = 6.0/7776.0; 
    assert_approx_eq!( _result.ev , in_1_odds * 50.0 );
}

// #[test]
fn ev_of_yahtzee_in_2_rolls() {
    let game = GameState{   rolls_remaining: 2, 
                            sorted_open_slots: [YAHTZEE].into(), 
                            sorted_dievals: Default::default(), 
                            upper_bonus_deficit: INIT_DEFICIT , yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let _result = best_choice_ev(game, app);
    let in_2 = 0.01263; //https://www.datagenetics.com/blog/january42012/    
    // let in_1 = 6.0/7776.0; //0.00077; 
    assert_approx_eq!( rounded( _result.ev ,3), rounded( (in_2)*50.0, 3) );
}

// #[test]
fn ev_of_yahtzee_in_3_rolls() {
// see https://www.yahtzeemanifesto.com/yahtzee-odds.php 
    let game = GameState{   rolls_remaining: 3, 
                            sorted_open_slots: [YAHTZEE].into(), 
                            sorted_dievals: Default::default(), 
                            upper_bonus_deficit: INIT_DEFICIT , yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let _result = best_choice_ev(game, app);
    assert_approx_eq!( rounded( _result.ev, 2), rounded( 0.04603 * 50.0, 2) );
}

// #[test] 
fn ev_of_smstraight_in_1() {
// see https://www.yahtzeemanifesto.com/yahtzee-odds.php 
    let game = GameState{   rolls_remaining: 1, 
                            sorted_open_slots: [SM_STRAIGHT].into(), 
                            sorted_dievals: [0,0,0,0,0].into(), 
                            upper_bonus_deficit: INIT_DEFICIT , yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let result = best_choice_ev(game, app);
    assert_eq!( rounded( result.ev / 30.0, 2), rounded( 0.1235 + 0.0309 , 2) );
}

// #[test]
fn ev_of_lgstraight_in_1() {
// see https://www.yahtzeemanifesto.com/yahtzee-odds.php 
    let game = GameState{   rolls_remaining: 1, 
                            sorted_open_slots: [LG_STRAIGHT].into(), 
                            sorted_dievals: Default::default(), 
                            upper_bonus_deficit: INIT_DEFICIT , yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let result = best_choice_ev(game, app);
    assert_eq!( rounded( result.ev  / 40.0, 4), rounded( 0.0309, 4) );
}

// #[test] 
fn ev_of_4ofakind_in_1() {
// see https://www.yahtzeemanifesto.com/yahtzee-odds.php 
    let game = GameState{   rolls_remaining: 1, 
                            sorted_open_slots: [FOUR_OF_A_KIND].into(), 
                            sorted_dievals: Default::default(), 
                            upper_bonus_deficit: INIT_DEFICIT , yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let result = best_choice_ev(game, app);
    assert_eq!( 
        rounded( result.ev  / 17.5, 3), // we divide EV by average dice-total to get odds
        rounded( 0.0193+0.00077, 3) //our 3 of a kind includes 4 of a kind & yahtzee
    );
}

// #[test] 
fn ev_of_3ofakind_in_1() {
// see https://www.yahtzeemanifesto.com/yahtzee-odds.php 
    let game = GameState{   rolls_remaining: 1, 
                            sorted_open_slots: [THREE_OF_A_KIND].into(), 
                            sorted_dievals: Default::default(), 
                            upper_bonus_deficit: INIT_DEFICIT , yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let result = best_choice_ev(game, app); 
    assert_eq!( 
        rounded( result.ev  / 17.5, 3), // we divide EV by average dice-total to get odds
        rounded( 0.1929+0.0193+0.0007, 3) //our 3 of a kind includes 4 of a kind & yahtzee
    );
}


// #[test]
fn make_permutations(){
    for n in 1_u8..=13_u8 {
        let filename = "perms".to_string() + &n.to_string();
        let mut f = BufWriter::with_capacity(1_000_000_000,File::create(filename).unwrap());
        for perms in &(1_u8..=n).into_iter().permutations(n as usize).chunks(1024) {
            bincode::serialize_into(&mut f,&perms.collect_vec()).unwrap();
        }
    }
}
// Output: 
//             17 Mar 12 10:23 perms1
//       65346752 Mar 12 10:23 perms10
//      758731056 Mar 12 10:23 perms11
//     9583774200 Mar 12 10:23 perms12
//   130816085400 Mar 12 10:32 perms13
//             28 Mar 12 10:23 perms2
//             74 Mar 12 10:23 perms3
//            296 Mar 12 10:23 perms4
//           1568 Mar 12 10:23 perms5
//          10088 Mar 12 10:23 perms6
//          75640 Mar 12 10:23 perms7
//         645440 Mar 12 10:23 perms8
//        6171800 Mar 12 10:23 perms9

// #[test]
fn print_misc() {
    // eprint!("{:?}", selection_ranges() );
    // eprint!("{:?}", all_selection_outcomes() );
    eprintln!("{:?}", selection_ranges() );
    // eprint!("{:?}", five_dice_combinations() );
    // eprint!("{:?}", die_index_combos() );
}

// #[test]
// fn unique_upper_totals_test() {
//     let s:Slots = [1,2,7].into();
//     assert_eq!(s.unique_upper_totals(), 16);
// }

#[test]
fn bench_test() {
    let game = GameState{   rolls_remaining: 0, 
                            sorted_open_slots: [SIXES, FOUR_OF_A_KIND, YAHTZEE].into(), 
                            sorted_dievals: Default::default(), 
                            upper_bonus_deficit: 30, 
                            yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let result = best_choice_ev(game, app);
    assert_eq!(rounded(result.ev,2),  21.8);
    // save_cache(&app);
}

// #[test]
fn progress_eta_test() {
    let game = GameState{   rolls_remaining: 3, 
                            sorted_open_slots: [1,2,3,4,5,6,7,8,9,10,11,12,13].into(), 
                            sorted_dievals: Default::default(), 
                            upper_bonus_deficit: 30, 
                            yahtzee_is_wild: false, };
    let app = &mut AppState::new(&game);
    let result = best_choice_ev(game, app);
}

// #[test]
fn test_permutations() {

    let mut hm = FxHashMap::<Slots,u8>::default();
    let mut tot:u64 = 0;

    
    // let b:slots = [1,2,3,4].into();
    // let c:slots = [5,6,7,8].into();

    // let b_ = std::thread::spawn(move || {
    //     for perm in a.permutations() { 
    //         eprintln!("{}", perm); 
    //     };
    // });

    // let c_ = std::thread::spawn(move || {
    //     for perm in a { 
    //         eprintln!("{}", perm); 
    //     };
    // });

    // b_.join().unwrap();
    // c_.join().unwrap();

    let a:Slots = [1,2,3,4,5,6,7,8].into();
    for perm in a.permutations_within(8,4) { 
        if let Some(s) = hm.get(&perm) {
            eprintln!("{} {}", perm, s) ;
            tot+=*s as u64;
        } else {
            let s = perm.into_iter().sum() ;
            hm.insert(perm, s);
            eprintln!("{} {}", perm, s) ;
            tot+=s as u64;
        }
    }; 
    eprintln!("{}", tot); // 1451520 1.40s

 }

 