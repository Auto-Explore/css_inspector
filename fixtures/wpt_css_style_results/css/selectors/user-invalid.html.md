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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
