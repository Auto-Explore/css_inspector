# css/css-anchor-position/last-successful-pseudo-element-fallbacks.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/last-successful-pseudo-element-fallbacks.html"
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
  #anchored::before {
    position-anchor: --a;
    bottom: anchor(top);
    left: anchor(left);
    position-try-fallbacks: flip-block;
    position: absolute;
    width: 100px;
    height: 200px;
    background: lime;
    content: "";
  }
  /*
    Hack to change position-try-fallbacks of the pseudo-element,
    since there's no easy way to change it in JavaScript.
  */
  #anchored.new-fallback::before {
    position-try-fallbacks: flip-block, --foo;
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
      "message": "Unknown property “position-try-fallbacks”.",
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
