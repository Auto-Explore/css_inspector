# css/css-position/position-absolute-chrome-bug-002.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-chrome-bug-002.html"
}
```

## style[0]

```css


#container {
  position: relative;
  background: gray;
}
#container::after {
  content: '';
  display: table;
  clear:both;
}
#target {
  position: absolute;
  right: 0;
  background: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
