# css/css-contain/contain-size-grid-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-size-grid-002.html"
}
```

## style[0]

```css

body {
  overflow: hidden;
}
div {
  position: absolute;
}
#red {
  background: red;
  width: 100px;
  height: 100px;
}
#test {
  background: green;

  contain: size;
  display: grid;
  grid-gap: 20px;
  grid-template-columns: auto 80px; /* 0 + 20 + 80 = 100 */
  grid-template-rows: 40px 40px; /* 40 + 20 + 40 = 100 */
  font-size: 800px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
