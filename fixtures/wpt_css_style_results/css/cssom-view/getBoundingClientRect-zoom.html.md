# css/cssom-view/getBoundingClientRect-zoom.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/getBoundingClientRect-zoom.html"
}
```

## style[0]

```css

      div {
        width: 64px;
        height: 64px;
        background-color: blue
      }
      div.x4_zoom {
        zoom: 4.0;
        background-color: blueviolet;
      }
      div.x2_zoom {
        background-color: chartreuse;
        zoom: 2.0;
      }

      div.transform {
        transform: scale(2);
        transform-origin: top left;
      }


    
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
