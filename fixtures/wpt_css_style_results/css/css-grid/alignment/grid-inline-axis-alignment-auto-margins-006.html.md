# css/css-grid/alignment/grid-inline-axis-alignment-auto-margins-006.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-inline-axis-alignment-auto-margins-006.html"
}
```

## style[0]

```css

  #grid {
      display: grid;
      position: relative;
      background: grey;
      font: 10px/1 Ahem;
      grid-template-columns: 40% 60%;
      grid-template-rows: 100px;
      height: auto;
      width: 500px;
      align-items: start;
  }
  #grid div {
    margin: 0px auto 0px auto;
  }
  #item1 {
      color: green;
  }
  #item2 {
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
