# css/filter-effects/crashtests/external-reference-in-interleaved-oof-crash.html

```json
{
  "format_version": 3,
  "file": "css/filter-effects/crashtests/external-reference-in-interleaved-oof-crash.html"
}
```

## style[0]

```css

  #target {
    width: 100px;
    height: 100px;
    background-color: blue;
    --f: url("data:image/svg+xml,");
    position: absolute;
    position-area: block-end span-inline-end;
    position-try-fallbacks: block-start span-inline-end;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
