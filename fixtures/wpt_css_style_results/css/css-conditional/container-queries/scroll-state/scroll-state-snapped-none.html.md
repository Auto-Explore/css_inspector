# css/css-conditional/container-queries/scroll-state/scroll-state-snapped-none.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/scroll-state-snapped-none.html"
}
```

## style[0]

```css

  #no-snap { container-type: scroll-state; }
  @container scroll-state(snapped: none) {
    #target { --snapped-none: true }
  }
  @container not scroll-state(snapped) {
    #target { --snapped-boolean: true }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
