# css/css-values/urls/empty.html

```json
{
  "format_version": 3,
  "file": "css/css-values/urls/empty.html"
}
```

## style[0]

```css

#inline-unquoted {
    background-image: url();
    cursor: url(), pointer;
}

#inline-quoted {
    background-image: url("");
    cursor: url(""), pointer;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
