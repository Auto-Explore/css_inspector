# css/css-transitions/after-change-style-inherited-try-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/after-change-style-inherited-try-fallback.html"
}
```

## style[0]

```css

  body { margin: 0; }
  #outer, #inner {
    transition: 2s steps(2, start);
    transition-property: color, width;
  }
  #outer {
    anchor-name: --anchor;
    width: 100px;
    height: 100px;
    background-color: orange;
  }
  #inner.wide {
    width: 300px !important;
  }
  #inner {
    width: 100px;
    height: 100px;
    position: absolute;
    position-anchor: --anchor;
    position-area: left center;
    position-try-fallbacks: --fallback;
    background-color: teal;
  }
  @position-try --fallback {
    position-area: right center;
    width: 200px;
  }

  #outer { color: red; }
  #inner { color: black; }
  #outer.green { color: lime; }
  #outer.green #inner { color: unset; }
```

```json
{
  "errors": 6,
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
      "message": "Invalid value for property “background-color”.",
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
