# css/css-images/object-fit-containcontainintrinsicsize-png-001e.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-images/object-fit-containcontainintrinsicsize-png-001e.tentative.html"
}
```

## style[0]

```css

      embed {
        border: 1px dashed gray;
        padding: 1px;
        image-rendering: pixelated;
        float: left;
        object-position: top left;
        contain: size;
      }
      br {
        clear: both;
      }
      .big {
        contain-intrinsic-width: 32px;
        contain-intrinsic-height: 48px;
      }
      .small {
        contain-intrinsic-width: 8px;
        contain-intrinsic-height: 8px;
      }

      .cover { object-fit: cover }
      .contain { object-fit: contain }
      .fill { object-fit: fill }
      .none { object-fit: none }
      .scaledown { object-fit: scale-down }

    
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-height”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-width”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “contain-intrinsic-height”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
