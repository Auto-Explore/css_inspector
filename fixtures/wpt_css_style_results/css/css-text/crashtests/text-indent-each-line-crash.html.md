# css/css-text/crashtests/text-indent-each-line-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-text/crashtests/text-indent-each-line-crash.html"
}
```

## style[0]

```css

*:scope {
  text-indent: 41% each-line;
  columns: 0;
}
#a {
  float: right;
  writing-mode: tb;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-indent”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
