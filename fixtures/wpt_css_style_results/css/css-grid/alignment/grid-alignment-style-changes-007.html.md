# css/css-grid/alignment/grid-alignment-style-changes-007.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-alignment-style-changes-007.html"
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
  font-family: Ahem;
  text-orientation: sideways;
  line-height: 1;
}
#container > div { writing-mode: vertical-lr; }
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
