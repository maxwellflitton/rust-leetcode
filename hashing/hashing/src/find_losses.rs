use std::collections::HashMap;

pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut loss_counts: HashMap<i32, i32> = HashMap::new();

    for game in matches {
        let winner = game[0];
        let loser = game[1];

        // Ensure the winner is recorded, even if they never lose.
        loss_counts.entry(winner).or_insert(0);

        // Record another loss for the loser.
        *loss_counts.entry(loser).or_insert(0) += 1;
    }

    let mut no_losses = Vec::new();
    let mut one_loss = Vec::new();

    for (player, losses) in loss_counts {
        match losses {
            0 => no_losses.push(player),
            1 => one_loss.push(player),
            _ => {}
        }
    }

    no_losses.sort_unstable();
    one_loss.sort_unstable();

    vec![no_losses, one_loss]
}

pub fn find_winners_using_vec(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    const MAX_PLAYER: usize = 100_000;

    // -1 means the player never appeared.
    //  0 means they appeared but never lost.
    //  1+ is their loss count.
    let mut losses = vec![-1; MAX_PLAYER + 1];

    for game in matches {
        let winner = game[0] as usize;
        let loser = game[1] as usize;

        if losses[winner] == -1 {
            losses[winner] = 0;
        }

        if losses[loser] == -1 {
            losses[loser] = 1;
        } else {
            losses[loser] += 1;
        }
    }

    let mut no_losses = Vec::new();
    let mut one_loss = Vec::new();

    for player in 1..=MAX_PLAYER {
        match losses[player] {
            0 => no_losses.push(player as i32),
            1 => one_loss.push(player as i32),
            _ => {}
        }
    }

    vec![no_losses, one_loss]
}
