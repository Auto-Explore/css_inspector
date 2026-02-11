# css/css-grid/alignment/grid-alignment-style-changes-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-alignment-style-changes-002.html"
}
```

## style[0]

```css

#container {
  position: relative;
  display: inline-grid;
  grid: 100px / 50px 50px 50px;
  background: grey;
  align-items: baseline;
}
#item1 {
  height: 20px;
  background: blue;
}
#item2 {
  height: 50px;
  background: green;
  align-self: center;
}
#item3 {
  height: 30px;
  background: red;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
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
