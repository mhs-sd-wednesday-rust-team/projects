# Передвижения

[Существа](./creatures.md) должны уметь перемещаться. Этот документ описывает механику движений.
## Сценарии

### Игрок может управлять героем

### Игрок видит перемещения других существ

### Игрок двигается в сторону стены или другого массивного объекта и упирается в него

## Требования

- Игрок может управлять своим персонажем с клавиатуры
- Должны быть стены

## Дизайн

Управление персонажем реализовано с помощью отдельной системы `PlayerInput`, которая собирает нажатия из ресурса `Platform` (подробнее см в [главе про отрисовку](./rendering.md)) и обновляет глобальное состояние игры, а также порождает пользовательские действия в виде компонент.

Выделим следующие компонент движения `MoveCommand` - означает, что пользователь выбрал действие движения и содержит направление. Также `MoveCommand` может принадлежать другим существам.

Для отображения непроходимых объектов добавим компонент `Solid`. Вместе с компонентами `Position` и `Renderable` получаем знание о габаритах объекта.

Система `MoveSystem` отвечает за обработку `MoveCommand`, перемещая все `Position` в соответствии с командой и удаляя соответствующую команду из игрового мира. Если на пути встречается `Solid` объект, движения не происходит.