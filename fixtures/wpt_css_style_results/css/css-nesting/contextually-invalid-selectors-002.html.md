# css/css-nesting/contextually-invalid-selectors-002.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/contextually-invalid-selectors-002.html"
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

    :is(*, ::before) * {
        color: purple;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
