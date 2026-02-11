# css/css-view-transitions/navigation/prerender-removed-during-navigation.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/prerender-removed-during-navigation.html"
}
```

## style[0]

```css

@view-transition {
  navigation: auto;
}
html {
  background-color: red;
}
html.outgoing {
  background-color: cornflowerblue;
}
html.incoming {
  background-color: hotpink;
}
::view-transition {
  background-color: limegreen;
}
::view-transition-new(root) {
  transform: translateY(55vh);
  animation: none;
  opacity: 1;
}
::view-transition-old(root) {
  transform: translateY(-55vh);
  animation: none;
  opacity: 1;
}
::view-transition-group(root) {
  animation-duration: 300s;
}
```

```json
{
  "errors": 7,
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
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
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
