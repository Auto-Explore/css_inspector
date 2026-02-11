# css/css-anchor-position/container-queries/anchored-fallback-color-change.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-fallback-color-change.html"
}
```

## style[0]

```css

  #wrapper {
    position: relative;
    height: 200px;
  }
  #anchor {
    position: relative;
    anchor-name: --a;
    width: 100px;
    height: 100px;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: top;
    position-try-fallbacks: flip-block;
    width: 100px;
    height: 100px;
    container-type: anchored;
    background-color: red;
  }
  #target {
    position: absolute;
    width: 100px;
    height: 100px;
    left: 100px;
  }
  @container anchored(fallback: none) {
    #target {
      background-color: green;
      left: auto;
    }
  }
  @container anchored(fallback: flip-block) {
    #target {
      background-color: red;
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
