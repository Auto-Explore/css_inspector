# css/css-gaps/grid/grid-gap-decorations-025.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-025.html"
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
    grid-template-columns: 100px 100px 100px;
    height: 320px;

    column-rule-color: blue;
    column-rule-style: solid;
    column-rule-width: 5px;

    row-rule-color: red;
    row-rule-style: solid;
    row-rule-width: 5px;

  }
  .item {
    background: gray;
    opacity: 0.5;
  }
  .grid-link:visited .grid-container {
    column-rule-color: red, lime, blue;
  }
```

```json
{
  "errors": 4,
  "messages": [
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
    },
    {
      "message": "Invalid value for property “column-rule-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
