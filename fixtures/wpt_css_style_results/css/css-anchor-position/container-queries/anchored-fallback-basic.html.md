# css/css-anchor-position/container-queries/anchored-fallback-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-fallback-basic.html"
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
  .anchored {
    position: absolute;
    position-anchor: --a;
    position-area: top;
    position-try-fallbacks: flip-block;
    width: 100px;
    height: 100px;
    container-type: anchored;
  }
  .anchored + .anchored {
    /* Too tall to fit over the anchor to trigger fallback */
    height: 200px;
  }
  @container anchored(fallback: none) {
    div { --fallback: no; }
  }
  @container anchored(fallback: flip-block) {
    div { --fallback: yes; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
