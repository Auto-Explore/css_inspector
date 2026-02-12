# css/css-anchor-position/container-queries/anchored-fallback-display-change.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-fallback-display-change.html"
}
```

## style[0]

```css

  body { margin: 0; }
  #anchor {
    anchor-name: --a;
    margin-top: 100px;
    width: 100px;
    height: 100px;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: top;
    position-try-fallbacks: flip-block;
    width: 100px;
    height: 100px;
    container-type: anchored;
  }
  #t1 { height: 100%; }
  @container anchored(fallback: none) {
    #t1 { display: none; }
  }
  @container anchored(fallback: flip-block) {
    #t1 { display: block; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
