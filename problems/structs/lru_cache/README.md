# LRU Cache

В этой задаче Вам предстоит написать реализацию кеша со стратегией [Least Recently Used (LRU)](https://ru.wikipedia.org/wiki/Алгоритмы_кэширования#Least_recently_used_(Вытеснение_давно_неиспользуемых)).

## Задача

Имплементируйте структуру `LRUCache` в файле [src/lib.rs](src/lib.rs):

- `LRUCache::new(capacity: usize) -> Self` - Инициализирует кеш с _положительным_ capacity.
- `LRUCache::len(&self) -> usize` - Возвращает количество элементов в кеше.
- `LRUCache::is_empty(&self) -> bool` - Возвращает true, если кеш пустой, иначе false.
- `LRUCache::get(&mut self, key: &K) -> Option<&V>` - Возвращает `Some(&value)`, где `value` - значение по ключу `key`, если `key` существует в кеше. Иначе возвращает `None`.
- `LRUCache::get_mut(&mut self, key: &K) -> Option<&mut V>` - Делает то же, что и `LRUCache::get`, но возвращает мутабельную ссылку.
- `LRUCache::insert(&mut self, key: K, value: V) -> Option<V>` - Обновляет значение по ключу и возвращает `Some(old_value)`, в котором хранится предыдущее значение по ключу `key`, если оно там было. Если ключа `key` в кеше не было, то возвращается `None`. Если количество элементов превысит capacity кеша во время этой операции, то нужно удалить элементы, неиспользовавшиеся дольше всех (least recently used).
- `LRUCache::clear(&mut self)` - Очищает кеш, удаляя все элементы из него.

В этой задаче вам **не** требуется писать решение за `O(1)` на операции `get` и `insert` (хотя такое решение существует, подумайте на досуге как это сделать).
Но и наивное решение за `O(N)` на операцию не зайдет. Придумайте решение хотя бы за `O(log(N))` на операции `get[_mut]` и `insert`.

Подсказка. Тип ключа `K` является клонируемым ([Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)), хешируемым ([Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html)) и сравниваемым ([Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)). Это значит, что можно использовать такие структуры данных как сбалансированные деревья поиска ([BTreeMap](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html)) и/или хеш-таблицы (только не колхозные, а например [HashMap](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html)).
