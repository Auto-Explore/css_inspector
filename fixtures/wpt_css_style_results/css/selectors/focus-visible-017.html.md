# css/selectors/focus-visible-017.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-017.html"
}
```

## style[0]

```css

  #warning {
    display: none;
    background: red;
  }

  @supports not selector(:focus-visible) {
    #instructions {
      display: none;
    }

    #warning {
      display: block;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
