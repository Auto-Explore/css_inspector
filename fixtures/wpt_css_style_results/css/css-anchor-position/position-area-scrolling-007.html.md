# css/css-anchor-position/position-area-scrolling-007.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-scrolling-007.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    margin-top: 200px;
    margin-left: 100px;
    width: 150px;
    height: 50px;
    background: blue;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: top left;
    position-try-fallbacks: flip-inline, flip-block, flip-block flip-inline;
    width: 100%;
    height: 100%;
    min-width: 60px;
    min-height: 60px;
    background: hotpink;
  }
```

```json
{
  "errors": 5,
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
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
