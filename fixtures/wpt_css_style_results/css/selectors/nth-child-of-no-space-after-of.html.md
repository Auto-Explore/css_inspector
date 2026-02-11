# css/selectors/nth-child-of-no-space-after-of.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-of-no-space-after-of.html"
}
```

## style[0]

```css

    target {
        color: red;
    }

    :nth-child(3 of/* my comment */target) {
        color: green;
    }

    .target {
        color: red;
    }

    :nth-child(3 of.target) {
        color: green;
    }

    [target] {
        color: red;
    }

    :nth-child(3 of[target]) {
        color: green;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
