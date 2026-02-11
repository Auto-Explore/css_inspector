# css/selectors/focus-visible-006.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-006.html"
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

    span[contenteditable] {
        border: 1px solid black;
        background-color: white;
        padding: 2px 5px;
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
