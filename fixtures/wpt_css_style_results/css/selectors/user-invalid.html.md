# css/selectors/user-invalid.html

```json
{
  "format_version": 3,
  "file": "css/selectors/user-invalid.html"
}
```

## style[0]

```css

:is(input:not([type=submit], [type=reset]), textarea) {
  border: 2px solid black;
}

:is(input:not([type=submit], [type=reset]), textarea):user-valid {
  border-color: green;
}

:is(input:not([type=submit], [type=reset]), textarea):user-invalid {
  border-color: red;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Conflicting attribute selector constraints.",
      "severity": "Warning"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Conflicting attribute selector constraints.",
      "severity": "Warning"
    },
    {
      "message": "Conflicting attribute selector constraints.",
      "severity": "Warning"
    }
  ],
  "warnings": 3
}
```
