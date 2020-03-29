use std::collections::BTreeMap;

struct TopVotedCandidate {
    winner: BTreeMap<i32, i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut score = vec![0; persons.len()];
        let mut winner = BTreeMap::new();
        let mut cur_winner = 0i32;
        for i in 0..times.len() {
            let person = persons[i];
            score[person as usize] += 1;
            winner.insert(
                times[i],
                if score[person as usize] >= score[cur_winner as usize] {
                    cur_winner = person;
                    person
                } else {
                    cur_winner
                },
            );
        }

        Self { winner }
    }

    fn q(&self, t: i32) -> i32 {
        *self.winner.range(0..=t).last().unwrap().1
    }
}

#[test]
fn case1() {
    let persons = vec![0, 0, 0, 0, 1];
    let times = vec![0, 6, 39, 52, 75];
    let votes = TopVotedCandidate::new(persons, times);
    assert_eq!(votes.q(98), 0);
}
