# css/css-anchor-position/position-try-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-dynamic.html"
}
```

## style[0]

```css

  body { margin: 0; }

  @position-try --fallback1 {
    left: anchor(--a1 right);
  }
  #anchor {
    anchor-name: --a1;
    width: 100px;
    height: 100px;
  }
  #anchored {
    position: absolute;
    left: 999999px; /* Force fallback */
    width: 100px;
    height: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
