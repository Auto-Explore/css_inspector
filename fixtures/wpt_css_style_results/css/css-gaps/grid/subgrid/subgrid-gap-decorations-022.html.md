# css/css-gaps/grid/subgrid/subgrid-gap-decorations-022.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/subgrid/subgrid-gap-decorations-022.html"
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
  }
  .subgrid {
    display: grid;
    grid-template: subgrid / subgrid;
    grid-row: 1/-1;
    grid-column: 1/-1;
    column-rule: 6px solid blue;
    row-rule: 6px solid red;
    rule-inset: 0px;
    column-rule-visibility-items: between;
    row-rule-visibility-items: between;
  }
  .item {
    width: 100%;
    height: 100%;
    background: lightgrey;
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
