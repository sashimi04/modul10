## Experiment 1.2: Understanding how it works

### Output
![Hasil Run](screenshots/experiment1_2.png)

### Explanation
The output shows that "hey hey" is printed immediately after the task is spawned,
even before the async block runs.

This happens because `spawner.spawn(...)` only schedules the task — it doesn't block.
The executor (`executor.run()`) is the one that polls and runs the async task.
Therefore, the line after `spawn` (i.e., `println!("hey hey")`) runs first.

This confirms how the executor operates: tasks are scheduled, not immediately executed.
