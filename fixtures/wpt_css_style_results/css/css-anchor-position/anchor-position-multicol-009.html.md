# css/css-anchor-position/anchor-position-multicol-009.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-position-multicol-009.html"
}
```

## style[0]

```css

  .anchor {
    anchor-name: --tjor;
    inline-size: 50px;
    background: green;
  }
  .pos {
    position: absolute;
    position-anchor: --tjor;
    inset-inline-start: anchor(outside);
    inset-block-start: anchor(start);
    inline-size: 50px;
    background: green;
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
