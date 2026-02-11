# css/css-view-transitions/only-child-image-pair.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/only-child-image-pair.html"
}
```

## style[0]

```css

::view-transition {
  background-color: black;
}
html:only-child {
  background-color: black;
}
:root:only-child {
  background-color: black;
}
:only-child {
  background-color: black;
}
.foo:only-child {
  background-color: black;
}

::view-transition-image-pair(root):only-child {
  background-color: red;
}
::view-transition-image-pair(*):only-child {
  color: red;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
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
