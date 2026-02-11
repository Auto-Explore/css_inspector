# css/css-anchor-position/remove-anchor-dirty-layout.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/remove-anchor-dirty-layout.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
  }
  #target {
    position: absolute;
    top: anchor(top);
    position-anchor: --a;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
