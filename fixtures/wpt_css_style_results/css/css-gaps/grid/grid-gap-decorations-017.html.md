# css/css-gaps/grid/grid-gap-decorations-017.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-017.html"
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

    column-rule-color: blue;
    column-rule-style: solid, repeat(auto, groove, double), repeat(2, dotted);
    column-rule-width: 5px;

    row-rule-color: red;
    row-rule-style: repeat(auto, double, solid), dotted, repeat(2, ridge);
    row-rule-width: 5px;
  }
  .item {
    background: gray;
    opacity: 0.5;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “column-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-style”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “row-rule-width”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
