# css/css-scroll-snap/unreachable-snap-positions-004.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/unreachable-snap-positions-004.html"
}
```

## style[0]

```css

.content {
  overflow-y: scroll;
  scroll-snap-type: y mandatory;
  white-space: nowrap;
  max-height: 800px;
  border: 2px solid black;
  /* padding-right so you can see the scrollbar: */
  padding-right: 10px;
  writing-mode: vertical-lr;
}
.item {
  height: 500px;
  width: 200px;
  border: 3px solid orange;
  box-sizing: border-box;
  scroll-snap-align: start;
  display: inline-block;
  writing-mode: horitontal-tb;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
