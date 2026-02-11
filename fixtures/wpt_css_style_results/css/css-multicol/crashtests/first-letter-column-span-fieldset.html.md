# css/css-multicol/crashtests/first-letter-column-span-fieldset.html

```json
{
  "format_version": 3,
  "file": "css/css-multicol/crashtests/first-letter-column-span-fieldset.html"
}
```

## style[0]

```css

*:last-of-type {
  columns: 474px 1;
}
*:valid {
  column-span: all;
}
*::first-letter { }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “columns”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
