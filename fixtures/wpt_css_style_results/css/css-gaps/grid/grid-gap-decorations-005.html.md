# css/css-gaps/grid/grid-gap-decorations-005.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-005.html"
}
```

## style[0]

```css


  body {
    margin: 0px;
  }

  .grid-container {
    height: 110px;
    width: 110px;

    display: grid;
    grid-template-columns: repeat(2, 1fr);

    column-gap: 10px;
    row-gap: 10px;

    background-color: green;

    column-rule-color: pink;
    column-rule-style: double;
    column-rule-width: 5px;

    row-rule-color: pink;
    row-rule-style: double;
    row-rule-width: 5px;
  }

  .grid-item {
    background: skyblue;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
