# css/css-grid/alignment/grid-alignment-style-changes-005.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-alignment-style-changes-005.html"
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
  font-family: Ahem;
  line-height: 1;
}
#item1 {
  font-size: 20px;
  background: blue;
}
#item2 {
  font-size: 40px;
  background: green;
}
#item3 {
  font-size: 30px;
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
