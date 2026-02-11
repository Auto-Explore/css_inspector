# css/css-grid/alignment/grid-block-axis-alignment-auto-margins-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-block-axis-alignment-auto-margins-005.html"
}
```

## style[0]

```css

  #grid {
      display: grid;
      position: relative;
      background: grey;
      grid-template-rows: 40% 60%;
      height: 500px;
      width: 300px;
  }
  #grid div {
    margin: auto 0px auto 0px;
  }
  #item1 {
      font: 20px/1 Ahem;
      color: green;
  }
  #item2 {
      font: 10px/1 Ahem;
      color: blue;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
