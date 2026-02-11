# css/fill-stroke/reference/paint-order-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/fill-stroke/reference/paint-order-001-ref.html"
}
```

## style[0]

```css

body>div { position: relative; clear: left; float: left; }
span.f { -webkit-text-stroke: 5px orange; }
span.s,
span.sf { display: inline-block; position: relative; }
span.s::before,
span.sf::before { -webkit-text-stroke: 5px orange; position: absolute; z-index: -1; }
span.s::before { content: "stroke"; }
span.sf::before { content: "s-f"; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “-webkit-text-stroke”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
