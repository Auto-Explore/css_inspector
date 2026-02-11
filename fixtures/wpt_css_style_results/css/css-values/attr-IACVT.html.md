# css/css-values/attr-IACVT.html

```json
{
  "format_version": 3,
  "file": "css/css-values/attr-IACVT.html"
}
```

## style[0]

```css

    #expected {
        background-color: red;
    }
    #test {
        background-color: red;
        background-color: attr(data-color type(<color>));
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
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
