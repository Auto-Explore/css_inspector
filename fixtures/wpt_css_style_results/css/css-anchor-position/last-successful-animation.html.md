# css/css-anchor-position/last-successful-animation.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/last-successful-animation.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 400px;
    height: 400px;
    background: teal;
  }

  #anchor {
    position: relative;
    top: 100px;
    left: 100px;
    width: 100px;
    height: 100px;
    background: red;
    anchor-name: --a;
  }

  /* Dummy animation keyframe */
  @keyframes anime {
    from { background: #000000; }
    to { background: #ffffff; }
  }

  #anchored {
    position-anchor: --a;
    position-try-fallbacks: flip-block;
    position: absolute;
    width: 100px;
    height: 200px;
    position-area: top center;
    background: lime;
    animation: anime 1s linear;
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
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
