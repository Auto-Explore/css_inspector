# css/selectors/nth-last-child-specificity-4.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-last-child-specificity-4.html"
}
```

## style[0]

```css

    /* (1, 2, 1) */
    :nth-last-child(even of target.foo, .foo#bar, target.foo#bar, target#bar) {
        background-color: red;
        color: white;
        border: 5px solid purple;
    }

    /* (0, 1, 1) */
    target.foo {
        color: red;
    }
    /* (0, 2, 1) */
    target.foo.foo {
        border: 5px solid black;
    }

    /* (1, 1, 0) */
    .foo#bar {
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
