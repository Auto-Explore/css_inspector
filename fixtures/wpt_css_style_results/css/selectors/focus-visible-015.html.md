# css/selectors/focus-visible-015.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-015.html"
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
    background: red;
  }

  :focus:not(:focus-visible) {
    background-color: lime;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
