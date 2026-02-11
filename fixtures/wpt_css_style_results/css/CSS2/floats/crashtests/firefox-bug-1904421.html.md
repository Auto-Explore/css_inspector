# css/CSS2/floats/crashtests/firefox-bug-1904421.html

```json
{
  "format_version": 3,
  "file": "css/CSS2/floats/crashtests/firefox-bug-1904421.html"
}
```

## style[0]

```css

.a {
  float: left;
}
:not(tfoot) {
  column-width: 0;
  border-top: 41px dashed -moz-activehyperlinktext;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
