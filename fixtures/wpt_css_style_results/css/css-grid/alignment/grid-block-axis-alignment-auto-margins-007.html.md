# css/css-grid/alignment/grid-block-axis-alignment-auto-margins-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-block-axis-alignment-auto-margins-007.html"
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
      width: 200px;
  }
  #grid div {
    margin: auto 0px auto 0px;
  }
  #item1 {
      background: green;
      width: 25px;
      height: 50px;
  }
  #item2 {
      background: blue;
      width: 25px;
      height: 100px;
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
