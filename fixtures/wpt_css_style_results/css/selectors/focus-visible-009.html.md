# css/selectors/focus-visible-009.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-009.html"
}
```

## style[0]

```css

    @supports not selector(:focus-visible) {
      #button:focus {
        outline: red solid 5px;
        background-color: red;
      }
    }

    :focus-visible {
      outline: green solid 5px;
    }

    #button:focus:not(:focus-visible) {
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
