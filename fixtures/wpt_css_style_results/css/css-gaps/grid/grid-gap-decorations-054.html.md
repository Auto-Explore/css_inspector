# css/css-gaps/grid/grid-gap-decorations-054.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-054.html"
}
```

## style[0]

```css

  body {
    margin: 0px;
  }
  .grid {
    display: grid;
    grid-template: repeat(3, 100px) / repeat(3, 100px);
    gap: 20px;
    row-rule: 6px solid red;
    column-rule: 6px solid blue;
    rule-inset: 0px;

    column-rule-visibility-items: around;
    row-rule-visibility-items: around;

    column-rule-break: intersection;
    row-rule-break: intersection;
  }

  .item {
    width: 100%;
    height: 100%;
    background: lightgray;
    opacity: 0.8;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
