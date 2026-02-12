# css/css-shapes/shape-outside-infinite-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-shapes/shape-outside-infinite-crash.html"
}
```

## style[0]

```css

.test {
  width: 200px; overflow: hidden; line-height: 0;
}
.float {
  float: left;
  width: 100px;
  height: 100px;
  background: orange;
  shape-outside: polygon(0 0, 100% 100%, 0 100%);
  clip-path: polygon(0 0, 100% 100%, 0 100%);
}
span {
  display: inline-block;
  width: 90px;
  background: rebeccapurple;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
