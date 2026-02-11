# css/css-view-transitions/web-animations-api.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/web-animations-api.html"
}
```

## style[0]

```css

#first {
  background: blue;
  width: 100px;
  height: 100px;
  contain:  paint;
}
#second {
  background: green;
  width: 100px;
  height: 100px;
  contain: paint;
}

/* Unset all animations since the test drives it using WA-API */
html::view-transition-group(*),
html::view-transition-image-pair(*),
html::view-transition-new(*),
html::view-transition-old(*) {
  animation: unset;
}

html::view-transition-group(root){
  opacity: 0;
}

```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
