# css/css-nesting/contextually-invalid-selectors-003.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/contextually-invalid-selectors-003.html"
}
```

## style[0]

```css

    div {
        color: green;
        background-color: currentColor;
        width: 100px;
        height: 100px;
    }

    p {
        color: initial;
    }

    div::before {
        & { /* `&` must not match the originating element */
            color: red;
        }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
