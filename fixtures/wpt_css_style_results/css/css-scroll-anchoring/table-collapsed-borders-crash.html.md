# css/css-scroll-anchoring/table-collapsed-borders-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/table-collapsed-borders-crash.html"
}
```

## style[0]

```css

body {
  height:200vh;
}
table {
  height: 200px;
  width: 200px;
  background-color: lime;
  border-collapse: collapse; /* triggers problematic border calculation */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
