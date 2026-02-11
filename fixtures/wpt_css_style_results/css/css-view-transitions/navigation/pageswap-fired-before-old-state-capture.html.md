# css/css-view-transitions/navigation/pageswap-fired-before-old-state-capture.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/pageswap-fired-before-old-state-capture.html"
}
```

## style[0]

```css

@view-transition {
  navigation: auto;
}

#target.oldPage {
  view-transition-name: target;
}
#target.newPage {
  view-transition-name: target;
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
