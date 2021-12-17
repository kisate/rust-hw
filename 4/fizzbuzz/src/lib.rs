use std::{fmt, ops::Rem, process::Output};

// MIT License
//
// Copyright (c) 2021 Exercism
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Все упоминания `PhantomData` в этом файле можно убрать, они только для того,
// чтобы текущий код компилировался

/// Правило для FizzBuzz: с помощью заданного предиката мы проверяем, должен ли
/// текущий элемент T быть заменен на слово? Если да, то на какое?
pub struct Matcher<T> {
    pub predicate: Box<dyn Fn(T) -> bool>,
    pub substitute: String,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static>(predicate: F, substitute: &str) -> Self {
        Matcher {
            predicate: Box::new(predicate),
            substitute: substitute.to_string(),
        }
    }
}

/// Набор правил Matcher, которые можно применить к итератору.
///
/// Более идиоматично использовать метод `.map()` для модификации итератора
/// вместо метода `Fizzy::apply()`, который этот итератор поглощает.
///
/// Зато можно попрактиковаться с более простым интерфейсом `apply`.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T> {
    // можете использовать `mut self` в сигнатуре, если вам нравится
    pub fn add_matcher(self, _matcher: Matcher<T>) -> Self {
        let mut matchers = self.matchers;
        matchers.push(_matcher);
        Self { matchers }
    }
}

impl<T: fmt::Display + Copy> Fizzy<T> {
    /// Применяет набор Matchers к каждому элементу итератора
    pub fn apply<I: Iterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String> {
        _iter.map(move |elem| {
            self.matchers
                .iter()
                .filter(|matcher| (matcher.predicate)(elem))
                .fold(None, |cum: Option<String>, mat| match cum {
                    Some(subs) => {
                        let mut new_subs = subs;
                        new_subs.push_str(&mat.substitute);
                        Some(new_subs)
                    }
                    None => Some(mat.substitute.clone()),
                })
                .unwrap_or(elem.to_string())
        })
    }
}

impl<T> Fizzy<T> {
    fn new() -> Self {
        Self { matchers: vec![] }
    }
}

/// Вспомогательная функция: возвращает `Fizzy` со стандартными правилами FizzBuzz
pub fn fizz_buzz<T: Rem<Output = T> + From<u8> + PartialEq>() -> Fizzy<T> {
    Fizzy {
        matchers: vec![
            Matcher::new(|x| x % T::from(3) == T::from(0), "fizz"),
            Matcher::new(|x| x % T::from(5) == T::from(0), "buzz"),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! expect {
        () => {
            vec![
                "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz",
                "13", "14", "fizzbuzz", "16",
            ]
        };
    }

    #[test]
    fn test_simple() {
        let got = fizz_buzz::<i32>().apply(1..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_u8() {
        let got = fizz_buzz::<u8>().apply(1_u8..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_u64() {
        let got = fizz_buzz::<u64>().apply(1_u64..=16).collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_nonsequential() {
        let collatz_12 = &[12, 6, 3, 10, 5, 16, 8, 4, 2, 1];
        let expect = vec![
            "fizz", "fizz", "fizz", "buzz", "buzz", "16", "8", "4", "2", "1",
        ];
        let got = fizz_buzz::<i32>()
            .apply(collatz_12.iter().cloned())
            .collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_custom() {
        let expect = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];
        let fizzer: Fizzy<i32> = Fizzy::new()
            .add_matcher(Matcher::new(|n: i32| n % 5 == 0, "Buzz"))
            .add_matcher(Matcher::new(|n: i32| n % 3 == 0, "Fizz"))
            .add_matcher(Matcher::new(|n: i32| n % 7 == 0, "Bam"));
        let got = fizzer.apply(1..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn test_f64() {
        // a tiny bit more complicated because range isn't natively implemented on floats
        let got = fizz_buzz::<f64>()
            .apply(std::iter::successors(Some(1.0), |prev| Some(prev + 1.0)))
            .take(16)
            .collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }

    #[test]
    fn test_minimal_generic_bounds() {
        use std::fmt;
        use std::ops::{Add, Rem};

        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        struct Fizzable(u8);

        impl From<u8> for Fizzable {
            fn from(i: u8) -> Fizzable {
                Fizzable(i)
            }
        }

        impl fmt::Display for Fizzable {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let Fizzable(ref n) = self;
                write!(f, "{}", n)
            }
        }

        impl Add for Fizzable {
            type Output = Fizzable;
            fn add(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 + n2)
            }
        }

        impl Rem for Fizzable {
            type Output = Fizzable;
            fn rem(self, rhs: Fizzable) -> Fizzable {
                let Fizzable(n1) = self;
                let Fizzable(n2) = rhs;
                Fizzable(n1 % n2)
            }
        }

        let got = fizz_buzz::<Fizzable>()
            .apply(std::iter::successors(Some(Fizzable(1)), |prev| {
                Some(*prev + 1.into())
            }))
            .take(16)
            .collect::<Vec<_>>();
        assert_eq!(expect!(), got);
    }
}
