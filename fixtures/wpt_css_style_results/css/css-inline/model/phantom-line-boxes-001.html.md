# css/css-inline/model/phantom-line-boxes-001.html

```json
{
  "format_version": 3,
  "file": "css/css-inline/model/phantom-line-boxes-001.html"
}
```

## style[0]

```css

.wrapper {
  float: left;
  width: 50px;
  background: red;
}
.wrapper.phantom {
  background: green;
}
.wrapper > div {
  line-height: 0;
  background: green;
}
.wrapper.phantom > div {
  background: red
}
.wrapper > div::after {
  content: "";
  display: flow-root;
  margin-top: 200px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
