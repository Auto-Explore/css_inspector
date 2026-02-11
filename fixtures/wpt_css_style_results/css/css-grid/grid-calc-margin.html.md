# css/css-grid/grid-calc-margin.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-calc-margin.html"
}
```

## style[0]

```css

  .cardStack {
    display: grid;
    width: 100px;
  }
  .card {
    height: 100px;
  }
  #card1 {
    background-color: lightpink;
    margin-top: calc(50% - 0px);
  }
  #card2 {
    background-color: lightgreen;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
