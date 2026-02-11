# css/css-anchor-position/container-queries/anchored-fallback-scroll.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-fallback-scroll.html"
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
    position: fixed;
    position-anchor: --a;
    position-area: top;
    position-try-fallbacks: flip-block;
    width: 100px;
    height: 100px;
    container-type: anchored;
  }
  @container anchored(fallback: none) {
    #t1 { --fallback: no; }
  }
  @container anchored(fallback: flip-block) {
    #t1 { --fallback: yes; }
  }
```

```json
{
  "errors": 4,
  "messages": [
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
