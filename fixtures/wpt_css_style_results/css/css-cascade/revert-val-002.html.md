# css/css-cascade/revert-val-002.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/revert-val-002.html"
}
```

## style[0]

```css

#outer {
  background-color: red;
  width: 100px;
  height: 100px;
  overflow: hidden;
}
#inner {
  /* This should win over `revert` */
  display: block !important;
}
#inner {
  color: green;
  background-color: green;
  display: revert;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
