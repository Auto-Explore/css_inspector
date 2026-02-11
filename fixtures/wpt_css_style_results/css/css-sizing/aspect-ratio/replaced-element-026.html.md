# css/css-sizing/aspect-ratio/replaced-element-026.html

```json
{
  "format_version": 3,
  "file": "css/css-sizing/aspect-ratio/replaced-element-026.html"
}
```

## style[0]

```css

  div {
    display: inline-block;
    width: 60px;
    height: 100px;
    background: green;
  }
  video {
    height: 100px;
    aspect-ratio: 3/1;
    object-fit: contain;
    object-position: top left;
  }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
