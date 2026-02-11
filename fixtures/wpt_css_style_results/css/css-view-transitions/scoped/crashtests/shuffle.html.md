# css/css-view-transitions/scoped/crashtests/shuffle.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/crashtests/shuffle.html"
}
```

## style[0]

```css


#s {
  position: relative;
  view-transition-name: wrapper;
  border: 5px solid lightgrey;
  height: 200px;
  width: 250px;
}
.i {
  position: relative;
  width: 150px; height: 60px;
  border: 5px solid #ace;
}
::view-transition-group(*),
::view-transition-old(*),
::view-transition-new(*) { animation: unset; }

```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
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
