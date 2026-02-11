# css/css-anchor-position/chrome-377324991-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/chrome-377324991-crash.html"
}
```

## style[0]

```css

  @position-try --opt {
    position-area: right center;
  }
  #anchor {
    width: 100px;
    height: 100px;
    background-color: green;
    anchor-name: --a;
  }
  #anchored {
    position: absolute;
    width: 100px;
    height: 100px;
    background-color: lime;
    position-anchor: --a;
    position-area: left center;
    position-try-fallbacks: --opt;
  }
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
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
