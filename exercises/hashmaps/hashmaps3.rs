// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Debug)]
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by
        // team_1.
        // NOTE: or_insert or_insert_with 区别
        // 通常，后面有 with 的区别:
        // or_insert：直接携带默认值，或者执行一个函数获取具体的值。
        // or_insert_with：参数中是一个匿名函数，懒惰执行
        // or_insert 功能：如果 entry 对应的值存在的话，不执行。如果不存在的话，才会将默认的值插入进去
        // or_insert 即便之前的值已经存在，也会计算出 default 默认值，只是最后扔掉了。
        // or_insert_with：只有在原本不存在 key 的时候，才会执行闭包
        let e = scores.entry(team_1_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        e.goals_scored += team_1_score;
        e.goals_conceded += team_2_score;

        // || 相当于创建了一个闭包
        let e = scores.entry(team_2_name).or_insert_with(|| Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        e.goals_scored += team_2_score;
        e.goals_conceded += team_1_score;
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        println!("{:?}", team);
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
