# css/fill-stroke/paint-order-001.html

```json
{
  "format_version": 3,
  "file": "css/fill-stroke/paint-order-001.html"
}
```

## style[0]

```css

span { -webkit-text-stroke: 5px orange; }
span.i { paint-order: initial; }
span.f { paint-order: fill; }
span.fs { paint-order: fill stroke; }
span.s { paint-order: stroke; }
span.sf { paint-order: stroke fill; }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “-webkit-text-stroke”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “paint-order”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “paint-order”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “paint-order”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “paint-order”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “paint-order”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
