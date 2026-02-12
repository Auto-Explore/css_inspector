# css/css-pseudo/marker-content-004-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-004-ref.html"
}
```

## style[0]

```css

body { margin-left: 100px; }
li {
  list-style-type: 'FAIL';
  background: grey;
}
li::marker {
  content: "X";
  display: block;
  width: 100px;
  text-align: start;
  background: lime;
}
span { font-size: 32pt; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
