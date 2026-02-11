# css/selectors/focus-visible-001.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-001.html"
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
      outline: green solid 5px;
    }

    :focus:not(:focus-visible) {
      outline: 0;
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
