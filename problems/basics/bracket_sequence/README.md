# Correct Bracket Sequence

## Условие

Реализуйте функцию `is_correct_bracket_sequence`, проверяющую является ли строка правильной скобочной последовательностью.
Входная последовательность может состоять из нескольких типов скобок: `(`, `)`, `{`, `}`, `[`, `]`.

Сигнатура функции уже объявлена в файле [src/lib.rs](./src/lib.rs), Вам нужно лишь дописать код в теле функции.

## Пример

```rust
  is_correct_bracket_sequence("()") == true
  is_correct_bracket_sequence("{([])}") == true
  is_correct_bracket_sequence("(()") == false
  is_correct_bracket_sequence("([)]") == false
```

## Подсказки

Строки в Rust ([`&str`](https://doc.rust-lang.org/std/primitive.str.html), [`String`](https://doc.rust-lang.org/std/string/struct.String.html)) представляют валидные UTF-8 последовательности.
Итерироваться по ним можно двумя способоами:

1. По юникодным символам ([`char`](https://doc.rust-lang.org/std/primitive.char.html)):

   ```rust
   for ch in s.chars() {
     ...
   }
   ```

   По индексу можно обратиться через `s.chars().nth(i)` – однако это неэффективно, т.к. `.nth(i)` работает за линейное от длины строки время.

2. По байтам (`u8`):

   ```rust
   for byte in s.as_bytes() {
     ...
   }
   ```

Для реализации решения Вам может понадобиться стек. Однако, в стандартной библиотеке Rust нет отдельной структуры для стека, поэтому можно воспользоваться [вектором](https://doc.rust-lang.org/std/vec/struct.Vec.html) (см. также методы [`push`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push) и [`pop`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop)).

Скорее всего, Вам также придется столкнуться с типом `Option<T>` (например, при вызове методов [`pop()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop) или [`last()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.last)): https://doc.rust-lang.org/std/option/enum.Option.html.
Это `enum`, который может принимать одно из двух значений: `Some(value)`, то есть значение типа `T`, или `None`, то есть ничего.
На данный момент нам могут быть полезные некоторые из следующих операций с типом `Option<T>`:

1. Метод [`unwrap()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap) – возвращает значение типа T, если значение установлено, иначе кидает панику (и программа экстренно завершается).
   Соответственно, вызывать этот метод можно только когда известно, что значение точно установлено.

   ```rust
   let ch: char = stack.pop().unwrap();
   ```

1. Метод [`is_some()`](https://doc.rust-lang.org/std/option/enum.Option.html#method.is_some) – проверяет, установлено ли значение `Some(T)`.

   ```rust
   if stack.last().is_some() {
     let ch: char = stack.last().unwrap();
   }
   ```

   Внимание, код выше – плохой вариант, так лучше **не писать**.

1. Сравнение с другим значением того же типа:

   ```rust
   let ch: Option<char> = stack.pop();
   if ch == Some('A') {
     ...
   } else { // if ch == None or ch == Some(x) where x != 'A'
   }
   ```

Полную документацию можно прочитать тут: https://doc.rust-lang.org/std/option/enum.Option.html.
Также можно изучать Rust на примерах: https://doc.rust-lang.org/rust-by-example/std/option.html – страница в Rust by Example про Option.
