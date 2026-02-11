# css/css-anchor-position/position-try-fallbacks-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-try-fallbacks-003.html"
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
  #anchored1 {
    position: absolute;
    position-anchor: --a;
    position-area: top left;
    position-try-fallbacks: flip-block;
    position-visibility: always;
    width: 100px;
    height: 100px;
    background: hotpink;
  }
  #anchored2 {
    position: absolute;
    position-anchor: --a;
    position-area: top right;
    position-try-fallbacks: flip-block;
    position-visibility: always;
    width: 100px;
    height: 300px;
    background: yellow;
  }
```

```json
{
  "errors": 10,
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
      "message": "Unknown property “position-visibility”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
      "message": "Unknown property “position-visibility”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
