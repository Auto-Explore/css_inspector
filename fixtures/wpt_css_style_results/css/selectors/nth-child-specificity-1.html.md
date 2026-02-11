# css/selectors/nth-child-specificity-1.html

```json
{
  "format_version": 3,
  "file": "css/selectors/nth-child-specificity-1.html"
}
```

## style[0]

```css

    /* The following 3 rules should all have the same specificity when matching <target>. They should be be applied in order. */
    foo:nth-child(n), bar:nth-child(n), target:nth-child(n) {
        background-color: red;
        color: red;
    }
    :nth-child(3n of foo, bar, target) {
        background-color: green;
        color: blue;
    }
    foo.target, bar.target, target.target {
        color: white;
    }
    * {
        background-color: white;
        color: black;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
