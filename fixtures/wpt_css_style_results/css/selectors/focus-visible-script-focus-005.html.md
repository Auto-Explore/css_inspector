# css/selectors/focus-visible-script-focus-005.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-script-focus-005.html"
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

  :focus-visible {
    outline: solid thick green;
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
