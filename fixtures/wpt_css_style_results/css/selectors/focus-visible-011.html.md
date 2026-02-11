# css/selectors/focus-visible-011.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-011.html"
}
```

## style[0]

```css

    @supports not selector(:focus-visible) {
      #next:focus {
        outline: red solid 5px;
        background-color: red;
      }
    }

    button {
      border: 0;
    }

    :focus-visible {
      outline: green solid 5px;
    }

    :focus:not(:focus-visible) {
      background-color: red;
      outline: 0;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
