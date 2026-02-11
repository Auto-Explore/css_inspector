# css/css-view-transitions/only-child-view-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/only-child-view-transition.html"
}
```

## style[0]

```css

::view-transition {
  background-color: red;
}
::view-transition:only-child {
  background-color: blue;
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
