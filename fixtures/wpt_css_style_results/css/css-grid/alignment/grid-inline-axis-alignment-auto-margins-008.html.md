# css/css-grid/alignment/grid-inline-axis-alignment-auto-margins-008.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-inline-axis-alignment-auto-margins-008.html"
}
```

## style[0]

```css

  #grid {
      display: grid;
      position: relative;
      background: grey;
      grid-template-columns: 40% 60%;
      grid-template-rows: 100px;
      height: 200px;
      width: auto;
      align-items: start;
  }
  #grid div {
    margin: 0px auto 0px auto;
  }
  #item1 {
      font: 20px/1 Ahem;
      color: green;
  }
  #item2 {
      font: 40px/1 Ahem;
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
