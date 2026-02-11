# css/css-conditional/container-queries/scroll-state/at-container-scrolled-serialization.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/at-container-scrolled-serialization.html"
}
```

## style[0]

```css

  @container scroll-state(   scrolled:top   ) { }
  @container scroll-STate(scrolled:    ) { }
  @container scroll-STate(scrolled) { }
  @container  scroll-state(  ( scrolled: LEFT) OR ( scrolled: BOTTOM  ) ) { }
  @container scroll-state (scrolled: right) { }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
