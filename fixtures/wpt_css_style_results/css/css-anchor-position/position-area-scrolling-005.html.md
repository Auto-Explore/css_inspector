# css/css-anchor-position/position-area-scrolling-005.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-scrolling-005.html"
}
```

## style[0]

```css

  #anchored {
    position: absolute;
    box-sizing: border-box;
    border: solid;
    position-anchor: --anchor;
    position-try-fallbacks: flip-block, flip-inline, flip-block flip-inline;
    position-visibility: always;
    width: 50%;
    height: 50%;
    background: cyan;
  }
  #container.thicker > .pos {
    border-width: thick;
  }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-visibility”.",
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
