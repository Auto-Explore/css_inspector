# css/css-values/urls/fragment-only.html

```json
{
  "format_version": 3,
  "file": "css/css-values/urls/fragment-only.html"
}
```

## style[0]

```css

#inline-unquoted {
    background-image: url(#foo);
    cursor: url(#foo), pointer;
}

#inline-quoted {
    background-image: url("#foo");
    cursor: url("#foo"), pointer;
}
```

```json
{
  "errors": 2,
  "messages": [
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
