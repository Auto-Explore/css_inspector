# css/selectors/focus-visible-008.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-008.html"
}
```

## style[0]

```css

    @supports not selector(:focus-visible) {
      #el:focus {
        outline: red solid 5px;
        background-color: red;
      }
    }

    :focus-visible {
      outline: green solid 5px;
    }

    #el:focus:not(:focus-visible) {
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
