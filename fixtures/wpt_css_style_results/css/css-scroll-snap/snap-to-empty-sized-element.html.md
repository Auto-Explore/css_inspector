# css/css-scroll-snap/snap-to-empty-sized-element.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-to-empty-sized-element.html"
}
```

## style[0]

```css

#scroller {
  height: 500px;
  width: 500px;
  overflow: scroll;
  scroll-snap-type: y proximity;
}

#scroller::after {
  display: block;
  content: "";
  scroll-snap-align: end;
}

li {
  height: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
