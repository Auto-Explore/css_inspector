# css/css-anchor-position/anchor-position-non-anchored-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-non-anchored-fallback.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --anchor;
    background: green;
    width: 100px;
    height: 100vh;
  }
  #anchored {
    background: green;
    top: anchor(--anchor bottom);
    width: 100px;
    height: 50px;
    position: absolute;
    position-try: --bottom;
  }

  @position-try --bottom {
    top: auto;
    bottom: 0px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
