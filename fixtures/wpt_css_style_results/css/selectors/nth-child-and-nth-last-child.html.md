# css/selectors/nth-child-and-nth-last-child.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-and-nth-last-child.html"
}
```

## style[0]

```css

    target1 {
        background-color: red;
    }

    :nth-child(1 of target1):nth-last-child(1 of target1) {
        background-color: green;
    }

    .target {
        background-color: red;
    }

    :nth-child(1 of .target):nth-last-child(1 of .target) {
        background-color: green;
    }

    [data-target] {
        background-color: red;
    }

    :nth-child(1 of [data-target]):nth-last-child(1 of [data-target]) {
        background-color: green;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
