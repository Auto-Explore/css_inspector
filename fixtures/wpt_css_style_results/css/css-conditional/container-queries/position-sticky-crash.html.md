# css/css-conditional/container-queries/position-sticky-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/position-sticky-crash.html"
}
```

## style[0]

```css

.container {
  min-width: 30px;
  container-type: inline-size;
}

.scroller {
  overflow: scroll;
  width: 100px;
  height: 100px;
}

@container (width = 100px) {
  .scroller { display: none; }
}

.float {
  background: cyan;
  float: left;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
