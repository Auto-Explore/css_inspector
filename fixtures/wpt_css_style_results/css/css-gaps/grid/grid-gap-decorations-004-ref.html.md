# css/css-gaps/grid/grid-gap-decorations-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-gaps/grid/grid-gap-decorations-004-ref.html"
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

  }

  .grid-item {
    background: skyblue;
  }

  .col-rule {
    margin: 0px;
    padding: 0px;
    width: 0px;


    height: 108px;
    border-right: 12px double;
    border-color: pink;
    position: absolute;
    left: 48px;
    top: 0px;

  }

  .row-rule {
    margin: 0px;
    padding: 0px;
    height: 0px;

    width: 108px;
    border-bottom: 12px double;
    border-color: pink;
    position: absolute;
    top: 48px;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
