# css/selectors/nth-child-specificity-2.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-specificity-2.html"
}
```

## style[0]

```css

    /* (1, 1, 0) */
    :nth-child(even of .foo, #bar, target) {
        background-color: red;
        color: white;
        border: 5px solid purple;
    }

    /* (0, 1, 1) */
    target.foo {
        color: red;
    }

    /* (0, 2, 0) */
    .foo.foo {
        background-color: green;
    }
    /* (1, 0, 1) */
    target#bar {
        border: 5px solid blue;
    }

    target {
        display: block;
        margin: 2px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
