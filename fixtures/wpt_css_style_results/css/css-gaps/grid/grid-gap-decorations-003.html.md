# css/css-gaps/grid/grid-gap-decorations-003.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-003.html"
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

    column-rule-color: pink;
    column-rule-style: dotted;
    column-rule-width: 10px;

    row-rule-color: green;
    row-rule-style: dotted;
    row-rule-width: 10px;
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
