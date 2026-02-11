# css/css-anchor-position/container-queries/anchored-fallback-implicit-any.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-fallback-implicit-any.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 100px;
    height: 100px;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: top;
    position-try-fallbacks: right span-bottom;
    width: 100px;
    /* Too tall to fit over the anchor to trigger fallback */
    height: 100px;
    container-type: anchored;
  }
  #target {
    /* Implicit 'any' for the horizontal part of the query */
    @container anchored(fallback: span-bottom) { --span-bottom: yes; }
    /* Implicit 'any' for the vertical part of the query */
    @container anchored(fallback: right) { --right: yes; }
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
