fn main() {
    let mut stacks_9000: Vec<Vec<char>> = Vec::new();
    let mut stacks_9001: Vec<Vec<char>> = Vec::new();

    for line in include_str!("../../../../inputs/05.txt").lines() {
        if line.trim().starts_with('[') {
            for i in 0..(line.len() + 1) / 4 {
                if stacks_9000.len() < i + 1 {
                    stacks_9000.push(Vec::new());
                    stacks_9001.push(Vec::new());
                }
                let block = &line[i * 4..(i + 1) * 4 - 1];
                if block.trim().is_empty() {
                    continue;
                }
                let mut block_chars = block.chars();
                block_chars.next().expect("No char");
                let item = block_chars.next().expect("No char");
                stacks_9000[i].push(item);
                stacks_9001[i].push(item);
            }
        }

        if line.trim().is_empty() {
            for i in 0..stacks_9001.len() {
                stacks_9000[i].reverse();
                stacks_9001[i].reverse();
            }
        }

        if line.starts_with("move") {
            let mut line_elements = line.split(' ');
            line_elements.next().expect("no move");
            let amount = line_elements
                .next()
                .expect("no amount")
                .parse::<usize>()
                .expect("amount is no int");
            line_elements.next().expect("no from");
            let origin = line_elements
                .next()
                .expect("no amount")
                .parse::<usize>()
                .expect("amount is no int")
                - 1;
            line_elements.next().expect("no to");
            let target = line_elements
                .next()
                .expect("no amount")
                .parse::<usize>()
                .expect("amount is no int")
                - 1;

            let mut temp_stack = Vec::new();
            for _ in 0..amount {
                let item_to_move_9000 = stacks_9000[origin].pop().expect("stack empty");
                stacks_9000[target].push(item_to_move_9000);

                let item_to_move_9001 = stacks_9001[origin].pop().expect("stack empty");
                temp_stack.push(item_to_move_9001);
            }
            temp_stack.reverse();
            for item in temp_stack {
                stacks_9001[target].push(item);
            }
        }
    }

    for stack in stacks_9000 {
        print!("{}", stack.last().expect("stack empty"));
    }
    println!();

    for stack in stacks_9001 {
        print!("{}", stack.last().expect("stack empty"));
    }
    println!();
}
