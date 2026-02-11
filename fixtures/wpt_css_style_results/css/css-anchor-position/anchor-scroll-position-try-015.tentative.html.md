# css/css-anchor-position/anchor-scroll-position-try-015.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-position-try-015.tentative.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    margin-top: 200px;
    width: 100px;
    height: 100px;
    background: blue;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: top center;
    position-try-fallbacks: flip-block;
    width: 100%;
    height: 100%;
    min-height: 100px;
    background: green;
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
