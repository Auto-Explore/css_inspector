# css/selectors/focus-visible-script-focus-020.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-script-focus-020.html"
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
    outline: solid thick red;
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
