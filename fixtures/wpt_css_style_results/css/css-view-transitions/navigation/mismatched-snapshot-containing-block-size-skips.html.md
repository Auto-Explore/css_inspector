# css/css-view-transitions/navigation/mismatched-snapshot-containing-block-size-skips.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/navigation/mismatched-snapshot-containing-block-size-skips.html"
}
```

## style[0]

```css

@view-transition {
  navigation: auto;
}
::view-transition-group(root) {
  animation-duration: 3s;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
