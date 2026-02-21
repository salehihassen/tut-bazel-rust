// Copyright 2024 The Bazel examples and tutorials Authors & Contributors. // All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

fn fib_memo(n: usize, memo: &mut Vec<Option<u64>>) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            if let Some(val) = memo[n] {
                return val;
            }
            let result = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
            memo[n] = Some(result);
            result
        }
    }
}

fn fibonacci(n: usize) -> u64 {
    let mut memo = vec![None; n + 1];
    return fib_memo(n, &mut memo);
}

fn computation_and_print() {
    let start = std::time::Instant::now();
    let n = 40;
    let result = fibonacci(n);
    println!("Time: {:?} ; fib({})={}", start.elapsed(), n, result);
}

fn main() {
    computation_and_print();
}
