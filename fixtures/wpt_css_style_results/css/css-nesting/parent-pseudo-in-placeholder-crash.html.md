# css/css-nesting/parent-pseudo-in-placeholder-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/parent-pseudo-in-placeholder-crash.html"
}
```

## style[0]

```css

::placeholder { & {} }
::placeholder, .a { & {} }
.a, ::placeholder { & {} }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
