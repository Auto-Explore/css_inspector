# css/css-anchor-position/inherit-height-from-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/inherit-height-from-fallback.html"
}
```

## style[0]

```css

  #container {
    position: relative;
  }
  #anchor {
    anchor-name: --a1;
    width: 0px;
    height: 100px;
  }
  #anchored {
    position-area: left center;
    position: absolute;
    position-anchor: --a1;
    position-try-fallbacks: --f1;
    width: 100px;
  }
  #child {
    height: inherit;
    background: green;
  }
  @position-try --f1 {
    position-area: right center;
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
