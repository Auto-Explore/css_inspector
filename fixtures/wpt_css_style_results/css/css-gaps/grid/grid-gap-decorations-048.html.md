# css/css-gaps/grid/grid-gap-decorations-048.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-048.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }

  .grid-container {
    display: grid;
    grid-gap: 10px;
    grid-template-columns: repeat(6, 100px);
    height: 650px;
    width: 650px;

    column-rule-color: purple, green, white, yellow, repeat(auto, red), blue;
    column-rule-style: solid;
    column-rule-width: 2px, repeat(auto, 5px, 2px), repeat(2, 10px);

    row-rule-color: repeat(auto, yellow);
    row-rule-style: solid;
    row-rule-width: repeat(auto, 10px, 8px), repeat(2, 2px), 5px;
  }

  .item {
    background: gray;
    opacity: 0.5;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
