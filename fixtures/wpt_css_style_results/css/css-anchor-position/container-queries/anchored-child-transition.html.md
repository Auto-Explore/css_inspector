# css/css-anchor-position/container-queries/anchored-child-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-child-transition.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
  }
  #container {
    container-type: anchored;
    position: absolute;
    position-anchor: --a;
    position-area: top;
    position-try-fallbacks: flip-block;
  }
  #child {
    scale: 1;
  }
  #child.large {
    scale: 2;
  }
  @container anchored(fallback: flip-block) {
    #child {
      transition: scale 1000s steps(2, start);
    }
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
