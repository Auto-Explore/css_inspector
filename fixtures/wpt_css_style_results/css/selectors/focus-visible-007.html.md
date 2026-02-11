# css/selectors/focus-visible-007.html

```json
{
  "format_version": 3,
  "file": "css/selectors/focus-visible-007.html"
}
```

## style[0]

```css

    [data-hadkeydown] :focus-visible {
      outline: green solid 5px;
    }

    [data-hadmousedown] :focus-visible {
      outline: red solid 5px;
    }

    [data-hadkeydown] :focus:not(:focus-visible) {
      outline: 0;
      background-color: red;
    }

    [data-hadmousedown] :focus:not(:focus-visible) {
      outline: 0;
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
