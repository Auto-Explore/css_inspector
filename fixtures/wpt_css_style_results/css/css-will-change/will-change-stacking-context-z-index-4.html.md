# css/css-will-change/will-change-stacking-context-z-index-4.html

```json
{
  "format_version": 3,
  "file": "css/css-will-change/will-change-stacking-context-z-index-4.html"
}
```

## style[0]

```css

.test {
  will-change: z-index;
  width: 100px;
  background: green;
}
.test::before {
  content: "";
  display: block;
  position: relative;
  z-index: -1;
  height: 100px;
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
