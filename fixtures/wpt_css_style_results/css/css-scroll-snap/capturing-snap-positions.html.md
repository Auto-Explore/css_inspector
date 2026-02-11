# css/css-scroll-snap/capturing-snap-positions.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/capturing-snap-positions.html"
}
```

## style[0]

```css


.scroller {
  width: 100px;
  height: 100px;
  overflow: auto;
  scroll-snap-type: block mandatory;
}
.item {
  background: gray;
  height: 100px;
  scroll-snap-type: block mandatory;
}
.item.snapped {
  background: green;
  scroll-snap-align: center;
}
.subitem {
  background: red;
  height: 100px;
  scroll-snap-align: center;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
