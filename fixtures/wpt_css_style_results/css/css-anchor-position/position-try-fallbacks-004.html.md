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
  "errors": 5,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
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
