# css/css-scroll-snap/scroll-margin-focus.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/scroll-margin-focus.html"
}
```

## style[0]

```css

.scroller {
  height: 200px;
  width: 400px;
  border: 1px solid black;
  overflow: auto;
}
.header {
  height: 100px;
  position: sticky;
  top: 0;
  background: rgb(255, 0, 0, 0.3);
}
.header + * {
  scroll-margin-top: 100px;
}
.spacer {
  height: 130px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
