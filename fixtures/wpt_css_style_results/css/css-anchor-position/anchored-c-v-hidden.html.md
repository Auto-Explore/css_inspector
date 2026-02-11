# css/css-anchor-position/anchored-c-v-hidden.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchored-c-v-hidden.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 100px;
    height: 100px;
  }
  #anchored {
    position: absolute;
    position-anchor: --a;
    position-area: center;
    content-visibility: hidden;
    width: 100px;
    height: 100px;
    background-color: green;
  }
  #fail { background-color: red; }
```

```json
{
  "errors": 3,
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
    }
  ],
  "warnings": 0
}
```
