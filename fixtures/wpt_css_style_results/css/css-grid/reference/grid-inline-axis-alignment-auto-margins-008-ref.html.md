# css/css-grid/reference/grid-inline-axis-alignment-auto-margins-008-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/reference/grid-inline-axis-alignment-auto-margins-008-ref.html"
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
      justify-items: center;
      align-items: start;
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
