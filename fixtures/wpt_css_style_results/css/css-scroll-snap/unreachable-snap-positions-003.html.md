# css/css-scroll-snap/unreachable-snap-positions-003.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/unreachable-snap-positions-003.html"
}
```

## style[0]

```css

.content {
  overflow-x: scroll;
  scroll-snap-type: x mandatory;
  white-space: nowrap;
  width: 400px;
  border: 2px solid black;
  /* padding-bottom so you can see the scrollbar: */
  padding-bottom: 10px;
}
.item {
  width: 250px;
  height: 200px;
  border: 3px solid orange;
  box-sizing: border-box;
  scroll-snap-align: start;
  display: inline-block;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
