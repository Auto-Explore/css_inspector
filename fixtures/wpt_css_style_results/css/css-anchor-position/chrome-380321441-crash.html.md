# css/css-anchor-position/chrome-380321441-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/chrome-380321441-crash.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: start;
    position-try-fallbacks: flip-block;
    width: 2px;
    height: 100px;
    background: teal;
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
