# css/css-view-transitions/only-child-group.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/only-child-group.html"
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

::view-transition-group(root) {
  background-color: blue;
}
::view-transition-group(target) {
  background-color: blue;
}
::view-transition-group(*) {
  color: blue;
}

::view-transition-group(root):only-child {
  background-color: red;
}
::view-transition-group(target):only-child {
  background-color: red;
}
::view-transition-group(*):only-child {
  color: red;
}

```

```json
{
  "errors": 7,
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
    },
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
