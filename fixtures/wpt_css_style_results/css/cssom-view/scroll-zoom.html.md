# css/cssom-view/scroll-zoom.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/scroll-zoom.html"
}
```

## style[0]

```css

      .container {
        height: 100px;
        width: 100px;
        overflow: scroll;
      }
      .content {
        height: 250px;
        width: 250px;
        background-image: linear-gradient(red, yellow);
      }
      #x4_zoom_container {
        zoom: 4;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
