# Игровой мир

В этом документе описан формат представления игровой карты и
пути работы с ней.

## Мотивация

Представление игрового мира в виде карты, по которой можно перемещаться, неотъемлемое требование Roguelike игр.
Карта может содержать элементы, такие как ландшафт, объекты, персонажи и другие элементы мира, предоставляя визуальные и интерактивные ориентиры.
Возможность сохранения и загрузки состояния мира позволяет игрокам достичь прогресса, который они могут продолжить в дальнейшем,
что увеличивает вовлеченность и удовлетворение от игры.

## Сценарии

### Игрок видит карту на экране

### Игрок может сохранить текущее состояние игрового мира и вернуться к нему после перезапуска игры

## Требования

- сериализуемость состояния игрового мира
- компактность сериализованного представления
- нет дополнительных ассетов (картинки, звуки, тд. должны быть встроены в бинарь игры)

## Дизайн решения

Чтобы обеспечить сериализуемость карты, опишем её в ECS, чтобы иметь воспользоваться модулем `saveload`, описанном в [соответствующем разделе](./component-system.md).

Базово карта состоит из фона, описанного одним большим тайлом с помощью компоненты `Renderable`. Объекты описываются с помощью отдельных entities с компонентами `Position` и `Renderable`.

Также опишем компонент `Biome`, содержащий в себе описание биома.

Загрузка/сохранение/обновление карты будет происходить вне ECS внутри game-loop.



## Альтернативы

TBD
