# css/css-anchor-position/at-position-try-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/at-position-try-invalidation.html"
}
```

## style[0]

```css

  body { margin: 0; }

  #anchor {
    anchor-name: --a;
    margin-left: 100px;
    width: 100px;
    height: 100px;
  }

  #anchored {
    position: absolute;
    width: 100px;
    height: 100px;
    position-try-fallbacks: --pf;
    left: 999999px; /* force fallback */
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
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

## style[1]

```css

  @position-try --pf {
    left: anchor(--a left);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
