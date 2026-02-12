# css/css-gaps/grid/grid-gap-decorations-004.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-004.html"
}
```

## style[0]

```css


  body {
    margin: 0px;
  }

  .grid-container {
    height: 108px;
    width: 108px;

    display: grid;
    grid-template-columns: repeat(2, 1fr);

    column-gap: 12px;
    row-gap: 12px;

    background-color: green;

    column-rule-color: pink;
    column-rule-style: double;
    column-rule-width: 12px;

    row-rule-color: pink;
    row-rule-style: double;
    row-rule-width: 12px;
  }

  .grid-item {
    background: skyblue;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
