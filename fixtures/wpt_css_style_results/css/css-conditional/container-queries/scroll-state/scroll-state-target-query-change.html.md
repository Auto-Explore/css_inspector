# css/css-conditional/container-queries/scroll-state/scroll-state-target-query-change.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-target-query-change.html"
}
```

## style[0]

```css

  html { scroll-snap-type: block mandatory; }
  body { margin: 0; }
  #filler { height: 10000px; }
  #snapped {
    container-type: scroll-state;
    scroll-snap-align: start;
    width: 100px;
    height: 100px;
  }
  @container scroll-state(snapped: block) {
    .target { --snapped: true; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
