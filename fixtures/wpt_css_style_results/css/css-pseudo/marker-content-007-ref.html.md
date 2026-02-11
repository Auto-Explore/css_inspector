# css/css-pseudo/marker-content-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-content-007-ref.html"
}
```

## style[0]

```css

li {
  list-style-position: inside;
  list-style-type: none;
}
li::before {
  content: "ab";
  display: inline-block;
  width: 50px;
  text-align: right;
  padding-right: 60px;
}
li::after {
  content: "d";
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
