# css/selectors/focus-visible-014.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-014.html"
}
```

## style[0]

```css

  @supports not selector(:focus-visible) {
    :focus {
      outline: red solid 5px;
      background-color: red;
    }
  }

  :focus-visible {
    background: lime;
  }

  :focus:not(:focus-visible) {
    background-color: red;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
