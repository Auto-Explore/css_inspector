# css/css-scroll-snap/snap-after-style-change-respects-scroll-behavior.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-after-style-change-respects-scroll-behavior.html"
}
```

## style[0]

```css

#scroller {
  width: 300px;
  height: 100px;
  overflow: auto;
  white-space: nowrap;
  scroll-behavior: smooth;
}
.child {
  display: inline-block;
  width: 300px;
  height: 100px;
  scroll-snap-align: start;
}
#child1 {
  background-color: blue;
}
#child2 {
  background-color: green;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
