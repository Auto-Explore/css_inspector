# css/css-scroll-snap/unreachable-snap-positions-002.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/unreachable-snap-positions-002.html"
}
```

## style[0]

```css

.scroller {
  width: 100vw;
  height: 100px;
  display: flex;
  scroll-snap-type: x mandatory;
  overflow-x: auto;
}
.scroller.rtl {
  direction: rtl;
}
.scroller.end > span {
  scroll-snap-align: end;
}
.scroller.center > span {
  scroll-snap-align: end;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
