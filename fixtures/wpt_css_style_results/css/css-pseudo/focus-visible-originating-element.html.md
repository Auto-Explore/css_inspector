# css/css-pseudo/focus-visible-originating-element.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/focus-visible-originating-element.html"
}
```

## style[0]

```css

  input {
    outline: none;

    &:not(:disabled):focus-visible::before {
      background-color: green;
    }
  }

  input::before {
    content: "";
    background-color: red;
    display: inline-block;
    height: 50px;
    width: 50px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
