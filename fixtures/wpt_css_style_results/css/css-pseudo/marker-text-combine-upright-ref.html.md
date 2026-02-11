# css/css-pseudo/marker-text-combine-upright-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/marker-text-combine-upright-ref.html"
}
```

## style[0]

```css

body {
  writing-mode: vertical-lr;
}
ol, ul {
  display: flow-root;
  list-style-type: none;
}
ul {
  list-style-position: inside;
}
ol span {
  display: inline-block;
  margin-top: -50px;
  height: 50px;
  text-align: end;
}
span {
  white-space: pre;
  text-combine-upright: all;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
