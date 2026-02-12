# css/css-view-transitions/nested-elements-in-overflow-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/nested-elements-in-overflow-ref.html"
}
```

## style[0]

```css

body {
  background: rebeccapurple;
  margin: 0;
}
.outer {
  width: 50px;
  height: 100px;
  position: absolute;
  top: 50px;
  left: 50px;
  border: 2px solid black;
}
.inner {
  background: lightblue;
  width: 50px;
  height: 50px;
  view-transition-name: outer;
  position: absolute;
  top: 25px;
}
.grey {
  background: lightgrey;
  position: relative;
  width: 50px;
  height: 50px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
