# css/selectors/focus-visible-016.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-016.html"
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

  div:focus-visible {
    background: red;
  }

  div:focus:not(:focus-visible) {
    background-color: lime;
  }

  input:focus-visible {
    background: lime;
  }

  input:focus:not(:focus-visible) {
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
