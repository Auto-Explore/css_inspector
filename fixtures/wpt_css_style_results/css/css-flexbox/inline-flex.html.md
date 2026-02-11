# css/css-flexbox/inline-flex.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/inline-flex.html"
}
```

## style[0]

```css

#testcase > div {
    height: 50px;
    width: 50px;
    background-color: green;
    outline: 2px solid darkgreen;
}
#testcase > div > div {
    flex: 1;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “outline”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
