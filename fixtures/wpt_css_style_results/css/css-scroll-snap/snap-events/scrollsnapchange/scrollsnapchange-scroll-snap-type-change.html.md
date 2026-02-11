# css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-scroll-snap-type-change.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-snap/snap-events/scrollsnapchange/scrollsnapchange-scroll-snap-type-change.html"
}
```

## style[0]

```css

  html {
    scroll-snap-type: y mandatory;
    overflow-y: scroll;
  }
  #snapped {
    scroll-snap-align: start;
  }
  #filler {
    height: 10000px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “scroll-snap-type”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
