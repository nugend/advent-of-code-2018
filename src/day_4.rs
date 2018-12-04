use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;


#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Box<HashMap<u16,Vec<u8>>> {
    struct Event<'a>{time:&'a str,minute:u8,event:&'a str};
    let re = regex::Regex::new(r"\[(?P<time>\d\d\d\d-\d\d-\d\d \d\d:(?P<minute>\d\d))\]\s+(?P<event>.*)").unwrap();
    let mut ev:Vec<Event> = re.captures_iter(input).map(|c|Event{time: c.name("time").unwrap().as_str(),minute: c.name("minute").unwrap().as_str().parse::<u8>().unwrap(),event: c.name("event").unwrap().as_str()}).collect();
    ev.sort_unstable_by_key(|x|x.time);
    let mut guard_info = HashMap::new();
    let mut current_guard = 0;
    for e in ev {
        if e.event.contains("Guard") {
            current_guard = e.event.matches(char::is_numeric).collect::<String>().parse::<u16>().unwrap();
        } else {
            (*guard_info.entry(current_guard).or_insert(Vec::new())).push(e.minute);
        }
    }
    return Box::new(guard_info);
}

#[aoc(day4, part1, guards)]
pub fn sleepiest(input: &HashMap<u16,Vec<u8>>) -> u32 {
    let mut sleepiest = (0,0,HashMap::new());
    for (g,ts) in input.iter() {
        let sleep_sum = ts.chunks(2).map(|c|(c[1] - c[0]) as u16).sum();
        sleepiest = if sleep_sum > sleepiest.1 {
            let mut sleep_occurrences = HashMap::new();
            for m in ts.chunks(2).flat_map(|c|c[0]..c[1]){
                *sleep_occurrences.entry(m).or_insert(0)+=1;
            }
            (*g,sleep_sum,sleep_occurrences)
            } else { sleepiest };
    }
    return sleepiest.0 as u32 * *sleepiest.2.iter().max_by_key(|(_,v)| *v).unwrap().0 as u32;
}

#[aoc(day4, part2, guards)]
pub fn consistent_sleepiest(input: &HashMap<u16,Vec<u8>>) -> u32 {
    let mut sleepiest = (0,0,0);
    for (g,ts) in input.iter() {
        let mut sleep_occurrences = HashMap::new();
        for m in ts.chunks(2).flat_map(|c|c[0]..c[1]){
            *sleep_occurrences.entry(m).or_insert(0)+=1;
            }
        let (minute,sleeps) = sleep_occurrences.iter().max_by_key(|(_,v)| *v).unwrap();
        sleepiest = if *sleeps > sleepiest.2 {
            (*g,*minute,*sleeps)
            } else { sleepiest };
    }
    return sleepiest.0 as u32 * sleepiest.1 as u32;
}

