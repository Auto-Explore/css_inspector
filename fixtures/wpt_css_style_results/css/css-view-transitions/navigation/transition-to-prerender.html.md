# css/css-view-transitions/navigation/transition-to-prerender.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/transition-to-prerender.html"
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
```

```json
{
  "errors": 4,
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
    }
  ],
  "warnings": 0
}
```
