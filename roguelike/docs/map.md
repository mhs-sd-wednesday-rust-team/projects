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

Базово игровой мир состоит из карты, описанной отдельным ресурсом `WorldTileMap` в ECS, и объектов, расположенных на ней. `WorldTileMap` описывает матрицу, содержащую игровые тайлы `Tile`. Также он содержит описание биома, описанного enum-ом.

Пока выделяем два типа тайла: земля (по ней можно ходить) и стена (по ней ходить нельзя). Позже число тайлов будет расширено.

Загрузка/сохранение/обновление карты будет происходить вне ECS внутри game-loop.

Отрисовка происходит в отдельном виджете `BoardView`, создаваемом `RenderSystem`.

## Альтернативы

TBD

