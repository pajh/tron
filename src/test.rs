fn test_run(no:usize, moves:&AvailMoves, expected:usize) {
    if moves.c == expected {
        println!("Test {} OK",no);
    } else {
        println!("****FAIL***** {} result:{} <> expected:{} [{:?}]",no, moves.c,expected,moves);
    }
}

fn test_runner() {
       let mut b = Board::new(false);
       let mut players = PlayerData::new(3, 0);
       let mut tree = MoveTree::new();
       let mut node:usize = 0;

        // Base calc() checks
       test_run(1, &AvailMoves::calc((1,1)), 4 );
       test_run(2, &AvailMoves::calc((0,0)), 2);
       test_run(3, &AvailMoves::calc((0,19)),2);
       test_run(4, &AvailMoves::calc((29,19)),2);
       test_run(5, &AvailMoves::calc((29,0)),2);
       test_run(6, &AvailMoves::calc((13,0)),3);
       test_run(7, &AvailMoves::calc((29,9)),3);

    // empty board_calc() checks
       test_run(8, &AvailMoves::calc_board(&b,(1,1)),4);
       test_run(8, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (1,1), &players),4);
       test_run(9, &AvailMoves::calc_board(&b,(0,0)),2);
       test_run(9, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (0,0), &players),2);
       test_run(10, &AvailMoves::calc_board(&b,(0,19)),2);
       test_run(10, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (0,19), &players),2);
       test_run(11, &AvailMoves::calc_board(&b,(29,19)),2);
       test_run(11, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (29,19), &players),2);
       test_run(12, &AvailMoves::calc_board(&b,(29,0)),2);
       test_run(12, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (29,0), &players),2);
       test_run(13, &AvailMoves::calc_board(&b,(13,0)),3);       
       test_run(13, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (13,0), &players),3);
       test_run(14, &AvailMoves::calc_board(&b,(29,9)),3);
       test_run(14, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (29,9), &players),3);

       b.set( (1,0), 1, 0);
       test_run(151, &AvailMoves::calc_board(&b,(0,0)),1);
       test_run(152, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (0,0), &players),1);
       test_run(16, &AvailMoves::calc_board(&b,(1,1)),3);
       test_run(16, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (1,1), &players),3);
       test_run(17, &AvailMoves::calc_board(&b,(2,0)),2);
       test_run(17, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (2,0), &players),2);

       b.set( (1,1), 1, 1);
       test_run(18, &AvailMoves::calc_board(&b,(2,1)),3);
       test_run(18, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (2,1), &players),3);
       test_run(19, &AvailMoves::calc_board(&b,(0,1)),2);
       test_run(19, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (0,1), &players),2);
       node = tree.add((0,2), node, 0);
       node = tree.add((1,2), node, 0);
       node = tree.add((2,2), node, 0);
       test_run(20, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (0,3), &players),2);
       test_run(21, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (1,3), &players),3);
       test_run(22, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (0,1), &players),1);
       test_run(23, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (2,1), &players),2);
       test_run(24, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (5,5), &players),4);
       node = tree.add((3,1), node, 1);
       node = tree.add((3,2), node, 1);
       node = tree.add((3,3), node, 1);
       test_run(25, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (2,1), &players),1);
       test_run(26, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (2,3), &players),2);
       players.players[0].dead = true; // kill p0
       test_run(27, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (0,0), &players),2);
       test_run(28, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (2,1), &players),2);
      test_run(29, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (2,0), &players),3);
      test_run(30, &AvailMoves::calc_board_tree_dead(&b, &tree, node, (2,3), &players),3);
}
