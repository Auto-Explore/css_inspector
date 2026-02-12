# css/css-anchor-position/position-try-fallbacks-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-fallbacks-004.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
  }

  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: left; /* No room on the left. */
    position-try-fallbacks: --fits, --bogus;
  }

  @position-try --fits {
    position-area: right;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
