
macro_rules! recurrence {
    (a[n]: $sty:ty = $($inits:expr),+; ... ;$recur:expr) => {

        use std::ops::Index;

        struct Recurrence {
            mem: [$sty;2],
            pos: usize,
        }

        struct IndexOffset<'a> {
            slice: &'a [$sty;2],
            offset: usize,
        }

        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = $sty;
            fn index(&self, index: usize) -> &$sty {
                use std::num::Wrapping;

                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let window = Wrapping(2);

                let real_index = index - offset + window;
                &self.slice[real_index.0]
            }
        }

        impl Iterator for Recurrence {
            type Item = $sty;
            #[inline]
            fn next(&mut self) -> Option<$sty> {
                if self.pos < 2 {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                }else {
                    let next_val = {
                        let n = self.pos;
                        let a = IndexOffset {
                            slice: &self.mem,
                            offset: n,
                        };
                        (a[n - 1] + a[n - 2])
                    };

                    {
                        use std::mem::swap;

                        let mut swap_tmp = next_val;
                        for i in (0..2).rev() {
                            swap(&mut swap_tmp, &mut self.mem[i]);
                        }
                    }
                    self.pos += 1;
                    Some(next_val)
                }
            }
        }

        Recurrence {mem: [$($inits),+], pos: 0}
    };
}


fn main() {
    let fib = recurrence![a[n]: u64 = 0,1;...;a[n-1] + a[n-2]];
    for e in fib.take(10) {
        println!("{}", e);
    }
}