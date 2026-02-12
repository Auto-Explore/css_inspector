# css/css-grid/alignment/grid-alignment-style-changes-003.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-alignment-style-changes-003.html"
}
```

## style[0]

```css

#container {
  position: relative;
  display: inline-grid;
  grid: 50px 50px 50px / 100px;
  background: grey;
  justify-items: baseline;
}
#container > div { writing-mode: vertical-rl; }
#item1 {
  width: 20px;
  background: blue;
}
#item2 {
  width: 50px;
  margin: 0 10px;
  background: green;
}
#item3 {
  width: 30px;
  background: red;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
