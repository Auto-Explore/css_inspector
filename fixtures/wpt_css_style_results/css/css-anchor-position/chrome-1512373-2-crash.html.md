# css/css-anchor-position/chrome-1512373-2-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/chrome-1512373-2-crash.html"
}
```

## style[0]

```css

  @position-try --foo { left: 0; }
  #t {
    position: absolute;
    left: 999999px; /* force fallback */
    position-try-fallbacks: --foo;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
