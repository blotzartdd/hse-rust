# Rust HSE 2024

## Ссылки

### Основное

- Сайт курса: [https://rust-hse-2024.azazkamaz.me](https://rust-hse-2024.azazkamaz.me)
- Таблица с результатами сдачи задач: https://docs.google.com/spreadsheets/d/1R02622hEAEfSdK18Ng6BFx6Nxham8qMohZH8whE2Gls/edit#gid=0

### Коммуникация

- Телеграм канал: <https://t.me/rust24_25>
- Телеграм чат: <https://t.me/+9HOKbezDmmw5YzQy>.

### Материалы

- Видеозаписи лекций и семинаров: TODO
- Материалы семинаров: [seminars](./seminars)
- [Дополнительные материалы для изучения](docs/reading-list.md)

## Настройка окружения

Вся работа с курсом организована на GitHub, у Вас будет собственный репозиторий для решения задач, а также простенький UI [https://rust-hse-2024.azazkamaz.me](https://rust-hse-2024.azazkamaz.me) для просмотра задач, дедлайнов и т.п.

Для начала работы с курсом проследуйте инструкциям ниже. При любых трудностях обращайтесь в телеграм чат курса.

### Регистрация в системе [https://rust-hse-2024.azazkamaz.me](https://rust-hse-2024.azazkamaz.me)

1. Зарегистрируйтесь в [тестирующей системе](https://rust-hse-2024.azazkamaz.me). Для входа Вам потребуется аккаунт на [GitHub](github.com). Секретный код: `krusty-krab-2`. 

1. После регистрации автоматика добавит Вас в GitHub организацию [rust-hse](https://github.com/rust-hse) и создаст там для Вас копию репозитория с задачами [rust-hse/rust-hse-2024](https://github.com/rust-hse/rust-hse-2024).

### Настройка Git

Чтобы работать с git-репозиториями, вам нужно установить и настроить git на своем компьютере, а также аутентифицироваться в GitHub.
Пошаговая инструкция ниже.
Либо Вы можете проследовать инструкции от GitHub: [Set up Git](https://docs.github.com/en/get-started/quickstart/set-up-git).

Если Вы регулярно работаете с git и GitHub-ом, то скорее всего у Вас все настроено, Вы можете пропустить эти шаги и перейти к шагу клонирования репозитория.

1. Установите git: https://git-scm.com/downloads

1. Задайте имя и email (желательно тот же, что вы использовали при регистрации на GitHub):

   ```sh
   git config --global user.name "Ivan Ivanov"
   git config --global user.email ivan1337@yandex.ru
   ```

1. Аутентифицируйтесь в GitHub. Тут есть несколько вариантов как это сделать. Опишем два из них: через [GitHub CLI](https://docs.github.com/en/get-started/getting-started-with-git/caching-your-github-credentials-in-git#github-cli) (рекомендуемый GitHub-ом) и через [SSH](https://docs.github.com/en/authentication/connecting-to-github-with-ssh).

   - GitHub CLI:

     1. Установите GitHub CLI: https://github.com/cli/cli#installation.

     1. В терминале запустите команду `gh auth login` и следуйте инструкциям.
        Отвечайте на вопросы как на этом скриншоте:

        ![gh auth login result](https://rust-hse.ru/static/gh_auth_login.png)

   - SSH:

     1. Сгенерируйте ssh ключ, если у Вас его еще нет (см. `~/.ssh`).

        ```
        ssh-keygen -t ed25519 -C "<your email>"
        ```

     1. Скопируйте сгенерированный публичный ключ.

        ```
        cat ~/.ssh/id_ed25519.pub
        ```

     1. Перейдите в настройки аккаунта, в раздел SSH ключей https://github.com/settings/keys.

     1. Нажмите `Add SSH key`.

     1. Заполните Title (чем угодно), в поле Key вставьте скопированный публичный ключ. Нажмите `Add SSH key`.

     1. Проверьте, что ssh ключ работает. Выполните команду `ssh -T git@github.com`.
        Вы должны увидеть такое приветствие:

        ```
        $ ssh -T git@github.com
        Hi <your login>! You've successfully authenticated, but GitHub does not provide shell access.
        ```

### Клонирование репозитория на компьютер

В [нашей GitHub огранизации](https://github.com/orgs/rust-hse/repositories) Вам будут доступны два репозитория: публичный `rust-hse-2024` и Ваш личный `rust-hse-2024-<your login>`.
Для сдачи задач Вы будете работать со своим личным репозиторием.

1. Передите на страницу своего репозитория (если вы не читаете эту инструкцию уже с него).
   Его можно найти на [https://rust-hse-2024.azazkamaz.me](https://rust-hse-2024.azazkamaz.me), перейдя по ссылке `MY REPO`, или на странице репозиториев организации: https://github.com/orgs/rust-hse/repositories.

1. Склонируйте cвой репозиторий с задачами.
   Для этого найдите адрес репозитория:

   ![github repo clone](https://rust-hse-2024.azazkamaz.me/static/gh_clone_url.png)

   Какой выбрать адрес, зависит от того, как вы настроили git на предыдущем шаге:

   - Если вы использовали GitHub CLI, то попробуйте выполнить

     ```
     $ git clone https://github.com/rust-hse/rust-hse-2024-<your login>.git
     ```

     или

     ```
     $ gh repo clone rust-hse/rust-hse-2024-<your login>
     ```

   - Если вы добавляли SSH ключ, то может сработать git-адрес:

     ```
     $ git clone git@github.com:rust-hse/rust-hse-2024-<your login>.git
     ```

1. Каждую неделю после занятий вам надо будет обновлять репозиторий, чтобы у вас появились условия
   новых задач:

   ```sh
   $ git pull --rebase
   ```

### Настройка IDE

Официально поддерживаемой средой разработки является VS Code, однако Вы вольны использовать любые редакторы/IDE, которые вам нравятся.

1. Установите Rust, следуя [официальному руководству](https://www.rust-lang.org/tools/install).
1. Установите форматтер, линтер и утилиту бенчмаркинга:

    ```
    rustup component add rustfmt
    rustup component add clippy
    cargo install cargo-criterion
    ```

1. Установите [VS Code](https://code.visualstudio.com).
1. Установите расширения для VS Code:

   * [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
   * [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)

1. В VS Code нажмите "File" -> "Open Folder", откройте директорию, куда вы склонировали репозиторий курса.

### Отправка решения

Чтобы проверить работоспособность окружения, решите первую тестовую задачу:

1. Откройте `tutorial/add/src/lib.rs`. Убедитесь, что у вас работают базовые вещи: подсветка ошибок компиляции, autocomplete, go to definition.
1. Откройте `tutorial/add/tests/tests.rs`. Нажмите `Debug` над `fn it_works()`, убедитесь, что тест падает и вы оказываетесь в дебагере в момент его падения.
1. Напишите правильную реализацию функции `add` в `tutorial/add/src/lib.rs`.
1. Установите утилиту `rover` для тестирования и отправки решений на сервер командой `cargo install --path tools/rover` (запускайте из корня репозитория).
1. Запустите локальные тесты командой `rover test`, находясь в директории `tutorial/add`. Убедитесь, что они проходят.
1. Отправьте своё решение на сервер командой `rover submit`. Ваш сабмит должен появиться по ссылке "SUBMITS" на [https://rust-hse-2024.azazkamaz.me](https://rust-hse-2024.azazkamaz.me).
После успешного прохождения тестов вам должно начислиться 0 баллов в
[таблице с баллами](https://docs.google.com/spreadsheets/d/1KPk-WxWK4K_9AQZusKAgXgZw1Inz3zVgLnWft8AVIps/edit#gid=0).

Если на каком-то этапе у вас возникли проблемы - пишите в чат курса.
