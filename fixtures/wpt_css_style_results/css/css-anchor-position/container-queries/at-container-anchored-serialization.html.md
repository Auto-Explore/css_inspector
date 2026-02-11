# css/css-anchor-position/container-queries/at-container-anchored-serialization.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/at-container-anchored-serialization.html"
}
```

## style[0]

```css

  @container anchored(        fallback:--foo) { }
  @container ANChoreD(fallback:    ) { }
  @container anchoRed(fallback) { }
  @container  anchored(  ( fallback: flip-INLINE) OR ( FALLBACK : --bar  ) ) { }
  @container anchored (fallback: top right) { }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
