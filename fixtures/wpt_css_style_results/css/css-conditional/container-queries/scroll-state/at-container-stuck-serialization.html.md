# css/css-conditional/container-queries/scroll-state/at-container-stuck-serialization.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/scroll-state/at-container-stuck-serialization.html"
}
```

## style[0]

```css

  @container scroll-state(        stuck:top) { }
  @container scroll-STate(stuck:    ) { }
  @container scroll-STate(stuck) { }
  @container  scroll-state(  ( stuck: BOTTOM) OR ( STUCK: inline-START  ) ) { }
  @container scroll-state (stuck: top) { }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
