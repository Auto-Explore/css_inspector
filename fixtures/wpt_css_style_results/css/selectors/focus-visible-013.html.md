# css/selectors/focus-visible-013.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-013.html"
}
```

## style[0]

```css

  @supports not selector(:focus-visible) {
    :focus {
      outline: red solid 5px;
    }
  }

  #initial:focus-visible {
    outline: green solid 5px;
  }

  #initial:focus:not(:focus-visible) {
    outline: red solid 5px;
  }

  #target:focus-visible {
    outline: red solid 5px;
  }

  #target:focus:not(:focus-visible) {
    outline: green solid 5px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
